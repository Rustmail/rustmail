use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::utils::*;
use serenity::all::{CommandType, CreateCommand};
use std::collections::HashMap;

pub struct VersionCommand;

impl Command for VersionCommand {
    type Args = ();

    fn name(&self) -> &'static str {
        "version"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["v"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.version", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let description = get_translated_message(
            config,
            "slash_command.version_command_desc",
            None,
            None,
            None,
            None,
        )
        .await;

        vec![
            CreateCommand::new("version").description(description),
            CreateCommand::new("version").kind(CommandType::User),
        ]
    }

    fn parse_text(&self, _args: &str) -> ModmailResult<()> {
        Ok(())
    }

    fn parse_slash(&self, _options: &SlashOptions) -> ModmailResult<()> {
        Ok(())
    }

    async fn run(&self, _args: (), invocation: &Invocation, config: &Config) -> ModmailResult<()> {
        let mut params = HashMap::new();
        params.insert("version".to_string(), CURRENT_VERSION.to_string());
        params.insert("repository".to_string(), REPOSITORY_URL.to_string());

        let content = get_translated_message(
            config,
            "version_command.current",
            Some(&params),
            None,
            None,
            None,
        )
        .await;

        invocation
            .reply_unrecorded(
                MessageBuilder::system_message(invocation.ctx(), config).content(content),
            )
            .await?;
        Ok(())
    }
}
