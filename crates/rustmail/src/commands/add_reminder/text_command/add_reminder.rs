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
        target_roles: target_roles.clone(),
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
        &target_roles,
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

    let guild_id_obj = GuildId::new(guild_id);
    let guild = match guild_id_obj.to_partial_guild(&ctx.http).await {
        Ok(g) => g,
        Err(_) => return (None, content.to_string()),
    };

    let mention_regex = Regex::new(r"<@&(\d+)>").unwrap();
    let at_role_regex = Regex::new(r"^@(.+)$").unwrap();

    let mut found_role_ids: Vec<u64> = Vec::new();
    let mut last_valid_end = 0;

    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        while i < chars.len() && (chars[i].is_whitespace() || chars[i] == ',') {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }

        let word_start = i;
        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != ',' {
            i += 1;
        }
        let word: String = chars[word_start..i].iter().collect();

        if word.is_empty() {
            continue;
        }

        let role_id: Option<u64> = if let Some(caps) = mention_regex.captures(&word) {
            caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok())
        } else if let Some(caps) = at_role_regex.captures(&word) {
            let role_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let role_name_lower = role_name.to_lowercase();
            guild
                .roles
                .values()
                .find(|r| r.name.to_lowercase() == role_name_lower)
                .map(|r| r.id.get())
        } else {
            None
        };

        if let Some(id) = role_id {
            if guild.roles.contains_key(&RoleId::new(id)) {
                found_role_ids.push(id);
                last_valid_end = i;
            } else {
                break;
            }
        } else if word.starts_with('@') || word.starts_with("<@&") {
            break;
        } else {
            break;
        }
    }

    if !found_role_ids.is_empty() {
        let remaining = content.chars().skip(last_valid_end).collect::<String>();
        let remaining = remaining.trim_start_matches(|c: char| c.is_whitespace() || c == ',');
        let role_ids_str = found_role_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        (Some(role_ids_str), remaining.to_string())
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
        return Err(ModmailError::Command(CommandError::MissingArguments));
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
            return Err(ModmailError::Command(CommandError::ReminderRoleNotFound(
                role_name.to_string(),
            )));
        }
    };

    let member = match guild_id_obj.member(&ctx.http, msg.author.id).await {
        Ok(m) => m,
        Err(_) => {
            return Err(ModmailError::Discord(DiscordError::UserNotFound));
        }
    };

    if !member.roles.contains(&RoleId::new(role.id.get())) {
        return Err(ModmailError::Command(CommandError::ReminderRoleRequired(
            role.name.clone(),
        )));
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

        if !was_opted_out {
            return Err(ModmailError::Command(CommandError::ReminderAlreadySubscribed(
                role.name.clone(),
            )));
        }

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "reminder_subscription.subscribed",
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
            return Err(ModmailError::Command(CommandError::ReminderAlreadyUnsubscribed(
                role.name.clone(),
            )));
        }

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

    Ok(())
}
