use crate::bot::ShardManagerKey;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::CreateCommand;
use std::collections::HashMap;
use tokio::time::Instant;

pub struct PingCommand;

impl Command for PingCommand {
    type Args = ();

    fn name(&self) -> &'static str {
        "ping"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.ping", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let description = get_translated_message(
            config,
            "slash_command.ping_command_desc",
            None,
            None,
            None,
            None,
        )
        .await;

        vec![CreateCommand::new("ping").description(description)]
    }

    fn parse_text(&self, _args: &str) -> ModmailResult<()> {
        Ok(())
    }

    fn parse_slash(&self, _options: &SlashOptions) -> ModmailResult<()> {
        Ok(())
    }

    async fn run(&self, _args: (), invocation: &Invocation, config: &Config) -> ModmailResult<()> {
        let ctx = invocation.ctx();
        let shard_manager = ctx
            .data
            .read()
            .await
            .get::<ShardManagerKey>()
            .cloned()
            .ok_or(ModmailError::Discord(DiscordError::ShardManagerNotFound))?;

        let sent_at = Instant::now();
        let mut reply = invocation
            .reply_unrecorded(MessageBuilder::system_message(ctx, config).content("..."))
            .await?;
        let message_latency = sent_at.elapsed();

        let gateway_latency = {
            let runners = shard_manager.runners.lock().await;
            runners.get(&ctx.shard_id).and_then(|runner| runner.latency)
        };

        let api_start = Instant::now();
        ctx.http.get_gateway().await?;
        let api_latency = api_start.elapsed();

        let mut params = HashMap::new();
        params.insert(
            "gateway_latency".to_string(),
            gateway_latency.unwrap_or_default().as_millis().to_string(),
        );
        params.insert(
            "api_latency".to_string(),
            api_latency.as_millis().to_string(),
        );
        params.insert(
            "message_latency".to_string(),
            message_latency.as_millis().to_string(),
        );

        let content = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "slash_command.ping_command",
                Some(&params),
                Some(invocation.user_id()),
                invocation.guild_id().map(|g| g.get()),
            )
            .await;
        invocation.edit_reply(&mut reply, content).await
    }
}
