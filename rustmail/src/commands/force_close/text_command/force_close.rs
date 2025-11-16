use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn force_close(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if !is_a_ticket_channel(msg.channel_id, db_pool).await {
        return match msg.category_id(&ctx.http).await {
            Some(category_id) if category_id == config.thread.inbox_category_id => {
                delete_channel(&ctx, msg.channel_id).await
            }
            _ => Err(ModmailError::Thread(ThreadError::NotAThreadChannel)),
        };
    }

    match is_orphaned_thread_channel(msg.channel_id, db_pool).await {
        Ok(res) => {
            if !res {
                return Err(ModmailError::Thread(ThreadError::UserStillInServer));
            }
            delete_channel(&ctx, msg.channel_id).await
        }
        Err(..) => Err(ModmailError::Database(DatabaseError::QueryFailed(
            "Failed to check if thread channel is orphaned".to_string(),
        ))),
    }
}
