use crate::errors::ModmailResult;
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub async fn defer_response(ctx: &Context, command: &CommandInteraction) -> ModmailResult<()> {
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
