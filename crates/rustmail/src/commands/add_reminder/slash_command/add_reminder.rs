use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use chrono::{Local, NaiveTime, TimeZone};
use regex::Regex;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, GuildId, ResolvedOption, RoleId,
};
use std::sync::Arc;

pub struct AddReminderCommand;

#[async_trait::async_trait]
impl RegistrableCommand for AddReminderCommand {
    fn name(&self) -> &'static str {
        "remind"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(config, "help.add_reminder", None, None, None, None).await
        }.boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
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
            let roles_desc = get_translated_message(
                &config,
                "slash_command.add_reminder_roles_argument_description",
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
                    )
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::String, "roles", roles_desc)
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
        handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = config
                .db_pool
                .as_ref()
                .ok_or_else(database_connection_failed)?;

            let _ = defer_response(&ctx, &command).await;

            let mut time: Option<String> = None;
            let mut content: Option<String> = None;
            let mut roles: Option<String> = None;

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
                    "roles" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            roles.replace(val.clone());
                        }
                    }
                    _ => {}
                }
            }

            let time = match time {
                Some(t) => t.clone(),
                None => {
                    return Err(ModmailError::Command(CommandError::InvalidReminderFormat));
                }
            };

            let content = match content {
                Some(c) => c,
                None => {
                    return Err(ModmailError::Command(CommandError::InvalidReminderFormat));
                }
            };

            let time_str = time.to_string();
            let re = Regex::new(r"^(?P<hour>[01]?\d|2[0-3]):(?P<minute>[0-5]\d)$").unwrap();
            let captures = re
                .captures(&time_str)
                .ok_or_else(|| ModmailError::Command(CommandError::InvalidReminderFormat))?;

            let hours: u32 = captures
                .name("hour")
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0);

            let minutes: u32 = captures
                .name("minute")
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0);

            let time = NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();
            let now = Local::now().with_timezone(&config.bot.timezone);

            let mut trigger_dt = config
                .bot
                .timezone
                .from_local_datetime(&now.date_naive().and_time(time))
                .unwrap();

            if trigger_dt < now {
                trigger_dt += chrono::Duration::days(1);
            }

            let trigger_timestamp = trigger_dt.with_timezone(&config.bot.timezone).timestamp();

            let thread = match get_thread_by_user_id(command.user.id, pool).await {
                Some(t) => t,
                None => {
                    return Err(ModmailError::Thread(ThreadError::ThreadNotFound));
                }
            };

            let target_roles = if let Some(roles_str) = roles {
                resolve_role_names_to_ids(&ctx, config.bot.get_staff_guild_id(), &roles_str).await
            } else {
                None
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
                target_roles,
            };

            let reminder_id = match insert_reminder(&reminder, pool).await {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Failed to insert reminder: {}", e);
                    return Err(e);
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

            spawn_reminder(
                &reminder,
                Some(reminder_id),
                &ctx,
                &config,
                &pool,
                handler.shutdown.clone(),
            );

            Ok(())
        })
    }
}

async fn resolve_role_names_to_ids(
    ctx: &Context,
    guild_id: u64,
    roles_str: &str,
) -> Option<String> {
    if roles_str.is_empty() {
        return None;
    }

    let guild_id_obj = GuildId::new(guild_id);
    let guild = match guild_id_obj.to_partial_guild(&ctx.http).await {
        Ok(g) => g,
        Err(_) => return None,
    };

    let mention_regex = Regex::new(r"<@&(\d+)>").unwrap();

    let role_parts: Vec<&str> = roles_str.split(',').map(|s| s.trim()).collect();
    let mut role_ids: Vec<u64> = Vec::new();

    for role_part in role_parts {
        if role_part.is_empty() {
            continue;
        }

        if let Some(caps) = mention_regex.captures(role_part) {
            if let Some(id_match) = caps.get(1) {
                if let Ok(id) = id_match.as_str().parse::<u64>() {
                    if guild.roles.contains_key(&RoleId::new(id)) {
                        role_ids.push(id);
                    }
                }
            }
        } else {
            let role_name_lower = role_part.to_lowercase();
            if let Some(role) = guild
                .roles
                .values()
                .find(|r| r.name.to_lowercase() == role_name_lower)
            {
                role_ids.push(role.id.get());
            }
        }
    }

    if role_ids.is_empty() {
        None
    } else {
        Some(
            role_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}
