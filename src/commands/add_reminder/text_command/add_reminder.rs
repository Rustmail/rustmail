use crate::commands::add_reminder::common::{
    send_register_confirmation_from_message, spawn_reminder,
};
use crate::config::Config;
use crate::db::reminders::{insert_reminder, Reminder};
use crate::db::threads::get_thread_by_user_id;
use crate::errors::{
    common, CommandError, DatabaseError, ModmailError, ModmailResult, ThreadError,
};
use crate::utils::command::extract_reply_content::extract_reply_content;
use chrono::{Local, NaiveTime};
use regex::Regex;
use serenity::all::{Context, Message};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn add_reminder(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    shutdown: Arc<Receiver<bool>>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let content = match extract_reply_content(
        &msg.content,
        &config.command.prefix,
        &["add_reminder", "add_rap"],
    ) {
        Some(c) => c,
        None => {
            return Err(ModmailError::Command(CommandError::InvalidArguments(
                "No content provided".to_string(),
            )));
        }
    };

    let mut parts = content.splitn(2, ' ');
    let duration_str = parts.next().unwrap_or("");
    let reminder_content = parts.next().unwrap_or("");

    let re = Regex::new(r"^(?P<hour>[01]?\d|2[0-3]):(?P<minute>[0-5]\d)$").unwrap();
    let captures = re.captures(duration_str).ok_or_else(|| {
        ModmailError::Command(CommandError::InvalidArguments("duration".to_string()))
    })?;

    let hours: u32 = captures
        .name("hour")
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);

    let minutes: u32 = captures
        .name("minute")
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);

    let time = NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();
    let now = Local::now();
    let mut trigger_dt = now.date_naive().and_time(time);

    if trigger_dt < now.date_naive().and_time(time) {
        trigger_dt += chrono::Duration::days(1);
    }

    let trigger_timestamp = trigger_dt.and_local_timezone(Local).unwrap().timestamp();

    let thread = match get_thread_by_user_id(msg.author.id, pool).await {
        Some(t) => t,
        None => {
            return Err(ModmailError::Thread(ThreadError::ThreadNotFound));
        }
    };

    let reminder: Reminder = Reminder {
        thread_id: thread.id,
        user_id: msg.author.id.get() as i64,
        channel_id: msg.channel_id.get() as i64,
        guild_id: config.bot.get_staff_guild_id() as i64,
        reminder_content: reminder_content.to_string(),
        trigger_time: trigger_timestamp,
        created_at: now.timestamp(),
        completed: false,
    };

    let reminder_id = match insert_reminder(&reminder, pool).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to insert reminder: {}", e);
            return Err(ModmailError::Database(DatabaseError::InsertFailed(
                e.to_string(),
            )));
        }
    };

    send_register_confirmation_from_message(
        reminder_id,
        reminder_content,
        ctx,
        &msg,
        config,
        trigger_timestamp,
    )
    .await;

    let _ = msg.delete(&ctx.http).await;

    spawn_reminder(&reminder, Some(reminder_id), &ctx, &config, &pool, shutdown);

    Ok(())
}
