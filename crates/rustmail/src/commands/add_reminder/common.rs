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
    target_roles: &Option<String>,
) {
    let mut params = HashMap::new();
    params.insert("time".to_string(), format!("<t:{}:t>", trigger_timestamp));
    params.insert(
        "remaining_time".to_string(),
        format!("<t:{}:R>", trigger_timestamp),
    );

    if !reminder_content.is_empty() {
        params.insert("content".to_string(), reminder_content.to_string());
    }

    let has_roles = if let Some(roles_str) = target_roles {
        let role_mentions: String = roles_str
            .split(',')
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .map(|id| format!("<@&{}>", id))
            .collect::<Vec<_>>()
            .join(", ");
        params.insert("roles".to_string(), role_mentions);
        true
    } else {
        false
    };

    let key = match (has_roles, !reminder_content.is_empty()) {
        (true, true) => "reminder.registered_with_content_roles",
        (true, false) => "reminder.registered_without_content_roles",
        (false, true) => "reminder.registered_with_content",
        (false, false) => "reminder.registered_without_content",
    };

    let _ = MessageBuilder::system_message(&ctx, &config)
        .translated_content(key, Some(&params), None, None)
        .await
        .to_channel(msg.channel_id)
        .footer(format!("{}: {}", "ID", reminder_id))
        .send(true)
        .await;
}

pub async fn send_register_confirmation_from_command(
    reminder_id: i64,
    reminder_content: &str,
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
    trigger_timestamp: i64,
    target_roles: &Option<String>,
) {
    let mut params = HashMap::new();
    params.insert("time".to_string(), format!("<t:{}:t>", trigger_timestamp));
    params.insert(
        "remaining_time".to_string(),
        format!("<t:{}:R>", trigger_timestamp),
    );

    if !reminder_content.is_empty() {
        params.insert("content".to_string(), reminder_content.to_string());
    }

    let has_roles = if let Some(roles_str) = target_roles {
        let role_mentions: String = roles_str
            .split(',')
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .map(|id| format!("<@&{}>", id))
            .collect::<Vec<_>>()
            .join(", ");
        params.insert("roles".to_string(), role_mentions);
        true
    } else {
        false
    };

    let key = match (has_roles, !reminder_content.is_empty()) {
        (true, true) => "reminder.registered_with_content_roles",
        (true, false) => "reminder.registered_without_content_roles",
        (false, true) => "reminder.registered_with_content",
        (false, false) => "reminder.registered_without_content",
    };

    let _ = MessageBuilder::system_message(&ctx, &config)
        .translated_content(key, Some(&params), None, None)
        .await
        .to_channel(command.channel_id)
        .footer(format!("{}: {}", "ID", reminder_id))
        .send_interaction_followup(&command, true)
        .await;
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

    let mut optouts_by_role: HashMap<u64, HashSet<u64>> = HashMap::new();
    for role_id in &role_ids {
        match get_optouts_for_role(guild_id as i64, *role_id as i64, pool).await {
            Ok(optouts) => {
                optouts_by_role.insert(*role_id, optouts.into_iter().map(|id| id as u64).collect());
            }
            Err(e) => {
                eprintln!("Failed to get optouts for role {}: {}", role_id, e);
                optouts_by_role.insert(*role_id, HashSet::new());
            }
        }
    }

    let mut users_to_mention: Vec<UserId> = Vec::new();

    for member in &members {
        let user_target_roles: Vec<u64> = role_ids
            .iter()
            .filter(|&&role_id| member.roles.contains(&RoleId::new(role_id)))
            .copied()
            .collect();

        if user_target_roles.is_empty() {
            continue;
        }

        let has_subscribed_role = user_target_roles.iter().any(|role_id| {
            !optouts_by_role
                .get(role_id)
                .map(|optouts| optouts.contains(&member.user.id.get()))
                .unwrap_or(false)
        });

        if has_subscribed_role {
            users_to_mention.push(member.user.id);
        }
    }

    users_to_mention
}
