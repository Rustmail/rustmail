use serenity::all::{Context, Message};
use crate::config::Config;
use crate::db::operations::threads::{is_a_ticket_channel, is_orphaned_thread_channel};
use crate::errors::{common, ModmailError, ModmailResult};
use crate::errors::DatabaseError::QueryFailed;
use crate::errors::DiscordError::ApiError;
use crate::errors::ThreadError::{NotAThreadChannel, UserStillInServer};

pub async fn force_close(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    if !is_a_ticket_channel(msg.channel_id, db_pool).await {
        return Err(ModmailError::Thread(NotAThreadChannel));
    }

    match is_orphaned_thread_channel(msg.channel_id, db_pool).await {
        Ok(res) => {
            if !res {
                return Err(ModmailError::Thread(UserStillInServer));
            }
            match msg.channel_id.delete(ctx).await {
                Ok(_) => {
                    println!("Thread force closed successfully: {}", msg.channel_id);
                    Ok(())
                }
                Err(e) => Err(ModmailError::Discord(ApiError(e.to_string()))),
            }
        }
        Err(..) => {
            Err(ModmailError::Database(QueryFailed("Failed to check if thread channel is orphaned".to_string())))
        }
    }
}