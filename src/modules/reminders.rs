use crate::config::Config;
use crate::db::reminders::{get_all_pending_reminders, update_reminder_status};
use crate::utils::conversion::hex_string_to_int::hex_string_to_int;
use crate::utils::message::message_builder::MessageBuilder;
use chrono::Local;
use serenity::all::{ChannelId, Context};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

pub async fn load_reminders(ctx: &Context, config: &Config, pool: &sqlx::SqlitePool) {
    let reminders = get_all_pending_reminders(pool).await.unwrap_or_else(|e| {
        eprintln!("Failed to fetch pending reminders: {:?}", e);
        Vec::new()
    });

    for reminder in reminders {
        let pool = pool.clone();
        let config = config.clone();
        let ctx = ctx.clone();

        tokio::spawn(async move {
            let now = Local::now().timestamp();
            let delay_duration = if reminder.trigger_time > now {
                reminder.trigger_time - now
            } else {
                0
            };

            sleep(Duration::from_secs(delay_duration as u64)).await;

            let channel_id = ChannelId::new(reminder.channel_id as u64);

            let mut params = HashMap::new();
            params.insert("content".to_string(), reminder.reminder_content.clone());

            if !reminder.reminder_content.is_empty() {
                let _ = MessageBuilder::system_message(&ctx, &config)
                    .translated_content("reminder.show_with_content", Some(&params), None, None)
                    .await
                    .to_channel(channel_id)
                    .color(hex_string_to_int(&config.reminders.embed_color) as u32)
                    .send()
                    .await;
            } else {
                let _ = MessageBuilder::system_message(&ctx, &config)
                    .translated_content("reminder.show_without_content", None, None, None)
                    .await
                    .to_channel(channel_id)
                    .color(hex_string_to_int(&config.reminders.embed_color) as u32)
                    .send()
                    .await;
            }

            if let Err(e) = update_reminder_status(&reminder, true, &pool).await {
                eprintln!("Failed to update reminder status: {}", e);
            }
        });
    }
    println!("All pending reminders have been scheduled.");
}
