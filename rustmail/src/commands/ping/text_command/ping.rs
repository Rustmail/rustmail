use crate::bot::ShardManagerKey;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use serenity::all::{Context, Message};
use std::sync::Arc;

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

    println!("Latency for shard {}: {:?}", ctx.shard_id, latency);

    Ok(())
}
