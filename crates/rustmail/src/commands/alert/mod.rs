pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use std::collections::HashMap;

pub struct AlertCommand;

impl Command for AlertCommand {
    type Args = AlertAction;

    fn name(&self) -> &'static str {
        "alert"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.alert", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("alert")
                .description(translate("slash_command.alert_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "cancel",
                        translate("slash_command.alert_cancel_argument").await,
                    )
                    .required(false),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<AlertAction> {
        parse_alert_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<AlertAction> {
        parse_alert_slash(options)
    }

    async fn run(
        &self,
        action: AlertAction,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let ticket = current_ticket(invocation, pool).await?;
        let staff = invocation.user_id();

        let key = match action {
            AlertAction::Set => {
                set_alert_for_staff(staff, ticket.user_id, pool)
                    .await
                    .map_err(|_| ModmailError::Command(CommandError::AlertSetFailed))?;
                "alert.confirmation"
            }
            AlertAction::Cancel => {
                cancel_alert_for_staff(staff, ticket.user_id, pool).await?;
                "alert.cancel_confirmation"
            }
        };

        let params = HashMap::from([("user".to_string(), format!("<@{}>", ticket.user_id))]);
        invocation
            .reply_translated(config, key, Some(&params))
            .await?;
        Ok(())
    }
}
