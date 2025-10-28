use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::errors::ModmailResult;
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::FutureExt;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use serenity::futures::future::join_all;
use std::collections::HashMap;
use std::error::Error;
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

            vec![CreateCommand::new("help").description(cmd_desc)]
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
            let mut docs_message = String::new();

            let welcome_msg =
                get_translated_message(&config, "help.message", None, None, None, None).await;
            docs_message.push_str(&welcome_msg);

            let futures = handler.registry.commands.iter().map(|(name, command)| {
                let config = config.clone();

                async move {
                    let doc = command.doc(&config).await;
                    format!("**{}** — {}\n\n", name, doc)
                }
            });

            let results = join_all(futures).await;
            docs_message.push_str(&results.join(""));

            defer_response(&ctx, &command).await?;

            let response = MessageBuilder::system_message(&ctx, &config)
                .content(docs_message)
                .to_channel(command.channel_id)
                .build_interaction_message_followup()
                .await;

            let _ = command.create_followup(&ctx.http, response).await;

            Ok(())
        })
    }
}
