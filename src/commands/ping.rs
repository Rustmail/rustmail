use serenity::all::{Colour, Context, CreateEmbed, CreateMessage, Message, Timestamp};

use crate::config::Config;
use crate::errors::{ModmailResult, common};

pub async fn ping(ctx: &Context, msg: &Message, _config: &Config) -> ModmailResult<()> {
    let _channel = msg.channel_id.to_channel(&ctx).await
        .map_err(|_| common::channel_not_found())?;
    let embed = CreateEmbed::default()
        .title("Ping Command Used")
        .description("Pong !".to_string())
        .color(Colour::RED)
        .timestamp(Timestamp::now());

    let response = CreateMessage::new().embed(embed);

    msg.channel_id.send_message(&ctx.http, response).await
        .map_err(|_| common::validation_failed("Failed to send ping response"))?;
    
    Ok(())
}
