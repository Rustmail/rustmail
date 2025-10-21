use crate::config::Config;
use crate::db::operations::{create_thread_for_user, get_thread_channel_by_user_id};
use crate::errors::common::validation_failed;
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use crate::utils::message::ui;
use crate::utils::thread::get_thread_lock::get_thread_lock;
use crate::utils::thread::send_to_thread::send_to_thread;
use crate::utils::time::format_duration_since::format_duration_since;
use crate::utils::time::get_member_join_date::get_member_join_date_for_user;
use serenity::all::{
    ActionRowComponent, Channel, ChannelId, ComponentInteraction, Context, CreateChannel, GuildId,
    Message, ModalInteraction, UserId,
};
use serenity::builder::{CreateInteractionResponse, EditChannel, EditMessage};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

async fn create_or_get_thread_for_user(
    ctx: &Context,
    config: &Config,
    user_id: UserId,
) -> Result<(ChannelId, bool), Box<dyn std::error::Error + Send + Sync>> {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return Err("Database pool not available".into());
        }
    };

    let username = match user_id.to_user(&ctx.http).await {
        Ok(u) => u.name.clone(),
        Err(_) => user_id.get().to_string(),
    };

    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());
    let channel_builder =
        CreateChannel::new(&username).category(ChannelId::new(config.thread.inbox_category_id));

    let channel = staff_guild_id
        .create_channel(&ctx.http, channel_builder)
        .await?;

    let _ = create_thread_for_user(&channel, user_id.get() as i64, &username, pool)
        .await
        .map_err(|e| {
            eprintln!("Error creating thread: {}", e);
            e
        })?;

    let canonical_channel_id_str = get_thread_channel_by_user_id(user_id, pool).await;
    let (target_channel_id, is_new_thread) =
        if let Some(canonical_id_str) = canonical_channel_id_str {
            let canonical_matches = canonical_id_str == channel.id.to_string();
            if !canonical_matches {
                let _ = channel.delete(&ctx.http).await;
                (
                    ChannelId::new(canonical_id_str.parse::<u64>().unwrap_or(channel.id.get())),
                    false,
                )
            } else {
                (channel.id, true)
            }
        } else {
            (channel.id, true)
        };

    if is_new_thread {
        let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
        let member_join_date = get_member_join_date_for_user(ctx, user_id, community_guild_id)
            .await
            .unwrap_or_else(|| "Unknown".to_string());

        let open_thread_message = format!(
            "ACCOUNT AGE **{}**, ID **{}**\nNICKNAME **{}**, JOINED **{}**",
            format_duration_since(user_id.created_at()),
            user_id,
            username,
            member_join_date
        );

        let _ = MessageBuilder::system_message(ctx, config)
            .to_channel(target_channel_id)
            .content(open_thread_message)
            .send(false)
            .await;

        let _ = MessageBuilder::system_message(ctx, config)
            .content(&config.bot.welcome_message)
            .to_user(user_id)
            .send(false)
            .await;

        println!("Thread created successfully");
    }

    Ok((target_channel_id, is_new_thread))
}

pub async fn create_channel(ctx: &Context, msg: &Message, config: &Config) {
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    if (community_guild_id.member(&ctx.http, msg.author.id).await).is_err() {
        let error_msg = get_translated_message(
            config,
            "server.not_in_community",
            None,
            Some(msg.author.id),
            Some(community_guild_id.get()),
            None,
        )
        .await;

        let error = validation_failed(&error_msg);
        if let Some(error_handler) = &config.error_handler {
            let _ = error_handler
                .reply_to_msg_with_error(ctx, msg, &error)
                .await;
        }
        return;
    }

    let (target_channel_id, _is_new_thread) =
        match create_or_get_thread_for_user(ctx, config, msg.author.id).await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Failed to create or get thread: {}", e);
                return;
            }
        };

    if let Err(e) = send_to_thread(ctx, target_channel_id, msg, config, false).await {
        eprintln!("Failed to forward message to thread: {:?}", e);
    }
}

fn parse_thread_interaction(custom_id: &str) -> Option<String> {
    let parts: Vec<&str> = custom_id.split(':').collect();
    if parts.len() >= 2 && (parts[0] == "ticket" || parts[0] == "thread") {
        Some(parts[1].to_string())
    } else {
        None
    }
}

