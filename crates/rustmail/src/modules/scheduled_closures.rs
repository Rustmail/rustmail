use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::utils::*;
use chrono::Utc;
use serenity::all::{ChannelId, Context, UserId};
use sqlx::SqlitePool;
use std::sync::LazyLock;
use tokio::time::{Duration, sleep};

/// One closure timer per thread id.
static CLOSURE_TIMERS: LazyLock<KeyedTasks<String>> = LazyLock::new(KeyedTasks::new);

pub fn schedule_one(ctx: &Context, config: &Config, thread_id: String, close_at: i64) {
    let now = Utc::now().timestamp();
    let delay_secs = (close_at - now).max(0) as u64;
    let ctx_clone = ctx.clone();
    let config_clone = config.clone();

    CLOSURE_TIMERS.spawn(thread_id.clone(), async move {
        if delay_secs > 0 {
            sleep(Duration::from_secs(delay_secs)).await;
        }
        let Some(pool) = config_clone.db_pool.as_ref() else {
            return;
        };
        let current = match get_scheduled_closure(&thread_id, pool).await {
            Ok(Some(current)) => current,
            Ok(None) => return,
            Err(e) => {
                eprintln!("Failed to load scheduled closure for thread {thread_id}: {e:?}");
                return;
            }
        };

        if current.close_at > Utc::now().timestamp() {
            schedule_one(
                &ctx_clone,
                &config_clone,
                thread_id.clone(),
                current.close_at,
            );
            return;
        }

        match get_thread_by_id(&thread_id, pool).await {
            Some(thread) => {
                execute_scheduled_closure(&ctx_clone, &config_clone, &thread, &current, pool).await
            }
            None => {
                if let Err(e) = delete_scheduled_closure(&thread_id, pool).await {
                    eprintln!("Failed to delete scheduled closure for thread {thread_id}: {e:?}");
                }
            }
        }
    });
}

async fn execute_scheduled_closure(
    ctx: &Context,
    config: &Config,
    thread: &Thread,
    closure: &ScheduledClosure,
    pool: &SqlitePool,
) {
    let closed = match close_thread(
        &thread.id,
        &closure.closed_by,
        &closure.category_id,
        &closure.category_name,
        closure.required_permissions.parse::<u64>().unwrap_or(0),
        pool,
    )
    .await
    {
        Ok(closed) => closed,
        Err(e) => {
            // The closure stays in the database and is retried on the next startup.
            eprintln!(
                "Failed to close thread {} for scheduled closure: {e:?}",
                thread.id
            );
            return;
        }
    };

    if let Err(e) = delete_scheduled_closure(&thread.id, pool).await {
        eprintln!(
            "Failed to delete scheduled closure for thread {}: {e:?}",
            thread.id
        );
    }

    if !closed {
        // Already closed by another path, which handled the logs, message and channel.
        return;
    }

    if config.bot.enable_rustmail_logs
        && let Some(logs_channel_id) = config.bot.logs_channel_id
    {
        let base_url = config
            .bot
            .redirect_url
            .trim_end_matches("/api/auth/callback")
            .trim_end_matches('/');

        let panel_url = format!("{}/panel/tickets/{}", base_url, thread.id);

        let mut params = std::collections::HashMap::new();
        params.insert("username".to_string(), thread.user_name.clone());
        params.insert("user_id".to_string(), thread.user_id.to_string());
        params.insert("panel_url".to_string(), panel_url);

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("logs.ticket_closed", Some(&params), None, None)
            .await
            .to_channel(ChannelId::new(logs_channel_id))
            .send(true)
            .await;
    }

    let effective_silent = closure.silent || is_thread_silent(&thread.id, pool).await;
    if !effective_silent && thread.user_id > 0 {
        let _ = MessageBuilder::system_message(ctx, config)
            .content(&config.bot.close_message)
            .to_user(UserId::new(thread.user_id as u64))
            .send(true)
            .await;
    }

    if let Ok(channel_id) = thread.channel_id.parse::<u64>()
        && channel_id != 0
    {
        let _ = ChannelId::new(channel_id).delete(&ctx.http).await;
    }
}

/// Re-arms every scheduled closure after startup; overdue ones fire immediately.
pub async fn hydrate_scheduled_closures(ctx: &Context, config: &Config) {
    let Some(pool) = config.db_pool.as_ref() else {
        return;
    };

    let list = match get_all_scheduled_closures(pool).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to load scheduled closures: {e:?}");
            return;
        }
    };

    for sc in list {
        schedule_one(ctx, config, sc.thread_id, sc.close_at);
    }
}
