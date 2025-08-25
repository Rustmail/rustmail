use crate::config::Config;
use crate::db::operations::{
    get_all_opened_threads, get_last_recovery_timestamp, get_latest_thread_message,
    insert_recovered_message, update_last_recovery_timestamp,
};
use crate::db::repr::Thread;
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use chrono::{DateTime, Utc};
use serenity::all::{ChannelId, Context, GetMessages, Message, MessageId, UserId};
use std::collections::HashMap;

pub struct MessageRecoveryResult {
    pub thread_id: String,
    pub recovered_count: u32,
    pub success: bool,
    pub error_message: Option<String>,
}

pub async fn recover_missing_messages(
    ctx: &Context,
    config: &Config,
) -> Vec<MessageRecoveryResult> {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return vec![];
        }
    };

    let last_recovery = match get_last_recovery_timestamp(pool).await {
        Ok(Some(timestamp)) => match DateTime::parse_from_rfc3339(&timestamp) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_) => Utc::now() - chrono::Duration::hours(1),
        },
        Ok(None) => Utc::now() - chrono::Duration::hours(1),
        Err(e) => {
            eprintln!("Failed to get last recovery timestamp: {}", e);
            Utc::now() - chrono::Duration::hours(1)
        }
    };

    let threads = get_all_opened_threads(pool).await;

    let mut results = Vec::new();

    for thread in threads {
        let result = recover_messages_for_thread(ctx, &thread, pool, config, last_recovery).await;
        results.push(result);
    }

    if let Err(e) = update_last_recovery_timestamp(pool).await {
        eprintln!("Failed to update last recovery timestamp: {}", e);
    }

    results
}

async fn recover_messages_for_thread(
    ctx: &Context,
    thread: &Thread,
    pool: &sqlx::SqlitePool,
    config: &Config,
    last_recovery: DateTime<Utc>,
) -> MessageRecoveryResult {
    let user_id = UserId::new(thread.user_id as u64);

    let latest_message = match get_latest_thread_message(&thread.id, pool).await {
        Ok(Some(msg)) => msg,
        Ok(None) => {
            return MessageRecoveryResult {
                thread_id: thread.id.clone(),
                recovered_count: 0,
                success: true,
                error_message: None,
            };
        }
        Err(e) => {
            return MessageRecoveryResult {
                thread_id: thread.id.clone(),
                recovered_count: 0,
                success: false,
                error_message: Some(format!("Failed to get latest message: {}", e)),
            };
        }
    };

    let last_dm_message_id = match &latest_message.dm_message_id {
        Some(id) => id,
        None => {
            return MessageRecoveryResult {
                thread_id: thread.id.clone(),
                recovered_count: 0,
                success: true,
                error_message: None,
            };
        }
    };

    let dm_channel = match user_id.create_dm_channel(&ctx.http).await {
        Ok(channel) => channel,
        Err(e) => {
            return MessageRecoveryResult {
                thread_id: thread.id.clone(),
                recovered_count: 0,
                success: false,
                error_message: Some(format!("Failed to create DM channel: {}", e)),
            };
        }
    };

    let message_id = match last_dm_message_id.parse::<u64>() {
        Ok(id) => MessageId::new(id),
        Err(_) => {
            return MessageRecoveryResult {
                thread_id: thread.id.clone(),
                recovered_count: 0,
                success: false,
                error_message: Some("Invalid message ID format".to_string()),
            };
        }
    };

    let messages = match dm_channel
        .messages(&ctx.http, GetMessages::new().after(message_id))
        .await
    {
        Ok(messages) => messages,
        Err(e) => {
            return MessageRecoveryResult {
                thread_id: thread.id.clone(),
                recovered_count: 0,
                success: false,
                error_message: Some(format!("Failed to fetch messages: {}", e)),
            };
        }
    };

    if messages.is_empty() {
        return MessageRecoveryResult {
            thread_id: thread.id.clone(),
            recovered_count: 0,
            success: true,
            error_message: None,
        };
    }

    let mut messages_vec: Vec<_> = messages.into_iter().collect();
    messages_vec.reverse();

    let mut recovered_count = 0;
    let channel_id = ChannelId::new(thread.channel_id.parse().unwrap_or(0));

    let bot_user_id = ctx.cache.current_user().id;

    for message in messages_vec {
        if message.author.id == bot_user_id {
            continue;
        }

        let message_timestamp = DateTime::from_timestamp(message.timestamp.unix_timestamp(), 0)
            .unwrap_or_else(Utc::now);
        if message_timestamp < last_recovery {
            continue;
        }

        let content = extract_message_content(&message, config);

        if content.trim().is_empty() {
            continue;
        }

        if let Err(e) = insert_recovered_message(
            &thread.id,
            thread.user_id,
            &thread.user_name,
            &message.id.to_string(),
            &content,
            pool,
        )
        .await
        {
            eprintln!("Failed to insert recovered message: {}", e);
            continue;
        }

        let _ = MessageBuilder::user_message(
            ctx,
            config,
            message.author.id,
            message.author.name.clone(),
        )
        .content(content)
        .to_channel(channel_id)
        .send()
        .await;

        recovered_count += 1;
    }

    if recovered_count > 0 {
        let mut params = HashMap::new();
        params.insert("count".to_string(), recovered_count.to_string());

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "recovery.messages_recovered",
                Some(&params),
                Some(user_id),
                None,
            )
            .await
            .to_channel(channel_id)
            .send()
            .await;
    }

    MessageRecoveryResult {
        thread_id: thread.id.clone(),
        recovered_count,
        success: true,
        error_message: None,
    }
}

pub async fn send_recovery_summary(
    _ctx: &Context,
    config: &Config,
    results: &[MessageRecoveryResult],
) {
    let total_recovered: u32 = results.iter().map(|r| r.recovered_count).sum();
    let successful_threads = results.iter().filter(|r| r.success).count();
    let failed_threads = results.len() - successful_threads;

    if total_recovered == 0 && failed_threads == 0 {
        return;
    }

    let mut params = HashMap::new();
    params.insert("total".to_string(), total_recovered.to_string());
    params.insert("threads".to_string(), successful_threads.to_string());
    params.insert("failed".to_string(), failed_threads.to_string());

    let summary_message =
        get_translated_message(config, "recovery.summary", Some(&params), None, None, None).await;

    println!("=== Récupération des messages terminée ===");
    println!("{}", summary_message);

    for result in results {
        if result.recovered_count > 0 {
            println!(
                "Thread {}: {} messages récupérés",
                result.thread_id, result.recovered_count
            );
        }
        if let Some(error) = &result.error_message {
            println!("Thread {}: Erreur - {}", result.thread_id, error);
        }
    }
}

fn extract_message_content(msg: &Message, config: &Config) -> String {
    let mut content = if config.thread.embedded_message {
        msg.embeds
            .first()
            .and_then(|e| e.description.clone())
            .unwrap_or_else(|| msg.content.clone())
    } else {
        msg.content.clone()
    };

    if !content.is_empty() && config.thread.block_quote {
        content = content.replace(">>> ", "");
    }

    content
}
