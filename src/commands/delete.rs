use crate::config::Config;
use crate::db::messages::MessageIds;
use crate::db::operations::{
    delete_message, get_message_ids_by_number, get_thread_by_channel_id,
    get_user_id_from_channel_id, update_message_numbers_after_deletion,
};
use crate::db::repr::Thread;
use crate::errors::{ModmailResult, common};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, Message, MessageId, UserId};
use std::collections::HashMap;

pub async fn delete(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let (user_id, thread) = get_thread_info(ctx, msg, config, pool).await?;
    let message_number = extract_message_number(msg, config).await;

    if message_number.is_none() {
        send_delete_message(ctx, msg, config, "delete.missing_number", None).await;
        return Ok(());
    }

    let message_number = message_number.unwrap();
    let message_ids =
        get_message_ids(ctx, msg, config, user_id, &thread, message_number, pool).await?;

    delete_discord_messages(ctx, msg, config, user_id, &message_ids).await;
    delete_database_message(&message_ids, pool, ctx, msg, config).await?;
    update_message_numbers(&thread.channel_id, message_number, pool).await;

    if config.notifications.show_success_on_delete {
        let mut params = HashMap::new();
        params.insert("number".to_string(), message_number.to_string());
        send_delete_message(ctx, msg, config, "delete.success", Some(&params)).await;
    }
    let _ = msg.delete(&ctx.http).await;

    Ok(())
}

async fn get_thread_info(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<(i64, Thread)> {
    let channel_id = msg.channel_id.to_string();
    let user_id = match get_user_id_from_channel_id(&channel_id, pool).await {
        Some(uid) => uid,
        None => {
            send_delete_message(ctx, msg, config, "delete.not_in_thread", None).await;
            return Err(common::validation_failed("Not in a thread"));
        }
    };

    let thread = match get_thread_by_channel_id(&channel_id, pool).await {
        Some(thread) => thread,
        None => {
            send_delete_message(ctx, msg, config, "delete.not_in_thread", None).await;
            return Err(common::validation_failed("Thread not found"));
        }
    };

    Ok((user_id, thread))
}

async fn get_message_ids(
    ctx: &Context,
    msg: &Message,
    config: &Config,
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
            send_delete_message(ctx, msg, config, "delete.message_not_found", Some(&params)).await;
            Err(common::message_not_found("Try an other message number."))
        }
    }
}

async fn delete_discord_messages(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    user_id: i64,
    message_ids: &crate::db::operations::messages::MessageIds,
) {
    delete_inbox_message(ctx, msg, config, message_ids).await;
    delete_dm_message(ctx, user_id, message_ids).await;
}

async fn delete_inbox_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    message_ids: &crate::db::operations::messages::MessageIds,
) {
    if let Some(inbox_msg_id) = &message_ids.inbox_message_id
        && let Ok(msg_id) = inbox_msg_id.parse::<u64>()
        && let Err(e) = msg
            .channel_id
            .delete_message(&ctx.http, MessageId::new(msg_id))
            .await
    {
        eprintln!("Failed to delete inbox message: {}", e);
        send_delete_message(ctx, msg, config, "delete.discord_delete_failed", None).await;
    }
}

async fn delete_dm_message(
    ctx: &Context,
    user_id: i64,
    message_ids: &crate::db::operations::messages::MessageIds,
) {
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

async fn delete_database_message(
    message_ids: &crate::db::operations::messages::MessageIds,
    pool: &sqlx::SqlitePool,
    ctx: &Context,
    msg: &Message,
    config: &Config,
) -> ModmailResult<()> {
    if let Some(dm_msg_id) = &message_ids.dm_message_id
        && let Err(e) = delete_message(dm_msg_id, pool).await
    {
        eprintln!("Failed to delete message from database: {}", e);
        send_delete_message(ctx, msg, config, "delete.database_delete_failed", None).await;
        return Err(common::database_connection_failed());
    }
    Ok(())
}

async fn update_message_numbers(channel_id: &str, message_number: i64, pool: &sqlx::SqlitePool) {
    if let Err(e) = update_message_numbers_after_deletion(channel_id, message_number, pool).await {
        eprintln!("Failed to update message numbers: {}", e);
    }
}

async fn extract_message_number(msg: &Message, config: &Config) -> Option<i64> {
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

async fn send_delete_message(
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
        .send()
        .await;
}
