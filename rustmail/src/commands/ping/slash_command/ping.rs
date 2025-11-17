use std::collections::HashMap;
use crate::bot::ShardManagerKey;
use crate::commands::{BoxFuture, CommunityRegistrable, RegistrableCommand};
use crate::config::Config;
use crate::errors::{DiscordError, ModmailError, ModmailResult};
use crate::handlers::InteractionHandler;
use crate::i18n::get_translated_message;
use serenity::FutureExt;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::sync::Arc;
use std::time::Duration;
use crate::utils::{defer_response, MessageBuilder};

pub struct PingCommand;

#[async_trait::async_trait]
impl RegistrableCommand for PingCommand {
    fn as_community(&self) -> Option<&dyn CommunityRegistrable> {
        None
    }

    fn name(&self) -> &'static str {
        "ping"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.ping", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.ping_command_desc",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new(self.name()).description(cmd_desc)]
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

            let shard_manager = ctx
                .data
                .read()
                .await
                .get::<ShardManagerKey>()
                .unwrap()
                .clone();

            let latency = {
                let runners = shard_manager.runners.lock().await;
                runners.get(&ctx.shard_id).and_then(|runner| runner.latency)
            };

            let mut params = HashMap::new();
            params.insert("latency".to_string(), format!("{:?}", latency.unwrap_or(Duration::default()).as_millis()));

            let response = MessageBuilder::system_message(&ctx, &config)
                .translated_content("slash_command.ping_command", Some(&params), None, None).await
                .to_channel(command.channel_id)
                .build_interaction_message_followup().await;

            command.create_followup(&ctx.http, response).await
                .map_err(|e| ModmailError::Discord(DiscordError::ApiError(e.to_string())))?;

            Ok(())
        })
    }
}
