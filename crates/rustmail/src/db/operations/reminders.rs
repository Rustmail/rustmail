use crate::prelude::errors::*;

#[derive(Debug, Clone)]
pub struct Reminder {
    pub thread_id: String,
    pub user_id: i64,
    pub channel_id: i64,
    pub guild_id: i64,
    pub reminder_content: String,
    pub trigger_time: i64,
    pub created_at: i64,
    pub completed: bool,
    pub target_roles: Option<String>,
}

pub async fn insert_reminder(reminder: &Reminder, pool: &sqlx::SqlitePool) -> ModmailResult<i64> {
    let user_id = reminder.user_id;
    let channel_id = &reminder.channel_id;
    let guild_id = &reminder.guild_id;

    let existing = sqlx::query_scalar!(
        r#"
        SELECT id FROM reminders
        WHERE user_id = ? AND guild_id = ? AND trigger_time = ? AND completed = 0
        "#,
        reminder.user_id,
        reminder.guild_id,
        reminder.trigger_time
    )
    .fetch_optional(pool)
    .await?;

    if let Some(_existging_id) = existing {
        return Err(ModmailError::Command(CommandError::ReminderAlreadyExists));
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO reminders (thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed, target_roles)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        reminder.thread_id,
        user_id,
        channel_id,
        guild_id,
        reminder.reminder_content,
        reminder.trigger_time,
        reminder.created_at,
        reminder.completed,
        reminder.target_roles
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update_reminder_status(
    reminder: &Reminder,
    status: bool,
    pool: &sqlx::SqlitePool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE reminders
        SET completed = ?
        WHERE thread_id = ? AND trigger_time = ? AND completed = FALSE
        "#,
        status,
        reminder.thread_id,
        reminder.trigger_time
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all_pending_reminders(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Reminder,
        r#"
        SELECT thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed, target_roles
        FROM reminders
        WHERE completed = FALSE
        ORDER BY trigger_time ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_reminder_by_id(
    reminder_id: i64,
    pool: &sqlx::SqlitePool,
) -> Result<Option<Reminder>, sqlx::Error> {
    let row = sqlx::query_as!(
        Reminder,
        r#"
        SELECT thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed, target_roles
        FROM reminders
        WHERE id = ?
        "#,
        reminder_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn is_reminder_active(
    reminder_id: i64,
    pool: &sqlx::SqlitePool,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT completed
        FROM reminders
        WHERE id = ?
        "#,
        reminder_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = row {
        Ok(!record.completed)
    } else {
        Ok(false)
    }
}
