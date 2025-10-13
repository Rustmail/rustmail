use crate::commands::add_reminder::common::{
    send_register_confirmation_from_command, spawn_reminder,
};
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::reminders::{Reminder, insert_reminder};
use crate::db::threads::get_thread_by_user_id;
use crate::errors::{
    CommandError, DatabaseError, ModmailError, ModmailResult, ThreadError, common,
};
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use chrono::{Local, NaiveTime};
use regex::Regex;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub struct AddReminderCommand;

#[async_trait::async_trait]
impl RegistrableCommand for AddReminderCommand {
    fn name(&self) -> &'static str {
        "add_reminder"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
        let config = config.clone();
        let name = self.name();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.add_reminder_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let time_desc = get_translated_message(
                &config,
                "slash_command.add_reminder_time_argument_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let content_desc = get_translated_message(
                &config,
                "slash_command.add_reminder_content_argument_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(name)
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::String, "time", time_desc)
                            .required(true),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "content",
                            content_desc,
                        )
                        .required(true),
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
        shutdown: Arc<Receiver<bool>>,
    ) -> BoxFuture<ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = config
                .db_pool
                .as_ref()
                .ok_or_else(common::database_connection_failed)?;

            let _ = defer_response(&ctx, &command).await;

            let mut time: Option<String> = None;
            let mut content: Option<String> = None;

            for option in &command.data.options {
                match option.name.as_str() {
                    "time" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            time.replace(val.clone());
                        }
                    }
                    "content" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            content.replace(val.clone());
                        }
                    }
                    _ => {}
                }
            }

            let time = match time {
                Some(t) => t.clone(),
                None => {
                    return Err(ModmailError::Command(CommandError::InvalidArguments(
                        "Missing required arguments".to_string(),
                    )));
                }
            };

            let content = match content {
                Some(c) => c,
                None => {
                    return Err(ModmailError::Command(CommandError::InvalidArguments(
                        "Missing required arguments".to_string(),
                    )));
                }
            };

            let time_str = time.to_string();
            let re = Regex::new(r"^(?P<hour>[01]?\d|2[0-3]):(?P<minute>[0-5]\d)$").unwrap();
            let captures = re.captures(&time_str).ok_or_else(|| {
                return ModmailError::Command(CommandError::InvalidArguments(
                    "duration".to_string(),
                ));
            })?;

            let hours: u32 = captures
                .name("hour")
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0);

            let minutes: u32 = captures
                .name("minute")
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0);

            let time = NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();
            let now = Local::now();
            let mut trigger_dt = now.date_naive().and_time(time);

            if trigger_dt < now.date_naive().and_time(time) {
                trigger_dt += chrono::Duration::days(1);
            }

            let trigger_timestamp = trigger_dt.and_local_timezone(Local).unwrap().timestamp();

            let thread = match get_thread_by_user_id(command.user.id, pool).await {
                Some(t) => t,
                None => {
                    return Err(ModmailError::Thread(ThreadError::ThreadNotFound));
                }
            };

            let reminder: Reminder = Reminder {
                thread_id: thread.id,
                user_id: command.user.id.get() as i64,
                channel_id: command.channel_id.get() as i64,
                guild_id: config.bot.get_staff_guild_id() as i64,
                reminder_content: content.clone(),
                trigger_time: trigger_timestamp,
                created_at: now.timestamp(),
                completed: false,
            };

            let reminder_id = match insert_reminder(&reminder, pool).await {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Failed to insert reminder: {}", e);
                    return Err(ModmailError::Database(DatabaseError::InsertFailed(
                        e.to_string(),
                    )));
                }
            };

            send_register_confirmation_from_command(
                reminder_id,
                &content,
                &ctx,
                &command,
                &config,
                trigger_timestamp,
            )
            .await;

            spawn_reminder(&reminder, Some(reminder_id), &ctx, &config, &pool, shutdown);

            Ok(())
        })
    }
}
