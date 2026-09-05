use crate::prelude::errors::*;

#[derive(Debug, Clone)]
pub struct ReminderData {
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

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: i64,
    pub data: ReminderData,
}

pub async fn insert_reminder(
    reminder_data: &ReminderData,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<Reminder> {
    let user_id = reminder_data.user_id;
    let channel_id = &reminder_data.channel_id;
    let guild_id = &reminder_data.guild_id;

    let result = sqlx::query!(
        r#"
        INSERT INTO reminders (thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed, target_roles)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        reminder_data.thread_id,
        user_id,
        channel_id,
        guild_id,
        reminder_data.reminder_content,
        reminder_data.trigger_time,
        reminder_data.created_at,
        reminder_data.completed,
        reminder_data.target_roles
    )
    .execute(pool)
    .await?;

    Ok(Reminder {
        id: result.last_insert_rowid(),
        data: reminder_data.clone(),
    })
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
        WHERE id = ? AND thread_id = ? AND trigger_time = ? AND completed = FALSE
        "#,
        status,
        reminder.id,
        reminder.data.thread_id,
        reminder.data.trigger_time
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all_pending_reminders(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed, target_roles
        FROM reminders
        WHERE completed = FALSE
        ORDER BY trigger_time ASC
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Reminder {
            id: row.id,
            data: ReminderData {
                thread_id: row.thread_id,
                user_id: row.user_id,
                channel_id: row.channel_id,
                guild_id: row.guild_id,
                reminder_content: row.reminder_content,
                trigger_time: row.trigger_time,
                created_at: row.created_at,
                completed: row.completed,
                target_roles: row.target_roles,
            },
        })
        .collect())
}

pub async fn get_reminder_by_id(
    reminder_id: i64,
    pool: &sqlx::SqlitePool,
) -> Result<Option<Reminder>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed, target_roles
        FROM reminders
        WHERE id = ?
        "#,
        reminder_id
    )
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|row| Reminder {
        id: row.id,
        data: ReminderData {
            thread_id: row.thread_id,
            user_id: row.user_id,
            channel_id: row.channel_id,
            guild_id: row.guild_id,
            reminder_content: row.reminder_content,
            trigger_time: row.trigger_time,
            created_at: row.created_at,
            completed: row.completed,
            target_roles: row.target_roles,
        },
    }))
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

    Ok(row.map(|record| !record.completed).unwrap_or(false))
}
