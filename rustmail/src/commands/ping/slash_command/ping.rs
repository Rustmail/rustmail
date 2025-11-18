use crate::bot::ShardManagerKey;
use crate::commands::{BoxFuture, CommunityRegistrable, RegistrableCommand};
use crate::config::Config;
use crate::errors::{DiscordError, ModmailError, ModmailResult};
use crate::handlers::InteractionHandler;
use crate::i18n::get_translated_message;
use crate::utils::{MessageBuilder, defer_response};
use serenity::FutureExt;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;

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
            let response = MessageBuilder::system_message(&ctx, &config)
                .content("...")
                .to_channel(command.channel_id)
                .build_interaction_message_followup()
                .await;

            let shard_manager = ctx
                .data
                .read()
                .await
                .get::<ShardManagerKey>()
                .cloned()
                .ok_or(ModmailError::Discord(DiscordError::ShardManagerNotFound))?;

            let time_before = Instant::now();
            let mut res = command.create_followup(&ctx.http, response).await?;
            let msg_send_ping = time_before.elapsed().as_millis();

            let gateway_ping = {
                let runners = shard_manager.runners.lock().await;
                runners.get(&ctx.shard_id).and_then(|runner| runner.latency)
            };

            let start = Instant::now();
            ctx.http.get_gateway().await?;
            let api_ping = start.elapsed();

            let mut params = HashMap::new();
            params.insert(
                "gateway_latency".to_string(),
                format!(
                    "{:?}",
                    gateway_ping.unwrap_or(Duration::default()).as_millis()
                ),
            );
            params.insert(
                "api_latency".to_string(),
                format!("{:?}", api_ping.as_millis()),
            );
            params.insert(
                "message_latency".to_string(),
                format!("{:?}", msg_send_ping),
            );

            let response = MessageBuilder::system_message(&ctx, &config)
                .translated_content("slash_command.ping_command", Some(&params), None, None)
                .await
                .to_channel(command.channel_id)
                .build_edit_message()
                .await;

            res.edit(&ctx.http, response).await?;

            Ok(())
        })
    }
}
