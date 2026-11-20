pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use std::collections::HashMap;

pub struct RenameCommand;

impl Command for RenameCommand {
    type Args = RenameArgs;

    fn name(&self) -> &'static str {
        "rename"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["rn"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.rename", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("rename")
                .description(translate("slash_command.rename_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "label",
                        translate("slash_command.rename_label_option").await,
                    )
                    .required(false),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<RenameArgs> {
        parse_rename_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<RenameArgs> {
        parse_rename_slash(options)
    }

    async fn run(
        &self,
        args: RenameArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;

        let ticket = current_ticket(invocation, pool).await?;
        let mut status = ticket_status(&ticket, pool).await?;
        status.label = args.label.clone();
        let rename = save_ticket_status(invocation, &ticket, &status, pool).await?;

        let key = match (args.label.is_some(), rename) {
            (true, ChannelRename::Applied) => "rename.confirmation",
            (true, ChannelRename::Delayed) => "rename.confirmation_rate_limited",
            (false, ChannelRename::Applied) => "rename.cleared",
            (false, ChannelRename::Delayed) => "rename.cleared_rate_limited",
        };
        let params = args
            .label
            .map(|label| HashMap::from([("label".to_string(), label)]));
        invocation
            .reply_translated(config, key, params.as_ref())
            .await?;
        Ok(())
    }
}
