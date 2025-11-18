use crate::bot::ShardManagerKey;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::utils::MessageBuilder;
use serenity::all::{Context, Message};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;

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
        .cloned()
        .ok_or(ModmailError::Discord(DiscordError::ShardManagerNotFound))?;

    let time_before = Instant::now();
    let mut res = msg.reply(&ctx.http, "...").await?;
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

    let edited_msg = MessageBuilder::system_message(&ctx, &config)
        .translated_content("slash_command.ping_command", Some(&params), None, None)
        .await
        .to_channel(msg.channel_id)
        .build_edit_message()
        .await;

    res.edit(&ctx.http, edited_msg).await?;

    Ok(())
}
