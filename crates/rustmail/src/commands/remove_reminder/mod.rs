pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use std::collections::HashMap;

pub struct RemoveReminderCommand;

impl Command for RemoveReminderCommand {
    type Args = RemoveReminderArgs;

    fn name(&self) -> &'static str {
        "unremind"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["urem"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.remove_reminder", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("unremind")
                .description(translate("slash_command.remove_reminder_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Number,
                        "id",
                        translate("slash_command.remove_reminder_id_argument").await,
                    )
                    .required(true),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<RemoveReminderArgs> {
        parse_remove_reminder_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<RemoveReminderArgs> {
        parse_remove_reminder_slash(options)
    }

    async fn run(
        &self,
        args: RemoveReminderArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;

        let reminder = get_reminder_by_id(args.id, pool)
            .await
            .map_err(|e| ModmailError::Database(DatabaseError::QueryFailed(e.to_string())))?
            .ok_or_else(|| ModmailError::Database(DatabaseError::NotFound(String::new())))?;
        if reminder.data.completed {
            return Err(ModmailError::Command(
                CommandError::ReminderAlreadyCompleted(args.id.to_string()),
            ));
        }

        update_reminder_status(&reminder, true, pool)
            .await
            .map_err(|e| ModmailError::Database(DatabaseError::QueryFailed(e.to_string())))?;

        let mut params = HashMap::new();
        params.insert("id".to_string(), args.id.to_string());
        let _ = invocation
            .reply_translated(config, "remove_reminder.confirmation", Some(&params))
            .await;
        Ok(())
    }
}
