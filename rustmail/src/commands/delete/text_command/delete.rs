use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn delete(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let (user_id, thread) = get_thread_info(&msg.channel_id.to_string(), pool).await?;
    let message_number = extract_message_number(&msg, config).await;

    if message_number.is_none() {
        send_delete_message(&ctx, &msg, config, "delete.missing_number", None).await;
        return Ok(());
    }

    let message_number = message_number.unwrap();
    let message_ids = get_message_ids_for_delete(user_id, &thread, message_number, pool).await?;

    delete_discord_messages(&ctx, &msg.channel_id, user_id, &message_ids).await?;
    delete_database_message(&message_ids, pool).await?;
    update_message_numbers(&thread.channel_id, message_number, pool).await;

    let _ = msg.delete(&ctx.http).await;

    Ok(())
}
