use crate::bot::ShardManagerKey;
use crate::commands::{BoxFuture, CommunityRegistrable, RegistrableCommand};
use crate::config::Config;
use crate::errors::ModmailResult;
use crate::handlers::InteractionHandler;
use crate::i18n::get_translated_message;
use serenity::FutureExt;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::sync::Arc;

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

            println!("Latency for shard {}: {:?}", ctx.shard_id, latency);

            Ok(())
        })
    }
}
