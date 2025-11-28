use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::utils::*;
use chrono::Utc;
use serenity::FutureExt;
use serenity::all::{
    Channel, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, GuildId, PermissionOverwriteType, ResolvedOption, RoleId, UserId,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub struct CloseCommand;

#[async_trait::async_trait]
impl RegistrableCommand for CloseCommand {
    fn name(&self) -> &'static str {
        "close"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.close", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.close_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let time_before_close_desc = get_translated_message(
                &config,
                "slash_command.close_time_before_close_argument",
                None,
                None,
                None,
                None,
            )
            .await;
            let silent_desc = get_translated_message(
                &config,
                "slash_command.close_silent_argument",
                None,
                None,
                None,
                None,
            )
            .await;
            let cancel_desc = get_translated_message(
                &config,
                "slash_command.close_cancel_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("close")
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "time_before_close",
                            time_before_close_desc,
                        )
                        .required(false),
                    )
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::Boolean, "silent", silent_desc)
                            .required(false),
                    )
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::Boolean, "cancel", cancel_desc)
                            .required(false),
                    ),
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        _handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let db_pool = config
                .db_pool
                .as_ref()
                .ok_or_else(database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            let mut time_before_close: Option<String> = None;
            let mut silent: bool = false;
            let mut cancel: Option<bool> = None;
            let mut duration: Option<Duration> = None;

            for option in &command.data.options {
                match option.name.as_str() {
                    "time_before_close" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            time_before_close.replace(val.clone());
                        }
                    }
                    "silent" => {
                        if let CommandDataOptionValue::Boolean(val) = &option.value {
                            silent = *val;
                        }
                    }
                    "cancel" => {
                        if let CommandDataOptionValue::Boolean(val) = &option.value {
                            cancel.replace(*val);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(time_before_close) = time_before_close {
                if let Some(dur) = parse_duration_spec(&time_before_close) {
                    duration = Some(dur);
                }
            }

            let thread = fetch_thread(db_pool, &command.channel_id.to_string()).await?;
            let user_id = UserId::new(thread.user_id as u64);
            let community_guild_id = GuildId::new(config.bot.get_community_guild_id());

            if let Some(cancel) = cancel
                && cancel
            {
                let existed = delete_scheduled_closure(&thread.id, db_pool)
                    .await
                    .unwrap_or(false);
                return if existed {
                    let response = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "close.closure_canceled",
                            None,
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(command.channel_id)
                        .build_interaction_message_followup()
                        .await;

                    let _ = command.create_followup(&ctx.http, response).await;
                    Ok(())
                } else {
                    Err(ModmailError::Command(
                        CommandError::NoSchedulableClosureToCancel,
                    ))
                };
            }

            if duration.is_none() {
                if let Ok(Some(existing)) = get_scheduled_closure(&thread.id, db_pool).await {
                    let remaining = existing.close_at - Utc::now().timestamp();

                    let mut params = HashMap::new();
                    params.insert("seconds".to_string(), remaining.to_string());

                    if remaining > 0 {
                        return Err(ModmailError::Command(CommandError::ClosureAlreadyScheduled));
                    }
                }
            }

            if let Some(delay) = duration {
                if let Ok(Some(existing)) = get_scheduled_closure(&thread.id, db_pool).await {
                    let remaining = existing.close_at - Utc::now().timestamp();
                    if remaining > 0 {
                        let old_human = format_duration(remaining as u64);

                        let mut warn_params = HashMap::new();
                        warn_params.insert("old_time".to_string(), old_human);

                        let response = MessageBuilder::system_message(&ctx, &config)
                            .translated_content(
                                "close.replacing_existing_closure",
                                Some(&warn_params),
                                Some(command.user.id),
                                command.guild_id.map(|g| g.get()),
                            )
                            .await
                            .to_channel(command.channel_id)
                            .build_interaction_message_followup()
                            .await;

                        let _ = command.create_followup(&ctx.http, response).await;
                    }
                }

                let delay_secs = delay.as_secs();
                let human = format_duration(delay_secs);
                let mut params = HashMap::new();
                params.insert("time".to_string(), human);

                let _ = if silent {
                    let response = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "close.silent_closing",
                            Some(&params),
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(command.channel_id)
                        .build_interaction_message_followup()
                        .await;

                    command.create_followup(&ctx.http, response).await
                } else {
                    let response = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "close.closing",
                            Some(&params),
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(command.channel_id)
                        .build_interaction_message_followup()
                        .await;

                    command.create_followup(&ctx.http, response).await
                };

                let closed_by = command.user.id.to_string();

                let (category_id, category_name, required_permissions) =
                    match command.channel_id.to_channel(&ctx.http).await {
                        Ok(Channel::Guild(guild_channel)) => {
                            let guild_id = guild_channel.guild_id;
                            let parent_id = guild_channel.parent_id;

                            let category_id =
                                parent_id.map(|id| id.to_string()).unwrap_or_default();

                            let category_name = if let Some(parent_id) = parent_id {
                                guild_id
                                    .channels(&ctx.http)
                                    .await
                                    .ok()
                                    .and_then(|channels| {
                                        channels.get(&parent_id).map(|c| c.name.clone())
                                    })
                                    .unwrap_or_default()
                            } else {
                                String::new()
                            };

                            let guild = guild_id.to_partial_guild(&ctx.http).await.ok();
                            let everyone_role_id = RoleId::new(guild_id.get());

                            let mut perms = guild
                                .and_then(|g| {
                                    g.roles.get(&everyone_role_id).map(|r| r.permissions.bits())
                                })
                                .unwrap_or(0u64);

                            for overwrite in &guild_channel.permission_overwrites {
                                if let PermissionOverwriteType::Role(_) = overwrite.kind {
                                    let allow = overwrite.allow.bits();
                                    let deny = overwrite.deny.bits();
                                    perms = (perms & !deny) | allow;
                                }
                            }

                            (category_id, category_name, perms)
                        }
                        _ => (String::new(), String::new(), 0u64),
                    };

                let thread_id = thread.id.clone();
                let close_at = Utc::now().timestamp() + delay.as_secs() as i64;

                if let Err(e) = upsert_scheduled_closure(
                    &thread_id,
                    close_at,
                    silent,
                    &closed_by,
                    &category_id,
                    &category_name,
                    &required_permissions.to_string(),
                    db_pool,
                )
                .await
                {
                    eprintln!("Failed to persist scheduled closure: {e:?}");
                }

                schedule_one(&ctx, &config, thread_id, close_at);
                return Ok(());
            }

            let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

            let closed_by = command.user.id.to_string();
            let category_id = get_category_id_from_command(&ctx, &command).await;
            let category_name = get_category_name_from_command(&ctx, &command).await;
            let required_permissions =
                get_required_permissions_channel_from_command(&ctx, &command).await;

            if user_still_member && !silent {
                let _ = MessageBuilder::system_message(&ctx, &config)
                    .content(&config.bot.close_message)
                    .to_user(user_id)
                    .send(true)
                    .await;
            } else if !user_still_member {
                let mut params = HashMap::new();
                params.insert("username".to_string(), thread.user_name.clone());

                let response = MessageBuilder::system_message(&ctx, &config)
                    .translated_content(
                        "user.left_server_close",
                        Some(&params),
                        Some(command.user.id),
                        command.guild_id.map(|g| g.get()),
                    )
                    .await
                    .to_channel(command.channel_id)
                    .build_interaction_message_followup()
                    .await;

                let _ = command.create_followup(&ctx.http, response).await;
            }

            close_thread(
                &thread.id,
                &closed_by,
                &category_id,
                &category_name,
                required_permissions,
                db_pool,
            )
            .await?;
            let _ = delete_scheduled_closure(&thread.id, db_pool).await;

            if config.bot.enable_logs {
                if let Some(logs_channel_id) = config.bot.logs_channel_id {
                    let base_url = config
                        .bot
                        .redirect_url
                        .trim_end_matches("/api/auth/callback")
                        .trim_end_matches('/');

                    let panel_url = format!("{}/panel/tickets/{}", base_url, thread.id);

                    let mut params = HashMap::new();
                    params.insert("username".to_string(), thread.user_name.clone());
                    params.insert("user_id".to_string(), thread.user_id.to_string());
                    params.insert("panel_url".to_string(), panel_url);

                    let _ = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "logs.ticket_closed",
                            Some(&params),
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(serenity::all::ChannelId::new(logs_channel_id))
                        .send(true)
                        .await;
                }
            }

            let _ = command.channel_id.delete(&ctx.http).await?;

            Ok(())
        })
    }
}
