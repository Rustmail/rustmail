use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::modules::*;
use chrono::Utc;
use serenity::all::{ChannelId, Context};
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time::interval;

const SWEEP_INTERVAL: Duration = Duration::from_secs(60);

pub fn close_inactive_tickets(ctx: &Context, config: &Config, mut shutdown: Receiver<bool>) {
    let timeout_minutes = config.thread.inactivity_timeout;
    if timeout_minutes == 0 {
        return;
    }

    let ctx = ctx.clone();
    let config = config.clone();

    tokio::spawn(async move {
        let mut interval = interval(SWEEP_INTERVAL);

        loop {
            tokio::select! {
                _ = interval.tick() => {}
                _ = shutdown.changed() => {}
            }
            if *shutdown.borrow() {
                return;
            }

            schedule_inactive_closures(&ctx, &config, timeout_minutes).await;
        }
    });
}

async fn schedule_inactive_closures(ctx: &Context, config: &Config, timeout_minutes: u64) {
    let Some(pool) = config.db_pool.as_ref() else {
        return;
    };

    let inactive_since = Utc::now() - chrono::Duration::minutes(timeout_minutes as i64);
    let threads = match get_inactive_threads(inactive_since, pool).await {
        Ok(threads) => threads,
        Err(e) => {
            eprintln!("Failed to load inactive tickets: {e:?}");
            return;
        }
    };

    let closed_by = ctx.cache.current_user().id.to_string();

    for thread in threads {
        let snapshot = match thread.channel_id.parse::<u64>() {
            Ok(channel_id) if channel_id != 0 => {
                ChannelSnapshot::take(ctx, ChannelId::new(channel_id)).await
            }
            _ => ChannelSnapshot::empty(),
        };
        let close_at = Utc::now().timestamp();

        if let Err(e) = upsert_scheduled_closure(
            &thread.id,
            close_at,
            false,
            &closed_by,
            &snapshot.category_id,
            &snapshot.category_name,
            &snapshot.required_permissions.to_string(),
            pool,
        )
        .await
        {
            eprintln!(
                "Failed to schedule closure of inactive ticket {}: {e:?}",
                thread.id
            );
            continue;
        }

        schedule_one(ctx, config, thread.id, close_at);
    }
}
