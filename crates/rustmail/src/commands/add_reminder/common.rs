use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::utils::*;
use chrono::Local;
use serenity::all::{ChannelId, CommandInteraction, Context, GuildId, Message, RoleId, UserId};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
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
        let _ = MessageBuilder::system_message(&ctx, &config)
            .translated_content(
                "reminder.registered_with_content",
                Some(&params),
                None,
                None,
            )
            .await
            .to_channel(command.channel_id)
            .footer(format!("{}: {}", "ID", reminder_id))
            .send_interaction_followup(&command, true)
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
            .to_channel(command.channel_id)
            .footer(format!("{}: {}", "ID", reminder_id))
            .send_interaction_followup(&command, true)
            .await;
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

        let (mentions, is_role_targeted) = if let Some(ref target_roles_str) = reminder.target_roles
        {
            let role_mentions: String = target_roles_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .map(|id| format!("<@&{}>", id))
                .collect::<Vec<_>>()
                .join(", ");
            params.insert("roles".to_string(), role_mentions);

            let members =
                get_targeted_mentions(&ctx, &pool, reminder.guild_id as u64, target_roles_str)
                    .await;
            (members, true)
        } else {
            (vec![UserId::new(reminder.user_id as u64)], false)
        };

        if mentions.is_empty() {
            if let Err(e) = update_reminder_status(&reminder, true, &pool).await {
                eprintln!("Failed to update reminder status: {}", e);
            }
            return;
        }

        let (key_with_content, key_without_content) = if is_role_targeted {
            (
                "reminder.show_with_content_roles",
                "reminder.show_without_content_roles",
            )
        } else {
            (
                "reminder.show_with_content",
                "reminder.show_without_content",
            )
        };

        if !reminder.reminder_content.is_empty() {
            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content(key_with_content, Some(&params), None, None)
                .await
                .to_channel(ChannelId::new(reminder.channel_id as u64))
                .color(hex_string_to_int(&config.reminders.embed_color) as u32)
                .mention(mentions)
                .send(true)
                .await;
        } else {
            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content(key_without_content, Some(&params), None, None)
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

async fn get_targeted_mentions(
    ctx: &Context,
    pool: &SqlitePool,
    guild_id: u64,
    target_roles_str: &str,
) -> Vec<UserId> {
    let guild_id_obj = GuildId::new(guild_id);

    let role_ids: Vec<u64> = target_roles_str
        .split(',')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect();

    if role_ids.is_empty() {
        return vec![];
    }

    let members = match guild_id_obj.members(&ctx.http, None, None).await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to fetch guild members: {}", e);
            return vec![];
        }
    };

    let mut user_ids_with_roles: HashSet<UserId> = HashSet::new();

    for role_id in &role_ids {
        let role_id_obj = RoleId::new(*role_id);
        for member in &members {
            if member.roles.contains(&role_id_obj) {
                user_ids_with_roles.insert(member.user.id);
            }
        }
    }

    let mut opted_out_users: HashSet<u64> = HashSet::new();

    for role_id in &role_ids {
        match get_optouts_for_role(guild_id as i64, *role_id as i64, pool).await {
            Ok(optouts) => {
                for user_id in optouts {
                    opted_out_users.insert(user_id as u64);
                }
            }
            Err(e) => {
                eprintln!("Failed to get optouts for role {}: {}", role_id, e);
            }
        }
    }

    user_ids_with_roles
        .into_iter()
        .filter(|user_id| !opted_out_users.contains(&user_id.get()))
        .collect()
}
