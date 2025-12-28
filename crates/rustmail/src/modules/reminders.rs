use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use serenity::all::Context;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn load_reminders(
    ctx: &Context,
    config: &Config,
    pool: &sqlx::SqlitePool,
    shutdown: Arc<Receiver<bool>>,
) {
    let reminders = get_all_pending_reminders(pool).await.unwrap_or_else(|e| {
        eprintln!("Failed to fetch pending reminders: {:?}", e);
        Vec::new()
    });

    for reminder in reminders {
        spawn_reminder(&reminder, None, &ctx, &config, &pool, shutdown.clone());
    }
    println!("All pending reminders have been scheduled.");
}
