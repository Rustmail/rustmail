use crate::config::Config;
use crate::utils::conversion::hex_string_to_int::hex_string_to_int;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, Message};
use std::collections::HashMap;

pub async fn send_register_confirmation(
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
            .send()
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
            .send()
            .await;
    }
}

pub async fn send_reminder_content(
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
            .translated_content("reminder.show_with_content", Some(&params), None, None)
            .await
            .to_channel(msg.channel_id)
            .color(hex_string_to_int(&config.reminders.embed_color) as u32)
            .send()
            .await;
    } else {
        let _ = MessageBuilder::system_message(&ctx, &config)
            .translated_content("reminder.show_without_content", None, None, None)
            .await
            .to_channel(msg.channel_id)
            .color(hex_string_to_int(&config.reminders.embed_color) as u32)
            .send()
            .await;
    }
}
