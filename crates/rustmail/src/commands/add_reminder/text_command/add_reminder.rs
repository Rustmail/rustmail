use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use chrono::{Local, NaiveTime, TimeZone};
use regex::Regex;
use serenity::all::{Context, GuildId, Message, RoleId};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn add_reminder(
    ctx: Context,
    msg: Message,
    config: &Config,
    handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let content =
        match extract_reply_content(&msg.content, &config.command.prefix, &["remind", "rem"]) {
            Some(c) => c,
            None => {
                return Err(ModmailError::Command(CommandError::InvalidReminderFormat));
            }
        };

    let first_word = content.split_whitespace().next().unwrap_or("");
    match first_word.to_lowercase().as_str() {
        "subscribe" | "sub" => {
            let args = content.strip_prefix(first_word).unwrap_or("").trim();
            return handle_subscription(&ctx, &msg, config, pool, args, true).await;
        }
        "unsubscribe" | "unsub" => {
            let args = content.strip_prefix(first_word).unwrap_or("").trim();
            return handle_subscription(&ctx, &msg, config, pool, args, false).await;
        }
        _ => {}
    }

    let mut parts = content.splitn(2, ' ');
    let duration_str = parts.next().unwrap_or("");
    let rest_after_time = parts.next().unwrap_or("");

    let re = Regex::new(r"^(?P<hour>[01]?\d|2[0-3]):(?P<minute>[0-5]\d)$").unwrap();
    let captures = re
        .captures(&duration_str)
        .ok_or_else(|| ModmailError::Command(CommandError::InvalidReminderFormat))?;

    let hours: u32 = captures
        .name("hour")
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);

    let minutes: u32 = captures
        .name("minute")
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);

    let time = NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();
    let now = Local::now().with_timezone(&config.bot.timezone);

    let mut trigger_dt = config
        .bot
        .timezone
        .from_local_datetime(&now.date_naive().and_time(time))
        .unwrap();

    if trigger_dt < now {
        trigger_dt += chrono::Duration::days(1);
    }

    let trigger_timestamp = trigger_dt.with_timezone(&config.bot.timezone).timestamp();

    let thread = match get_thread_by_channel_id(&msg.channel_id.to_string(), pool).await {
        Some(t) => t,
        None => {
            return Err(ModmailError::Thread(ThreadError::ThreadNotFound));
        }
    };

    let (target_roles, reminder_content) =
        parse_roles_and_content(&ctx, config.bot.get_staff_guild_id(), rest_after_time).await;

    let reminder: Reminder = Reminder {
        thread_id: thread.id,
        user_id: msg.author.id.get() as i64,
        channel_id: msg.channel_id.get() as i64,
        guild_id: config.bot.get_staff_guild_id() as i64,
        reminder_content: reminder_content.to_string(),
        trigger_time: trigger_timestamp,
        created_at: now.timestamp(),
        completed: false,
        target_roles,
    };

    let reminder_id = match insert_reminder(&reminder, pool).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to insert reminder: {}", e);
            return Err(e);
        }
    };

    send_register_confirmation_from_message(
        reminder_id,
        &reminder_content,
        &ctx,
        &msg,
        config,
        trigger_timestamp,
    )
    .await;

    let _ = msg.delete(&ctx.http).await;

    spawn_reminder(
        &reminder,
        Some(reminder_id),
        &ctx,
        &config,
        &pool,
        handler.shutdown.clone(),
    );

    Ok(())
}

