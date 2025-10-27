use crate::config::Config;
use crate::db::reminders::{is_reminder_active, update_reminder_status, Reminder};
use crate::utils::conversion::hex_string_to_int::hex_string_to_int;
use crate::utils::message::message_builder::MessageBuilder;
use chrono::Local;
use serenity::all::{ChannelId, CommandInteraction, Context, Message, UserId};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::sync::watch::Receiver;
use tokio::time::sleep;

pub async fn send_register_confirmation_from_message(
    reminder_id: i64,
    reminder_content: &str,
    ctx: &Context,
    msg: &Message,
    config: &Config,
    trigger_timestamp: i64,
) {
    let mut params = HashMap::new();
    params.insert("time".to_string(), format!("<t:{}:F>", trigger_timestamp));
    params.insert(
        "remaining_time".to_string(),
        format!("<t:{}:R>", trigger_timestamp),
    );

    if !reminder_content.is_empty() {
        params.insert("content".to_string(), reminder_content.to_string());
    }

    if !reminder_content.is_empty() {
        let _ = MessageBuilder::system_message(&ctx, &config)
            .translated_content(
                "reminder.registered_with_content",
                Some(&params),
                None,
                None,
            )
            .await
            .to_channel(msg.channel_id)
            .footer(format!("{}: {}", "ID", reminder_id))
            .send(true)
            .await;
    } else {
        let _ = MessageBuilder::system_message(&ctx, &config)
            .translated_content(
                "reminder.registered_without_content",
                Some(&params),
                None,
                None,
            )
            .await
            .to_channel(msg.channel_id)
            .footer(format!("{}: {}", "ID", reminder_id))
            .send(true)
            .await;
    }
}

pub async fn send_register_confirmation_from_command(
    reminder_id: i64,
    reminder_content: &str,
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
    trigger_timestamp: i64,
) {
    let mut params = HashMap::new();
    params.insert("time".to_string(), format!("<t:{}:F>", trigger_timestamp));
    params.insert(
        "remaining_time".to_string(),
        format!("<t:{}:R>", trigger_timestamp),
    );

    if !reminder_content.is_empty() {
        params.insert("content".to_string(), reminder_content.to_string());
    }

    if !reminder_content.is_empty() {
        let response = MessageBuilder::system_message(&ctx, &config)
            .translated_content(
                "reminder.registered_with_content",
                Some(&params),
                None,
                None,
            )
            .await
            .to_channel(command.channel_id)
            .footer(format!("{}: {}", "ID", reminder_id))
            .build_interaction_message_followup()
            .await;

        let _ = command.create_followup(&ctx.http, response).await;
    } else {
        let response = MessageBuilder::system_message(&ctx, &config)
            .translated_content(
                "reminder.registered_without_content",
                Some(&params),
                None,
                None,
            )
            .await
            .to_channel(command.channel_id)
            .footer(format!("{}: {}", "ID", reminder_id))
            .build_interaction_message_followup()
            .await;

        let _ = command.create_followup(&ctx.http, response).await;
    }
}

pub fn spawn_reminder(
    reminder: &Reminder,
    reminder_id: Option<i64>,
    ctx: &Context,
    config: &Config,
    pool: &SqlitePool,
    shutdown: Arc<Receiver<bool>>,
) {
    let pool = pool.clone();
    let config = config.clone();
    let ctx = ctx.clone();
    let reminder = reminder.clone();
    let mut shutdown_rx = shutdown.as_ref().clone();

    tokio::spawn(async move {
        let now = Local::now().timestamp();
        let delay_duration = if reminder.trigger_time > now {
            reminder.trigger_time - now
        } else {
            0
        };
        select! {
            _ = sleep(Duration::from_secs(delay_duration as u64)) => {}
            _ = shutdown_rx.changed() => {
                return;
            }
        }

        if let Some(reminder_id) = reminder_id {
            match is_reminder_active(reminder_id, &pool).await {
                Ok(active) => {
                    if !active {
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to check reminder status: {}", e);
                    return;
                }
            }
        }

        let mut params = HashMap::new();
        params.insert(
            "time".to_string(),
            format!("<t:{}:F>", reminder.trigger_time),
        );
        params.insert(
            "remaining_time".to_string(),
            format!("<t:{}:R>", reminder.trigger_time),
        );

        params.insert("user".to_string(), reminder.user_id.to_string());
        params.insert("content".to_string(), reminder.reminder_content.to_string());

        let mut mentions = Vec::<UserId>::new();
        mentions.push(UserId::new(reminder.user_id as u64));

        if !reminder.reminder_content.is_empty() {
            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content("reminder.show_with_content", Some(&params), None, None)
                .await
                .to_channel(ChannelId::new(reminder.channel_id as u64))
                .color(hex_string_to_int(&config.reminders.embed_color) as u32)
                .mention(mentions)
                .send(true)
                .await;
        } else {
            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content("reminder.show_without_content", Some(&params), None, None)
                .await
                .to_channel(ChannelId::new(reminder.channel_id as u64))
                .color(hex_string_to_int(&config.reminders.embed_color) as u32)
                .mention(mentions)
                .send(true)
                .await;
        }

        if let Err(e) = update_reminder_status(&reminder, true, &pool).await {
            eprintln!("Failed to update reminder status: {}", e);
        }
    });
}
