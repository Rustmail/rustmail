use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub fn responds_ephemerally(command: &CommandInteraction, config: &Config) -> bool {
    command
        .guild_id
        .is_some_and(|id| id.get() != config.bot.get_staff_guild_id())
}

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
