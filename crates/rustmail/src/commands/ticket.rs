use crate::prelude::commands::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use sqlx::SqlitePool;

/// The open ticket whose channel the command was run in.
pub async fn current_ticket(invocation: &Invocation, pool: &SqlitePool) -> ModmailResult<Thread> {
    if !is_a_ticket_channel(invocation.channel_id(), pool).await {
        return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
    }
    get_thread_by_channel_id(&invocation.channel_id().to_string(), pool)
        .await
        .ok_or_else(thread_not_found)
}

/// The stored status of a ticket (who took it, its label...).
pub async fn ticket_status(ticket: &Thread, pool: &SqlitePool) -> ModmailResult<TicketState> {
    get_thread_status(&ticket.id, pool)
        .await
        .ok_or_else(thread_not_found)
}

/// Saves a changed status and renames the ticket channel to match.
///
/// Returns whether the rename was applied now; `false` means it was rate limited and will be
/// applied later, which commands report with their `*_rate_limited` message.
pub async fn save_ticket_status(
    invocation: &Invocation,
    ticket: &Thread,
    status: &TicketState,
    pool: &SqlitePool,
) -> ModmailResult<bool> {
    update_thread_status_db(&ticket.id, status, pool).await?;
    Ok(update_thread_status_ui(invocation.ctx(), status)
        .await
        .unwrap_or(true))
}
