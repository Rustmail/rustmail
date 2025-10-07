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
}

pub async fn insert_reminder(
    reminder: &Reminder,
    pool: &sqlx::SqlitePool,
) -> Result<(), sqlx::Error> {
    let user_id = reminder.user_id;
    let channel_id = &reminder.channel_id;
    let guild_id = &reminder.guild_id;

    sqlx::query!(
        r#"
        INSERT INTO reminders (thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        reminder.thread_id,
        user_id,
        channel_id,
        guild_id,
        reminder.reminder_content,
        reminder.trigger_time,
        reminder.created_at,
        reminder.completed
    )
    .execute(pool)
    .await?;
    Ok(())
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
        SELECT thread_id, user_id, channel_id, guild_id, reminder_content, trigger_time, created_at, completed
        FROM reminders
        WHERE completed = FALSE
        ORDER BY trigger_time ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
