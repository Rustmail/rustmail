use crate::commands::add_reminder::common::spawn_reminder;
use crate::config::Config;
use crate::db::reminders::get_all_pending_reminders;
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
