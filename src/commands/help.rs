use serenity::all::{Colour, Context, CreateEmbed, CreateMessage, Mentionable, Message};

use crate::config::Config;
use crate::errors::{ModmailResult, common};

pub async fn help(ctx: &Context, msg: &Message, _config: &Config) -> ModmailResult<()> {
    let channel = msg.channel_id.to_channel(&ctx).await
        .map_err(|_| common::channel_not_found())?;

    let embed = CreateEmbed::default()
        .title("Help Command Used")
        .description(format!(
            "**User** {} used the 'help' command in the {} channel",
            msg.author.name,
            channel.mention()
        ))
        .color(Colour::FOOYOO);

    let response = CreateMessage::new().embed(embed);

    msg.channel_id.send_message(&ctx.http, response).await
        .map_err(|_| common::validation_failed("Failed to send help message"))?;
    
    Ok(())
}
