use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn version(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let content = build_version_content(config).await;

    let _ = MessageBuilder::system_message(&ctx, config)
        .content(content)
        .to_channel(msg.channel_id)
        .send(false)
        .await;

    Ok(())
}
