use crate::modules::update_thread_status_ui;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn rename_ticket(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if !is_a_ticket_channel(msg.channel_id, &db_pool).await {
        return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
    }

    let thread = match get_thread_by_channel_id(&msg.channel_id.to_string(), db_pool).await {
        Some(thread) => thread,
        None => return Err(thread_not_found()),
    };

    let new_label = extract_reply_content(&msg.content, &config.command.prefix, &["rename", "rn"])
        .unwrap_or_default();

    let config_clone = config.clone();

    tokio::spawn({
        let db_pool = db_pool.clone();
        async move {
            let mut ticket_status = match get_thread_status(&thread.id, &db_pool).await {
                Some(status) => status,
                None => return,
            };

            if new_label.is_empty() {
                ticket_status.label = None;
            } else {
                ticket_status.label = Some(new_label.clone());
            }

            let _ = update_thread_status_db(&thread.id, &ticket_status, &db_pool).await;

            let has_label = ticket_status.label.is_some();
            let applied = update_thread_status_ui(&ctx, &ticket_status)
                .await
                .unwrap_or(true);

            let key = match (has_label, applied) {
                (true, true) => "rename.confirmation",
                (true, false) => "rename.confirmation_rate_limited",
                (false, true) => "rename.cleared",
                (false, false) => "rename.cleared_rate_limited",
            };

            let params = if has_label {
                let mut p = std::collections::HashMap::new();
                p.insert("label".to_string(), new_label);
                Some(p)
            } else {
                None
            };

            let _ = MessageBuilder::system_message(&ctx, &config_clone)
                .translated_content(key, params.as_ref(), None, None)
                .await
                .to_channel(msg.channel_id)
                .send(true)
                .await;
        }
    });

    Ok(())
}
