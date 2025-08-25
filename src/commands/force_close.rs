use crate::config::Config;
use crate::db::operations::threads::{is_a_ticket_channel, is_orphaned_thread_channel};
use crate::errors::DatabaseError::QueryFailed;
use crate::errors::DiscordError::ApiError;
use crate::errors::ThreadError::{NotAThreadChannel, UserStillInServer};
use crate::errors::{ModmailError, ModmailResult, common};
use serenity::all::{ChannelId, Context, Message};

async fn delete_channel(ctx: &Context, channel_id: ChannelId) -> ModmailResult<()> {
    match channel_id.delete(ctx).await {
        Ok(_) => {
            println!("Channel {} deleted successfully", channel_id);
            Ok(())
        }
        Err(e) => Err(ModmailError::Discord(ApiError(e.to_string()))),
    }
}

pub async fn force_close(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    if !is_a_ticket_channel(msg.channel_id, db_pool).await {
        return match msg.category_id(&ctx.http).await {
            Some(category_id) if category_id == config.thread.inbox_category_id => {
                delete_channel(ctx, msg.channel_id).await
            }
            _ => Err(ModmailError::Thread(NotAThreadChannel)),
        };
    }

    match is_orphaned_thread_channel(msg.channel_id, db_pool).await {
        Ok(res) => {
            if !res {
                return Err(ModmailError::Thread(UserStillInServer));
            }
            delete_channel(ctx, msg.channel_id).await
        }
        Err(..) => Err(ModmailError::Database(QueryFailed(
            "Failed to check if thread channel is orphaned".to_string(),
        ))),
    }
}
