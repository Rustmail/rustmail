use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::CreateCommand;
use std::collections::HashMap;

pub struct IdCommand;

impl Command for IdCommand {
    type Args = ();

    fn name(&self) -> &'static str {
        "id"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.id", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let description = get_translated_message(
            config,
            "slash_command.id_command_description",
            None,
            None,
            None,
            None,
        )
        .await;

        vec![CreateCommand::new("id").description(description)]
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

        let thread = current_ticket(invocation, pool).await?;

        let mut params = HashMap::new();
        params.insert("user".to_string(), format!("<@{}>", thread.user_id));
        params.insert("id".to_string(), format!("||{}||", thread.user_id));
        invocation
            .reply_translated(config, "id.show_id", Some(&params))
            .await?;
        Ok(())
    }
}
