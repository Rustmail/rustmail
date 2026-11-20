use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::CreateCommand;
use std::collections::HashMap;

pub struct ReleaseCommand;

impl Command for ReleaseCommand {
    type Args = ();

    fn name(&self) -> &'static str {
        "release"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.release", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let description = get_translated_message(
            config,
            "slash_command.release_command_description",
            None,
            None,
            None,
            None,
        )
        .await;

        vec![CreateCommand::new("release").description(description)]
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

        if status.taken_by.take().is_none() {
            return Err(ModmailError::Command(CommandError::TicketAlreadyReleased));
        }

        let key = match save_ticket_status(invocation, &ticket, &status, pool).await? {
            ChannelRename::Applied => "release.confirmation",
            ChannelRename::Delayed => "release.confirmation_rate_limited",
        };

        let mut params = HashMap::new();
        params.insert("staff".to_string(), format!("<@{}>", invocation.user_id()));
        invocation
            .reply_translated(config, key, Some(&params))
            .await?;
        Ok(())
    }
}
