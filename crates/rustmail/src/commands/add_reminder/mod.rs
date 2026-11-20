pub mod args;
pub mod schedule;

pub use args::*;
pub use schedule::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use chrono::Local;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, GuildId, RoleId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub struct AddReminderCommand {
    pub shutdown: Arc<Receiver<bool>>,
}

impl Command for AddReminderCommand {
    type Args = RemindArgs;

    fn name(&self) -> &'static str {
        "remind"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["rem"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.add_reminder", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;
        let option = async |kind, name, key, required| {
            CreateCommandOption::new(kind, name, translate(key).await).required(required)
        };

        vec![
            CreateCommand::new("remind")
                .description(translate("slash_command.add_reminder_command_description").await)
                .add_option(
                    option(
                        CommandOptionType::String,
                        "time",
                        "slash_command.add_reminder_time_argument_description",
                        true,
                    )
                    .await,
                )
                .add_option(
                    option(
                        CommandOptionType::String,
                        "content",
                        "slash_command.add_reminder_content_argument_description",
                        true,
                    )
                    .await,
                )
                .add_option(
                    option(
                        CommandOptionType::String,
                        "roles",
                        "slash_command.add_reminder_roles_argument_description",
                        false,
                    )
                    .await,
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<RemindArgs> {
        parse_remind_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<RemindArgs> {
        parse_remind_slash(options)
    }

    async fn run(
        &self,
        args: RemindArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        match args {
            RemindArgs::Subscription(args) => {
                change_reminder_subscription(args, invocation, config).await
            }
            RemindArgs::Add(reminder) => self.add(reminder, invocation, config).await,
        }
    }
}

impl AddReminderCommand {
    async fn add(
        &self,
        reminder: NewReminder,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let ctx = invocation.ctx();
        let ticket = current_ticket(invocation, pool).await?;

        let now = Local::now().with_timezone(&config.bot.timezone);
        let trigger_time = next_occurrence(reminder.time, &now)
            .ok_or(ModmailError::Command(CommandError::InvalidReminderFormat))?
            .timestamp();

        let staff_guild = GuildId::new(config.bot.get_staff_guild_id());
        let (roles, content) = match reminder.content {
            ReminderContent::LeadingRoles(text) if text.is_empty() => (Vec::new(), text),
            ReminderContent::LeadingRoles(text) => {
                let guild_roles = guild_roles(ctx, staff_guild).await?;
                split_leading_roles(&text, |query| query.find(&guild_roles))
            }
            ReminderContent::Separate {
                content,
                roles: Some(roles),
            } => {
                let guild_roles = guild_roles(ctx, staff_guild).await?;
                (
                    parse_role_list(&roles, |query| query.find(&guild_roles)),
                    content,
                )
            }
            ReminderContent::Separate {
                content,
                roles: None,
            } => (Vec::new(), content),
        };
        let target_roles = (!roles.is_empty()).then(|| {
            roles
                .iter()
                .map(RoleId::to_string)
                .collect::<Vec<_>>()
                .join(",")
        });

        let saved = insert_reminder(
            &ReminderData {
                thread_id: ticket.id,
                user_id: invocation.user_id().get() as i64,
                channel_id: invocation.channel_id().get() as i64,
                guild_id: staff_guild.get() as i64,
                reminder_content: content.clone(),
                trigger_time,
                created_at: now.timestamp(),
                completed: false,
                target_roles,
            },
            pool,
        )
        .await?;

        let mut params = HashMap::from([
            ("time".to_string(), format!("<t:{trigger_time}:t>")),
            (
                "remaining_time".to_string(),
                format!("<t:{trigger_time}:R>"),
            ),
        ]);
        if !content.is_empty() {
            params.insert("content".to_string(), content.clone());
        }
        if !roles.is_empty() {
            let mentions: Vec<String> = roles.iter().map(|id| format!("<@&{id}>")).collect();
            params.insert("roles".to_string(), mentions.join(", "));
        }
        let key = match (!roles.is_empty(), !content.is_empty()) {
            (true, true) => "reminder.registered_with_content_roles",
            (true, false) => "reminder.registered_without_content_roles",
            (false, true) => "reminder.registered_with_content",
            (false, false) => "reminder.registered_without_content",
        };
        let confirmation = MessageBuilder::system_message(ctx, config)
            .translated_content(
                key,
                Some(&params),
                Some(invocation.user_id()),
                invocation.guild_id().map(|g| g.get()),
            )
            .await
            .footer(format!("ID: {}", saved.id));
        invocation.reply(confirmation).await?;

        if let Some(message) = invocation.message() {
            let _ = message.delete(&ctx.http).await;
        }

        spawn_reminder(
            &saved,
            Some(saved.id),
            ctx,
            config,
            pool,
            self.shutdown.clone(),
        );
        Ok(())
    }
}
