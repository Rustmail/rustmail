use crate::prelude::errors::*;
use serenity::all::{ChannelId, Context};

pub async fn delete_channel(ctx: &Context, channel_id: ChannelId) -> ModmailResult<()> {
    match channel_id.delete(ctx).await {
        Ok(_) => {
            println!("Channel {} deleted successfully", channel_id);
            Ok(())
        }
        Err(e) => Err(ModmailError::Discord(DiscordError::ApiError(e.to_string()))),
    }
}
