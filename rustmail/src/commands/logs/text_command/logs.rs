use crate::commands::logs::common::{extract_user_id, get_response, render_logs_page};
use crate::config::Config;
use crate::db::get_thread_by_channel_id;
use crate::db::logs::get_logs_from_user_id;
use crate::errors::{DatabaseError, ModmailError, ModmailResult, ThreadError};
use crate::features::make_buttons;
use crate::i18n::get_translated_message;
use crate::modules::commands::LOGS_PAGE_SIZE;
use crate::types::logs::{PaginationContext, PaginationStore};
use serenity::all::{ButtonStyle, ChannelId, CommandInteraction, Context, Message};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::watch::Receiver;
use uuid::Uuid;

pub async fn handle_logs_in_thread(
    ctx: &Context,
    channel_id: &ChannelId,
    command: Option<CommandInteraction>,
    config: &Config,
    pool: &SqlitePool,
    pagination: PaginationStore,
) -> ModmailResult<()> {
    let thread = match get_thread_by_channel_id(&channel_id.to_string(), &pool).await {
        Some(thread) => thread,
        None => return Err(ModmailError::Thread(ThreadError::ThreadNotFound)),
    };

    handle_logs_from_user_id(
        &ctx,
        channel_id,
        command,
        &config,
        &pool,
        &thread.user_id.to_string(),
        pagination,
    )
    .await
}

pub async fn handle_logs_from_user_id(
    ctx: &Context,
    channel_id: &ChannelId,
    command: Option<CommandInteraction>,
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
            page == 0,
        ),
        (
            &next_button.to_string(),
            &format!("command:logs_next:{}", session_id),
            ButtonStyle::Primary,
            (page + 1) * 10 >= logs.len(),
        ),
    ]);

    let response = get_response(
        ctx.clone(),
        config.clone(),
        &content,
        components,
        *channel_id,
        command,
    )
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
        handle_logs_in_thread(&ctx, &msg.channel_id, None, &config, &pool, pagination).await
    } else {
        handle_logs_from_user_id(
            &ctx,
            &msg.channel_id,
            None,
            config,
            &pool,
            &user_id.to_string(),
            pagination,
        )
        .await
    }
}
