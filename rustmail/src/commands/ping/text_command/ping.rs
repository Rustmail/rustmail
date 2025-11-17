use std::collections::HashMap;
use crate::bot::ShardManagerKey;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use serenity::all::{Context, Message};
use std::sync::Arc;
use std::time::Duration;
use crate::utils::MessageBuilder;

pub async fn ping(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
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

    let _ = MessageBuilder::system_message(&ctx, &config)
        .translated_content("slash_command.ping_command", Some(&params), None, None).await
        .to_channel(msg.channel_id)
        .send(true).await
        .map_err(|e| ModmailError::Discord(DiscordError::ApiError(e.to_string())))?;

    Ok(())
}
