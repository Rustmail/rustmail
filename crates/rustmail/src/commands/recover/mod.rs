use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use serenity::all::CreateCommand;
use std::collections::HashMap;

pub struct RecoverCommand;

impl Command for RecoverCommand {
    type Args = ();

    fn name(&self) -> &'static str {
        "recover"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.recover", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let description = get_translated_message(
            config,
            "slash_command.recover_command_description",
            None,
            None,
            None,
            None,
        )
        .await;

        vec![CreateCommand::new("recover").description(description)]
    }

    fn parse_text(&self, _args: &str) -> ModmailResult<()> {
        Ok(())
    }

    fn parse_slash(&self, _options: &SlashOptions) -> ModmailResult<()> {
        Ok(())
    }

    async fn run(&self, _args: (), invocation: &Invocation, config: &Config) -> ModmailResult<()> {
        config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;

        let started = HashMap::from([("user".to_string(), invocation.user().name.clone())]);
        invocation
            .reply_translated(config, "recovery.started", Some(&started))
            .await?;

        let results = recover_missing_messages(invocation.ctx(), config).await;
        let recovered: u32 = results.iter().map(|r| r.recovered_count).sum();
        let succeeded = results.iter().filter(|r| r.success).count();

        let summary = HashMap::from([
            ("total".to_string(), recovered.to_string()),
            ("threads".to_string(), succeeded.to_string()),
            (
                "failed".to_string(),
                (results.len() - succeeded).to_string(),
            ),
        ]);
        invocation
            .reply_translated(config, "recovery.summary", Some(&summary))
            .await?;
        Ok(())
    }
}
