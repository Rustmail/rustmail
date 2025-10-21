use crate::commands::logs::common::extract_user_id;
use crate::config::Config;
use crate::db::get_thread_by_channel_id;
use crate::db::logs::get_logs_from_user_id;
use crate::errors::{DatabaseError, ModmailError, ModmailResult, ThreadError};
use crate::features::make_buttons;
use crate::i18n::get_translated_message;
use crate::modules::commands::LOGS_PAGE_SIZE;
use crate::types::logs::{PaginationContext, PaginationStore, TicketLog};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{ButtonStyle, Context, Message};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::watch::Receiver;
use uuid::Uuid;

fn render_logs_page(logs: &[TicketLog], page: usize, per_page: usize) -> String {
    let total_pages = (logs.len() + per_page - 1) / per_page;
    let start = page * per_page;
    let end = usize::min(start + per_page, logs.len());

    let mut desc = String::new();

    for (_, log) in logs[start..end].iter().enumerate() {
        use std::fmt::Write;
        let _ = writeln!(
            desc,
            "**#{}** | [`Ticket {}`]({}) | Fermé le {} {}",
            log.id,
            log.ticket_id,
            format!("http://localhost:3002/panel/tickets/{}", log.ticket_id),
            log.created_at,
            "\n".to_string(),
        );
    }

    if desc.is_empty() {
        desc = "_Aucun log trouvé pour cet utilisateur._".into();
    }

    format!(
        "{}\n_Page {}/{} ({} logs totaux)_",
        desc,
        page + 1,
        total_pages.max(1),
        logs.len()
    )
}

pub async fn handle_logs_in_thread(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    pool: &SqlitePool,
    pagination: PaginationStore,
) -> ModmailResult<()> {
    let thread = match get_thread_by_channel_id(&msg.channel_id.to_string(), &pool).await {
        Some(thread) => thread,
        None => return Err(ModmailError::Thread(ThreadError::ThreadNotFound)),
    };

    handle_logs_from_user_id(
        &ctx,
        &msg,
        &config,
        &pool,
        &thread.user_id.to_string(),
        pagination,
    )
    .await
}

pub async fn handle_logs_from_user_id(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    pool: &SqlitePool,
    user_id: &str,
    pagination_store: PaginationStore,
) -> ModmailResult<()> {
    let logs = match get_logs_from_user_id(&user_id, &pool).await {
        Ok(logs) => logs,
        Err(e) => {
            eprintln!("Error retrieving logs for user ID {}: {:?}", user_id, e);
            return Err(ModmailError::Database(DatabaseError::QueryFailed(
                "Failed to retrieve logs.".to_string(),
            )));
        }
    };

    let page = 0;
    let content = render_logs_page(&logs, page, LOGS_PAGE_SIZE);
    let session_id = Uuid::new_v4().to_string();

    let next_button =
        get_translated_message(&config, "logs_command.next", None, None, None, None).await;
    let prev_button =
        get_translated_message(&config, "logs_command.prev", None, None, None, None).await;

    let components = make_buttons(&[
        (
            &prev_button.to_string(),
            &format!("command:logs_prev:{}", session_id),
            ButtonStyle::Primary,
            true,
        ),
        (
            &next_button.to_string(),
            &format!("command:logs_next:{}", session_id),
            ButtonStyle::Primary,
            false,
        ),
    ]);

    let response = MessageBuilder::system_message(&ctx, &config)
        .content(content)
        .components(components)
        .to_channel(msg.channel_id)
        .send(false)
        .await?;

    pagination_store.lock().await.insert(
        session_id.clone(),
        PaginationContext {
            user_id: user_id.to_string(),
            logs,
            current_page: page,
            message_id: response.id,
            channel_id: response.channel_id,
        },
    );

    Ok(())
}

pub async fn logs(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    _shutdown: Arc<Receiver<bool>>,
    pagination: PaginationStore,
) -> ModmailResult<()> {
    let pool = match config.db_pool.clone() {
        Some(pool) => pool.clone(),
        None => return Err(ModmailError::Database(DatabaseError::ConnectionFailed)),
    };

    let user_id = extract_user_id(&msg, &config);

    if user_id.is_empty() {
        handle_logs_in_thread(&ctx, &msg, &config, &pool, pagination).await
    } else {
        handle_logs_from_user_id(&ctx, &msg, &config, &pool, &user_id, pagination).await
    }
}
