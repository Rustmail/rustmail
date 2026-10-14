use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

/// Whether the answer to a slash command is ephemeral: outside the staff guild (community guild in
/// dual mode) only the invoking staff member sees the bot's answer (and any error).
pub fn responds_ephemerally(command: &CommandInteraction, config: &Config) -> bool {
    command
        .guild_id
        .is_some_and(|id| id.get() != config.bot.get_staff_guild_id())
}

/// Defers a slash command, ephemerally when `responds_ephemerally` says so.
pub async fn defer_response(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
) -> ModmailResult<()> {
    if responds_ephemerally(command, config) {
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