async fn parse_roles_and_content(
    ctx: &Context,
    guild_id: u64,
    content: &str,
) -> (Option<String>, String) {
    if content.is_empty() {
        return (None, String::new());
    }

    let mut parts = content.splitn(2, ' ');
    let first_word = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");

    if first_word.is_empty() {
        return (None, content.to_string());
    }

    let guild_id_obj = GuildId::new(guild_id);
    let guild = match guild_id_obj.to_partial_guild(&ctx.http).await {
        Ok(g) => g,
        Err(_) => return (None, content.to_string()),
    };

    let potential_role_names: Vec<&str> = first_word.split(',').map(|s| s.trim()).collect();

    let mut found_role_ids: Vec<u64> = Vec::new();
    let mut all_roles_valid = !potential_role_names.is_empty();

    for role_name in &potential_role_names {
        let role_id = role_name
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        if role_name.is_empty() {
            all_roles_valid = false;
            break;
        }

        let role_name_lower = role_name.to_lowercase();
        let found = guild.roles.values().find(|r| {
            r.name.to_lowercase() == role_name_lower || r.id.get().to_string() == *role_id
        });

        if let Some(role) = found {
            found_role_ids.push(role.id.get());
        } else {
            all_roles_valid = false;
            break;
        }
    }

    if all_roles_valid && !found_role_ids.is_empty() {
        let role_ids_str = found_role_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        (Some(role_ids_str), rest.to_string())
    } else {
        (None, content.to_string())
    }
}

async fn handle_subscription(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    pool: &SqlitePool,
    args: &str,
    is_subscribe: bool,
) -> ModmailResult<()> {
    let role_name = args.trim();

    if role_name.is_empty() {
        let mut params = HashMap::new();
        params.insert("prefix".to_string(), config.command.prefix.clone());

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "reminder_subscription.missing_role",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await;
        return Ok(());
    }

    let guild_id = config.bot.get_staff_guild_id();
    let guild_id_obj = GuildId::new(guild_id);
    let guild = match guild_id_obj.to_partial_guild(&ctx.http).await {
        Ok(g) => g,
        Err(_) => {
            return Err(ModmailError::Discord(DiscordError::ApiError(
                "Guild not found".to_string(),
            )));
        }
    };

    let role_name_lower = role_name.to_lowercase();
    let role = guild
        .roles
        .values()
        .find(|r| r.name.to_lowercase() == role_name_lower);

    let role = match role {
        Some(r) => r,
        None => {
            let mut params = HashMap::new();
            params.insert("role".to_string(), role_name.to_string());

            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content(
                    "reminder_subscription.role_not_found",
                    Some(&params),
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                )
                .await
                .reply_to(msg.clone())
                .send(true)
                .await;
            return Ok(());
        }
    };

    let member = match guild_id_obj.member(&ctx.http, msg.author.id).await {
        Ok(m) => m,
        Err(_) => {
            return Err(ModmailError::Discord(DiscordError::UserNotFound));
        }
    };

    if !member.roles.contains(&RoleId::new(role.id.get())) {
        let mut params = HashMap::new();
        params.insert("role".to_string(), role.name.clone());

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "reminder_subscription.role_required",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await;
        return Ok(());
    }

    let mut params = HashMap::new();
    params.insert("role".to_string(), role.name.clone());

    if is_subscribe {
        let was_opted_out = delete_reminder_optout(
            guild_id as i64,
            msg.author.id.get() as i64,
            role.id.get() as i64,
            pool,
        )
        .await?;

        let message_key = if was_opted_out {
            "reminder_subscription.subscribed"
        } else {
            "reminder_subscription.already_subscribed"
        };

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                message_key,
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await;
    } else {
        let is_already_opted_out = is_user_opted_out(
            guild_id as i64,
            msg.author.id.get() as i64,
            role.id.get() as i64,
            pool,
        )
        .await?;

        if is_already_opted_out {
            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content(
                    "reminder_subscription.already_unsubscribed",
                    Some(&params),
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                )
                .await
                .reply_to(msg.clone())
                .send(true)
                .await;
        } else {
            insert_reminder_optout(
                guild_id as i64,
                msg.author.id.get() as i64,
                role.id.get() as i64,
                pool,
            )
            .await?;

            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content(
                    "reminder_subscription.unsubscribed",
                    Some(&params),
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                )
                .await
                .reply_to(msg.clone())
                .send(true)
                .await;
        }
    }

    Ok(())
}
