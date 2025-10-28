use crate::commands::help::common::{display_command_help, display_commands_list};
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::errors::ModmailResult;
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use serenity::FutureExt;
use std::sync::Arc;

pub struct HelpCommand;

#[async_trait::async_trait]
impl RegistrableCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.help", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.help_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let cmd_arg_desc = get_translated_message(
                &config,
                "slash_command.help_command_argument_desc",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("help").description(cmd_desc).add_option(
                    CreateCommandOption::new(CommandOptionType::String, "command", cmd_arg_desc)
                        .required(false),
                ),
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            defer_response(&ctx, &command).await?;

            let mut command_name: Option<String> = None;

            for option in &command.data.options {
                match option.name.as_str() {
                    "command" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            command_name.replace(val.clone());
                        }
                    }
                    _ => {}
                }
            }

            if let Some(cmd_name) = command_name {
                display_command_help(
                    &ctx,
                    &config,
                    handler.registry.clone(),
                    None,
                    Some(&command),
                    &cmd_name,
                )
                .await?;
            } else {
                display_commands_list(
                    &ctx,
                    &config,
                    handler.registry.clone(),
                    None,
                    Some(&command),
                )
                .await?;
            }

            Ok(())
        })
    }
}
