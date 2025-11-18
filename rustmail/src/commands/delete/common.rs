use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::utils::*;
use serenity::all::{ChannelId, Context, Message, MessageId, UserId};
use std::collections::HashMap;

pub async fn get_thread_info(
    channel_id: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<(i64, Thread)> {
    let user_id = match get_user_id_from_channel_id(&channel_id, pool).await {
        Some(uid) => uid,
        None => {
            return Err(validation_failed("Not in a thread"));
        }
    };

    let thread = match get_thread_by_channel_id(&channel_id, pool).await {
        Some(thread) => thread,
        None => {
            return Err(validation_failed("Thread not found"));
        }
    };

    Ok((user_id, thread))
}

pub async fn get_message_ids_for_delete(
    user_id: i64,
    thread: &Thread,
    message_number: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<MessageIds> {
    match get_message_ids_by_number(
        message_number,
        UserId::new(user_id as u64),
        &thread.id,
        pool,
    )
    .await
    {
        Some(ids) => Ok(ids),
        None => {
            let mut params = HashMap::new();
            params.insert("number".to_string(), message_number.to_string());
            Err(message_not_found("Try an other message number."))
        }
    }
}

pub async fn delete_discord_messages(
    ctx: &Context,
    channel_id: &ChannelId,
    user_id: i64,
    message_ids: &MessageIds,
) -> ModmailResult<()> {
    delete_inbox_message(ctx, channel_id, message_ids).await?;
    delete_dm_message(ctx, user_id, message_ids).await;

    Ok(())
}

pub async fn delete_inbox_message(
    ctx: &Context,
    channel_id: &ChannelId,
    message_ids: &MessageIds,
) -> ModmailResult<()> {
    if let Some(inbox_msg_id) = &message_ids.inbox_message_id
        && let Ok(msg_id) = inbox_msg_id.parse::<u64>()
        && let Err(e) = channel_id
            .delete_message(&ctx.http, MessageId::new(msg_id))
            .await
    {
        eprintln!("Failed to delete inbox message: {}", e);
        return Err(ModmailError::Command(CommandError::DiscordDeleteFailed));
    }
    Ok(())
}

pub async fn delete_dm_message(ctx: &Context, user_id: i64, message_ids: &MessageIds) {
    if let Some(dm_msg_id) = &message_ids.dm_message_id
        && let Ok(msg_id) = dm_msg_id.parse::<u64>()
    {
        if let Ok(user) = ctx.http.get_user(UserId::new(user_id as u64)).await {
            if let Ok(dm_channel) = user.create_dm_channel(&ctx.http).await {
                match dm_channel.message(&ctx.http, msg_id).await {
                    Ok(dm_message) => {
                        if let Err(e) = dm_message.delete(&ctx.http).await {
                            eprintln!("Failed to delete DM message: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch DM message for deletion: {}", e);
                    }
                }
            } else {
                eprintln!("Failed to create DM channel for deletion");
            }
        } else {
            eprintln!("Failed to get user for DM deletion");
        }
    }
}

pub async fn delete_database_message(
    message_ids: &MessageIds,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    if let Some(dm_msg_id) = &message_ids.dm_message_id
        && let Err(e) = delete_message(dm_msg_id, pool).await
    {
        eprintln!("Failed to delete message from database: {}", e);
        return Err(database_connection_failed());
    }
    Ok(())
}

pub async fn update_message_numbers(
    channel_id: &str,
    message_number: i64,
    pool: &sqlx::SqlitePool,
) {
    if let Err(e) = update_message_numbers_after_deletion(channel_id, message_number, pool).await {
        eprintln!("Failed to update message numbers: {}", e);
    }
}

pub async fn extract_message_number(msg: &Message, config: &Config) -> Option<i64> {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_name = "delete";

    if content.starts_with(&format!("{}{}", prefix, command_name)) {
        let start = prefix.len() + command_name.len();
        let args = content[start..].trim();

        if args.is_empty() {
            return None;
        }

        args.parse::<i64>().ok()
    } else {
        None
    }
}

pub async fn send_delete_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    message_key: &str,
    params: Option<&HashMap<String, String>>,
) {
    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(
            message_key,
            params,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(msg.channel_id)
        .send(true)
        .await;
}
