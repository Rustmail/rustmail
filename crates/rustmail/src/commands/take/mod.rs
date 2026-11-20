use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::CreateCommand;
use std::collections::HashMap;

pub struct TakeCommand;

impl Command for TakeCommand {
    type Args = ();

    fn name(&self) -> &'static str {
        "take"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.take", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let description = get_translated_message(
            config,
            "slash_command.take_command_description",
            None,
            None,
            None,
            None,
        )
        .await;

        vec![CreateCommand::new("take").description(description)]
    }

    fn parse_text(&self, _args: &str) -> ModmailResult<()> {
        Ok(())
    }

    fn parse_slash(&self, _options: &SlashOptions) -> ModmailResult<()> {
        Ok(())
    }

    async fn run(&self, _args: (), invocation: &Invocation, config: &Config) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;

        let ticket = current_ticket(invocation, pool).await?;
        let mut status = ticket_status(&ticket, pool).await?;

        let staff_id = invocation.user_id().to_string();
        if status.taken_by.as_deref() == Some(staff_id.as_str()) {
            return Err(ModmailError::Command(CommandError::TicketAlreadyTaken));
        }
        status.taken_by = Some(staff_id);

        let key = match save_ticket_status(invocation, &ticket, &status, pool).await? {
            ChannelRename::Applied => "take.confirmation",
            ChannelRename::Delayed => "take.confirmation_rate_limited",
        };

        let mut params = HashMap::new();
        params.insert("staff".to_string(), format!("<@{}>", invocation.user_id()));
        invocation
            .reply_translated(config, key, Some(&params))
            .await?;
        Ok(())
    }
}
