use crate::errors::DiscordError::ApiError;
use crate::errors::{ModmailError, ModmailResult};
use serenity::all::{ChannelId, Context};

pub async fn delete_channel(ctx: &Context, channel_id: ChannelId) -> ModmailResult<()> {
    match channel_id.delete(ctx).await {
        Ok(_) => {
            println!("Channel {} deleted successfully", channel_id);
            Ok(())
        }
        Err(e) => Err(ModmailError::Discord(ApiError(e.to_string()))),
    }
}
