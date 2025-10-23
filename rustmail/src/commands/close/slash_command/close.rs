use crate::commands::close::common::parse_duration_spec;
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::{
    close_thread, delete_scheduled_closure, get_scheduled_closure, upsert_scheduled_closure,
};
use crate::errors::{common, CommandError, ModmailError, ModmailResult};
use crate::i18n::get_translated_message;
use crate::types::logs::PaginationStore;
use crate::utils::command::category::{
    get_category_id_from_command, get_category_name_from_command,
    get_required_permissions_channel_from_command,
};
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use crate::utils::thread::fetch_thread::fetch_thread;
use chrono::Utc;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, GuildId, ResolvedOption, UserId,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time::sleep;

pub struct CloseCommand;

#[async_trait::async_trait]
impl RegistrableCommand for CloseCommand {
    fn name(&self) -> &'static str {
        "close"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
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
        _shutdown: Arc<Receiver<bool>>,
        _pagination: PaginationStore,
    ) -> BoxFuture<ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let db_pool = config
                .db_pool
                .as_ref()
                .ok_or_else(common::database_connection_failed)?;

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
                let delay_secs = delay.as_secs();
                let human = if delay_secs < 60 {
                    format!("{}s", delay_secs)
                } else if delay_secs < 3600 {
                    format!("{}m", delay_secs / 60)
                } else if delay_secs < 86400 {
                    format!("{}h{}m", delay_secs / 3600, (delay_secs % 3600) / 60)
                } else {
                    format!("{}d{}h", delay_secs / 86400, (delay_secs % 86400) / 3600)
                };
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

                    let _ = command.create_followup(&ctx.http, response).await;
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

                    let _ = command.create_followup(&ctx.http, response).await;
                };

                let thread_id = thread.id.clone();
                let close_at = Utc::now().timestamp() + delay.as_secs() as i64;

                let closed_by = command.user.id.to_string();
                let category_id = get_category_id_from_command(&ctx, &command).await;
                let category_name = get_category_name_from_command(&ctx, &command).await;
                let required_permissions =
                    get_required_permissions_channel_from_command(&ctx, &command).await;

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
                let channel_id = command.channel_id;
                let config_clone = config.clone();
                let ctx_clone = ctx.clone();
                let user_id_clone = user_id;
                let thread_id_for_task = thread_id.clone();

                tokio::spawn(async move {
                    sleep(delay).await;
                    if let Some(pool) = config_clone.db_pool.as_ref() {
                        if let Ok(Some(record)) =
                            get_scheduled_closure(&thread_id_for_task, pool).await
                        {
                            if record.close_at <= Utc::now().timestamp() {
                                let _ = close_thread(
                                    &thread_id_for_task,
                                    &record.closed_by,
                                    &record.category_id,
                                    &record.category_name,
                                    record.required_permissions.parse::<u64>().unwrap_or(0),
                                    pool,
                                )
                                .await;
                                let _ = delete_scheduled_closure(&thread_id_for_task, pool).await;

                                let community_guild_id =
                                    GuildId::new(config_clone.bot.get_community_guild_id());

                                let user_still_member = community_guild_id
                                    .member(&ctx_clone.http, user_id_clone)
                                    .await
                                    .is_ok();

                                if !record.silent && user_still_member {
                                    let _ =
                                        MessageBuilder::system_message(&ctx_clone, &config_clone)
                                            .content(&config_clone.bot.close_message)
                                            .to_user(user_id_clone)
                                            .send(false)
                                            .await;
                                }
                                let _ = channel_id.delete(&ctx_clone.http).await;
                            } else {
                                let delay2 =
                                    (record.close_at - Utc::now().timestamp()).max(1) as u64;
                                let config_clone2 = config_clone.clone();
                                let ctx_clone2 = ctx_clone.clone();
                                let thread_id_again = thread_id_for_task.clone();

                                tokio::spawn(async move {
                                    sleep(Duration::from_secs(delay2)).await;
                                    if let Some(pool2) = config_clone2.db_pool.as_ref() {
                                        if let Ok(Some(r2)) =
                                            get_scheduled_closure(&thread_id_again, pool2).await
                                        {
                                            if r2.close_at <= Utc::now().timestamp() {
                                                let _ = close_thread(
                                                    &thread_id_again,
                                                    &r2.closed_by,
                                                    &r2.category_id,
                                                    &r2.category_name,
                                                    r2.required_permissions
                                                        .parse::<u64>()
                                                        .unwrap_or(0),
                                                    pool2,
                                                )
                                                .await;
                                                let _ = delete_scheduled_closure(
                                                    &thread_id_again,
                                                    pool2,
                                                )
                                                .await;
                                                let community_guild_id = GuildId::new(
                                                    config_clone2.bot.get_community_guild_id(),
                                                );
                                                let user_still_member = community_guild_id
                                                    .member(&ctx_clone2.http, user_id_clone)
                                                    .await
                                                    .is_ok();
                                                if !r2.silent && user_still_member {
                                                    let _ = MessageBuilder::system_message(
                                                        &ctx_clone2,
                                                        &config_clone2,
                                                    )
                                                    .content(&config_clone2.bot.close_message)
                                                    .to_user(user_id_clone)
                                                    .send(false)
                                                    .await;
                                                }
                                                let _ = channel_id.delete(&ctx_clone2.http).await;
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                });
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
                    .send(false)
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

            let _ = command.channel_id.delete(&ctx.http).await?;

            Ok(())
        })
    }
}
