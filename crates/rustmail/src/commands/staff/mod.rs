pub mod access;
pub mod args;

pub use access::*;
pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::{CommandOptionType, CommandType, CreateCommand, CreateCommandOption};

pub struct AddStaffCommand;

impl Command for AddStaffCommand {
    type Args = StaffTarget;

    fn name(&self) -> &'static str {
        "addmod"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["am"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.add_staff", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        vec![
            target_command(
                config,
                "addmod",
                "slash_command.add_staff_command_description",
                "slash_command.add_staff_target_argument",
            )
            .await,
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<StaffTarget> {
        parse_staff_target_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<StaffTarget> {
        parse_staff_target_slash(options)
    }

    async fn run(
        &self,
        target: StaffTarget,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        change_ticket_access(Access::Grant, target, invocation, config).await
    }
}

pub struct RemoveStaffCommand;

impl Command for RemoveStaffCommand {
    type Args = StaffTarget;

    fn name(&self) -> &'static str {
        "delmod"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["dm"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.remove_staff", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        vec![
            target_command(
                config,
                "delmod",
                "slash_command.remove_staff_command_description",
                "slash_command.remove_staff_target_argument",
            )
            .await,
            CreateCommand::new("delmod").kind(CommandType::User),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<StaffTarget> {
        parse_staff_target_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<StaffTarget> {
        parse_staff_target_slash(options)
    }

    async fn run(
        &self,
        target: StaffTarget,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        change_ticket_access(Access::Revoke, target, invocation, config).await
    }
}

async fn target_command(
    config: &Config,
    name: &str,
    description_key: &str,
    target_key: &str,
) -> CreateCommand {
    let translate = async |key| get_translated_message(config, key, None, None, None, None).await;

    CreateCommand::new(name)
        .description(translate(description_key).await)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Mentionable,
                "target",
                translate(target_key).await,
            )
            .required(true),
        )
}