pub async fn handle_thread_modal_interaction(
    ctx: &Context,
    config: &Config,
    interaction: &mut ModalInteraction,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let parts = match parse_thread_interaction(&interaction.data.custom_id) {
        Some(parts) => parts,
        None => {
            let response = CreateInteractionResponse::Message(
                MessageBuilder::system_message(&ctx, &config)
                    .translated_content(
                        "feature.not_implemented",
                        None,
                        Some(interaction.user.id),
                        interaction.guild_id.map(|g| g.get()),
                    )
                    .await
                    .to_channel(interaction.channel_id)
                    .build_interaction_message()
                    .await
                    .ephemeral(true),
            );
            let _ = interaction.create_response(&ctx.http, response).await;
            return Ok(());
        }
    };

    let key = interaction.channel_id.get();
    let lock = get_thread_lock(config, key);

    let guard = match lock.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        MessageBuilder::system_message(ctx, config)
                            .translated_content("thread.action_in_progress", None, None, None)
                            .await
                            .to_channel(interaction.channel_id)
                            .build_interaction_message()
                            .await
                            .ephemeral(true),
                    ),
                )
                .await;
            return Ok(());
        }
    };

    match parts.as_str() {
        "create" => {
            let user_id_str = interaction
                .data
                .components
                .iter()
                .flat_map(|row| row.components.iter())
                .find_map(|comp| match comp {
                    ActionRowComponent::InputText(input)
                        if input.custom_id == "thread:create:user_id" =>
                    {
                        input.value.as_deref().map(|s| s.trim().to_string())
                    }
                    _ => None,
                });

            let user_id = match user_id_str {
                Some(id) => match UserId::from_str(id.as_str()) {
                    Ok(user_id) => user_id,
                    Err(_) => {
                        eprintln!("Invalid user ID provided in modal interaction: {}", id);
                        let _ = interaction
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    MessageBuilder::system_message(ctx, config)
                                        .translated_content(
                                            "thread.modal_invalid_user_id",
                                            None,
                                            None,
                                            None,
                                        )
                                        .await
                                        .to_channel(interaction.channel_id)
                                        .build_interaction_message()
                                        .await
                                        .ephemeral(true),
                                ),
                            )
                            .await;
                        return Ok(());
                    }
                },
                None => {
                    return Ok(());
                }
            };

            let user = match user_id.to_user(&ctx.http).await {
                Ok(user) => user,
                Err(_) => {
                    eprintln!("Failed to fetch user by ID: {}", user_id);
                    let _ = interaction
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                MessageBuilder::system_message(ctx, config)
                                    .translated_content(
                                        "thread.modal_user_not_found",
                                        None,
                                        None,
                                        None,
                                    )
                                    .await
                                    .to_channel(interaction.channel_id)
                                    .build_interaction_message()
                                    .await
                                    .ephemeral(true),
                            ),
                        )
                        .await;
                    return Ok(());
                }
            };

            if user.bot {
                eprintln!(
                    "Attempted to create thread for a rustmail user: {}",
                    user_id
                );
                let _ = interaction
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            MessageBuilder::system_message(ctx, config)
                                .translated_content("thread.modal_bot_user", None, None, None)
                                .await
                                .to_channel(interaction.channel_id)
                                .build_interaction_message()
                                .await
                                .ephemeral(true),
                        ),
                    )
                    .await;
                return Ok(());
            }

            let pool = match &config.db_pool {
                Some(p) => p,
                None => {
                    drop(guard);
                    return Ok(());
                }
            };

            if let Some(existing_channel_str) = get_thread_channel_by_user_id(user_id, pool).await {
                let mut params = HashMap::new();
                params.insert(
                    "channel".to_string(),
                    format!("<#{}>", existing_channel_str),
                );
                let _ = interaction
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            MessageBuilder::system_message(ctx, config)
                                .translated_content(
                                    "thread.already_exists",
                                    Some(&params),
                                    None,
                                    None,
                                )
                                .await
                                .to_channel(interaction.channel_id)
                                .build_interaction_message()
                                .await
                                .ephemeral(true),
                        ),
                    )
                    .await;
                drop(guard);
                return Ok(());
            }

            let mut guild_channel = match interaction.channel_id.to_channel(&ctx.http).await {
                Ok(Channel::Guild(gc)) => gc,
                _ => {
                    let _ = interaction
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                MessageBuilder::system_message(ctx, config)
                                    .translated_content(
                                        "thread.not_a_thread_channel",
                                        None,
                                        None,
                                        None,
                                    )
                                    .await
                                    .to_channel(interaction.channel_id)
                                    .build_interaction_message()
                                    .await
                                    .ephemeral(true),
                            ),
                        )
                        .await;
                    drop(guard);
                    return Ok(());
                }
            };

            if let Some(parent_id) = guild_channel.parent_id {
                if parent_id.get() != config.thread.inbox_category_id {
                    let _ = interaction
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                MessageBuilder::system_message(ctx, config)
                                    .translated_content(
                                        "thread.not_a_thread_channel",
                                        None,
                                        None,
                                        None,
                                    )
                                    .await
                                    .to_channel(interaction.channel_id)
                                    .build_interaction_message()
                                    .await
                                    .ephemeral(true),
                            ),
                        )
                        .await;
                    drop(guard);
                    return Ok(());
                }
            } else {
                let _ = interaction
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            MessageBuilder::system_message(ctx, config)
                                .translated_content("thread.not_a_thread_channel", None, None, None)
                                .await
                                .to_channel(interaction.channel_id)
                                .build_interaction_message()
                                .await
                                .ephemeral(true),
                        ),
                    )
                    .await;
                drop(guard);
                return Ok(());
            }

            let _ = guild_channel.edit(&ctx.http, EditChannel::new()).await;

            if let Err(e) =
                create_thread_for_user(&guild_channel, user_id.get() as i64, &user.name, pool).await
            {
                eprintln!("Failed to create thread record: {}", e);
                let _ = interaction
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            MessageBuilder::system_message(ctx, config)
                                .translated_content("thread.creation_failed", None, None, None)
                                .await
                                .to_channel(interaction.channel_id)
                                .build_interaction_message()
                                .await
                                .ephemeral(true),
                        ),
                    )
                    .await;
                drop(guard);
                return Ok(());
            }

            let mut params = HashMap::new();
            params.insert("user".to_string(), user.name.clone());

            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content(
                    "new_thread.welcome_message",
                    Some(&params),
                    None,
                    Some(guild_channel.guild_id.get()),
                )
                .await
                .to_channel(interaction.channel_id)
                .send(false)
                .await;

            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content("new_thread.dm_notification", None, Some(user_id), None)
                .await
                .to_user(user_id)
                .send(false)
                .await;

            let mut params = HashMap::new();
            params.insert(
                "channel".to_string(),
                format!("<#{}>", interaction.channel_id),
            );
            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        MessageBuilder::system_message(ctx, config)
                            .translated_content("thread.created", Some(&params), None, None)
                            .await
                            .to_channel(interaction.channel_id)
                            .build_interaction_message()
                            .await
                            .ephemeral(true),
                    ),
                )
                .await;

            match &interaction.message {
                Some(message) => {
                    let _ = message.delete(&ctx.http).await;
                }
                None => {
                    eprintln!("Failed to delete interaction message!");
                }
            }
        }
        _ => {
            eprintln!("Unknown thread modal interaction action: {}", parts);
            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        MessageBuilder::system_message(ctx, config)
                            .translated_content("thread.unknown_action", None, None, None)
                            .await
                            .to_channel(interaction.channel_id)
                            .build_interaction_message()
                            .await,
                    ),
                )
                .await;
        }
    }

    drop(guard);
    Ok(())
}

