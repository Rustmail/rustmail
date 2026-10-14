use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

/// Defers a slash command. Outside the staff guild (community guild in dual mode) the response is
/// ephemeral, so only the invoking staff member sees the bot's answer (and any error).
pub async fn defer_response(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
) -> ModmailResult<()> {
    let outside_staff_guild = command
        .guild_id
        .is_some_and(|id| id.get() != config.bot.get_staff_guild_id());
    if outside_staff_guild {
        return defer_response_ephemeral(ctx, command).await;
    }

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
        )
        .await?;
    Ok(())
}

pub async fn defer_response_ephemeral(
    ctx: &Context,
    command: &CommandInteraction,
) -> ModmailResult<()> {
    command.defer_ephemeral(&ctx.http).await?;
    Ok(())
}
