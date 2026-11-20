pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub struct HelpCommand {
    pub catalog: CommandCatalog,
}

impl Command for HelpCommand {
    type Args = HelpArgs;

    fn name(&self) -> &'static str {
        "help"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.help", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("help")
                .description(translate("slash_command.help_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "command",
                        translate("slash_command.help_command_argument_desc").await,
                    )
                    .required(false),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<HelpArgs> {
        parse_help_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<HelpArgs> {
        parse_help_slash(options)
    }

    async fn run(
        &self,
        args: HelpArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let content = match args.command {
            Some(name) => format!("**{name}**\n\n{}", self.command_doc(&name, config).await?),
            None => self.command_list(config).await,
        };
        invocation
            .reply(MessageBuilder::system_message(invocation.ctx(), config).content(content))
            .await?;
        Ok(())
    }
}

impl HelpCommand {
    async fn command_list(&self, config: &Config) -> String {
        let mut names = self.catalog.names();
        names.push("help");
        names.sort_unstable();

        let mut list = get_translated_message(config, "help.message", None, None, None, None).await;
        for name in names {
            list.push_str(&format!("- **{name}**\n"));
        }
        list
    }

    async fn command_doc(&self, name: &str, config: &Config) -> ModmailResult<String> {
        if name == "help" {
            return Ok(Command::doc(self, config).await);
        }
        self.catalog
            .doc(name, config)
            .await
            .ok_or_else(|| ModmailError::Command(CommandError::UnknownCommand(name.to_string())))
    }
}