pub async fn handle_thread_component_interaction(
    ctx: &Context,
    config: &Config,
    interaction: &mut ComponentInteraction,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let parts = match parse_thread_interaction(&interaction.data.custom_id) {
        Some(parts) => parts,
        None => {
            let response = CreateInteractionResponse::Message(
                MessageBuilder::system_message(&ctx, &config)
                    .translated_content(
                        "feature.not_implemented",
                        None,
                        Some(interaction.user.id),
                        interaction.guild_id.map(|g| g.get()),
                    )
                    .await
                    .to_channel(interaction.channel_id)
                    .build_interaction_message()
                    .await
                    .ephemeral(true),
            );
            let _ = interaction.create_response(&ctx.http, response).await;
            return Ok(());
        }
    };

    let key = interaction.channel_id.get();
    let lock = get_thread_lock(config, key);

    let guard = match lock.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        MessageBuilder::system_message(ctx, config)
                            .translated_content("thread.action_in_progress", None, None, None)
                            .await
                            .to_channel(interaction.channel_id)
                            .build_interaction_message()
                            .await
                            .ephemeral(true),
                    ),
                )
                .await;
            return Ok(());
        }
    };

    match parts.as_str() {
        "delete" => {
            let mut params = HashMap::new();
            params.insert(
                "seconds".to_string(),
                format!("{}", config.thread.time_to_close_thread),
            );
            params.insert("user".to_string(), format!("<@{}>", interaction.user.id));

            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        MessageBuilder::system_message(ctx, config)
                            .translated_content("thread.thread_closing", Some(&params), None, None)
                            .await
                            .to_channel(interaction.channel_id)
                            .build_interaction_message()
                            .await,
                    ),
                )
                .await;

            sleep(Duration::from_secs(config.thread.time_to_close_thread)).await;

            interaction.channel_id.delete(&ctx.http).await?;
        }
        "keep" => {
            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        MessageBuilder::system_message(ctx, config)
                            .translated_content("thread.will_remain_open", None, None, None)
                            .await
                            .to_channel(interaction.channel_id)
                            .build_interaction_message()
                            .await,
                    ),
                )
                .await;

            let builder = EditMessage::default().components(vec![]);

            interaction.message.edit(&ctx.http, builder).await?;
        }
        "dont_create" => {
            interaction.message.delete(&ctx.http).await?;
        }
        "wants_to_create" => {
            let modal = ui::modal(
                "thread:create",
                get_translated_message(
                    config,
                    "thread.modal_to_create_ticket",
                    None,
                    None,
                    None,
                    None,
                )
                .await,
            )
            .short_input(
                "thread:create:user_id",
                "User ID",
                Some("1234567890123456789"),
                true,
            )
            .build();

            let _ = interaction
                .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
                .await;
        }
        _ => {
            eprintln!("Unknown thread component interaction action: {}", parts);
            let _ = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        MessageBuilder::system_message(ctx, config)
                            .translated_content("thread.unknown_action", None, None, None)
                            .await
                            .to_channel(interaction.channel_id)
                            .build_interaction_message()
                            .await,
                    ),
                )
                .await;
        }
    }

    drop(guard);
    Ok(())
}
