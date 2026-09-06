use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{CommandInteraction, CommandType, Context, CreateCommand, ResolvedOption};
use std::sync::Arc;

pub struct VersionCommand;

#[async_trait::async_trait]
impl RegistrableCommand for VersionCommand {
    fn as_community(&self) -> Option<&dyn CommunityRegistrable> {
        None
    }

    fn name(&self) -> &'static str {
        "version"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.version", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.version_command_desc",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(self.name()).description(cmd_desc),
                CreateCommand::new(self.name()).kind(CommandType::User),
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        _handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            defer_response(&ctx, &command).await?;

            let content = build_version_content(&config).await;

            let _ = MessageBuilder::system_message(&ctx, &config)
                .content(content)
                .to_channel(command.channel_id)
                .send_interaction_followup(&command, false)
                .await;

            Ok(())
        })
    }
}
