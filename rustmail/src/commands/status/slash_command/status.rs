use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::errors::ModmailResult;
use crate::handlers::InteractionHandler;
use crate::i18n::get_translated_message;
use futures::FutureExt;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedOption,
};
use std::sync::Arc;

pub struct StatusCommand;

impl RegistrableCommand for StatusCommand {
    fn name(&self) -> &'static str {
        "status"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(
                config,
                "slash_command.status_command_help",
                None,
                None,
                None,
                None,
            )
            .await
        }
        .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.status_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let mode_desc = get_translated_message(
                &config,
                "slash_command.mode_arg_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let online_mode = get_translated_message(
                &config,
                "slash_command.online_status_mode",
                None,
                None,
                None,
                None,
            )
            .await;
            let idle_mode = get_translated_message(
                &config,
                "slash_command.idle_status_mode",
                None,
                None,
                None,
                None,
            )
            .await;
            let dnd_mode = get_translated_message(
                &config,
                "slash_command.do_not_disturb_status_mode",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("status")
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::String, "mode", mode_desc)
                            .add_string_choice(online_mode, "online")
                            .add_string_choice(idle_mode, "idle")
                            .add_string_choice(dnd_mode, "dnd")
                            .add_string_choice("Invisible", "invisible")
                            .add_string_choice("Maintenance", "maintenance")
                            .required(true),
                    ),
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        options: &[ResolvedOption<'_>],
        config: &Config,
        handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        todo!()
    }
}
