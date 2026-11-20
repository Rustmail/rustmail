use crate::prelude::commands::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use sqlx::SqlitePool;

pub async fn current_ticket(invocation: &Invocation, pool: &SqlitePool) -> ModmailResult<Thread> {
    if !is_a_ticket_channel(invocation.channel_id(), pool).await {
        return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
    }
    get_thread_by_channel_id(&invocation.channel_id().to_string(), pool)
        .await
        .ok_or_else(thread_not_found)
}

pub async fn ticket_status(ticket: &Thread, pool: &SqlitePool) -> ModmailResult<TicketState> {
    get_thread_status(&ticket.id, pool)
        .await
        .ok_or_else(thread_not_found)
}

pub enum ChannelRename {
    Applied,
    Delayed,
}

pub async fn save_ticket_status(
    invocation: &Invocation,
    ticket: &Thread,
    status: &TicketState,
    pool: &SqlitePool,
) -> ModmailResult<ChannelRename> {
    update_thread_status_db(&ticket.id, status, pool).await?;
    let applied = update_thread_status_ui(invocation.ctx(), status)
        .await
        .unwrap_or(true);
    Ok(if applied {
        ChannelRename::Applied
    } else {
        ChannelRename::Delayed
    })
}
