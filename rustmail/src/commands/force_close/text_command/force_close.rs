use crate::commands::force_close::common::delete_channel;
use crate::config::Config;
use crate::db::threads::{is_a_ticket_channel, is_orphaned_thread_channel};
use crate::errors::DatabaseError::QueryFailed;
use crate::errors::ThreadError::{NotAThreadChannel, UserStillInServer};
use crate::errors::{common, ModmailError, ModmailResult};
use crate::types::logs::PaginationStore;
use serenity::all::{Context, Message};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn force_close(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    _shutdown: Arc<Receiver<bool>>,
    _pagination: PaginationStore,
) -> ModmailResult<()> {
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
