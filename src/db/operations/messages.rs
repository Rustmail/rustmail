use serenity::all::{Message, UserId};
use sqlx::{Error, SqlitePool};

use crate::config::Config;
use crate::db::operations::threads::get_user_name_from_thread_id;

#[derive(Debug, Clone)]
pub struct MessageIds {
    pub dm_message_id: Option<String>,
    pub inbox_message_id: Option<String>,
}

pub async fn insert_staff_message(
    inbox_msg: &Message,
    dm_msg_id: Option<String>,
    thread_id: &str,
    staff_user_id: UserId,
    is_anonymous: bool,
    pool: &SqlitePool,
    config: &Config,
    message_number: u64,
) -> Result<(), Error> {
    let inbox_message_id = inbox_msg.id.to_string();
    let user_id = staff_user_id.get() as i64;

    let user_name = get_user_name_from_thread_id(thread_id, pool)
        .await
        .unwrap_or_else(|| "Unknown".to_string());

    let content = extract_message_content(inbox_msg, config);
    let message_number_i64 = message_number as i64;

    sqlx::query!(
        r#"
        INSERT INTO thread_messages (
            thread_id, user_id, user_name, is_anonymous, dm_message_id, inbox_message_id, message_number, content, thread_status
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
        thread_id,
        user_id,
        user_name,
        is_anonymous,
        dm_msg_id,
        inbox_message_id,
        message_number_i64,
        content,
        1
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_message_ids_by_number(
    message_number: i64,
    _user_id: UserId,
    thread_id: &str,
    pool: &SqlitePool,
) -> Option<MessageIds> {
    let result = sqlx::query!(
        "SELECT dm_message_id, inbox_message_id FROM thread_messages WHERE message_number = ? AND thread_id = ?",
        message_number,
        thread_id
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(row)) => Some(MessageIds {
            dm_message_id: row.dm_message_id,
            inbox_message_id: row.inbox_message_id,
        }),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Database error getting message IDs: {:?}", e);
            None
        }
    }
}

pub async fn update_message_content(
    message_id: &str,
    new_content: &str,
    pool: &SqlitePool,
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE thread_messages SET content = ? WHERE dm_message_id = ? OR inbox_message_id = ?",
        new_content,
        message_id,
        message_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_message(message_id: &str, pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM thread_messages WHERE dm_message_id = ? OR inbox_message_id = ?",
        message_id,
        message_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_message_numbers_after_deletion(
    channel_id: &str,
    deleted_number: i64,
    pool: &SqlitePool,
) -> Result<(), Error> {
    let thread_id = match sqlx::query_scalar!(
        "SELECT id FROM threads WHERE channel_id = ? AND status = 1",
        channel_id
    )
    .fetch_optional(pool)
    .await?
    {
        Some(id) => id,
        None => return Ok(()),
    };

    sqlx::query!(
        "UPDATE thread_messages SET message_number = message_number - 1 
         WHERE thread_id = ? AND message_number > ? AND message_number IS NOT NULL",
        thread_id,
        deleted_number
    )
    .execute(pool)
    .await?;

    Ok(())
}

fn extract_message_content(msg: &Message, config: &Config) -> String {
    let mut content = if config.thread.embedded_message {
        msg.embeds
            .get(0)
            .and_then(|e| e.description.clone())
            .unwrap_or_else(|| msg.content.clone())
    } else {
        msg.content.clone()
    };

    if !content.is_empty() && config.thread.block_quote {
        content = content.replace(">>> ", "");
    }

    content
}

pub async fn get_latest_thread_message(
    thread_id: &str,
    pool: &SqlitePool,
) -> Result<Option<ThreadMessage>, Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, thread_id, user_id, user_name, is_anonymous,
               dm_message_id, inbox_message_id, message_number,
               created_at as "created_at: String", content
        FROM thread_messages
        WHERE thread_id = ?
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        thread_id
    )
    .fetch_optional(pool)
    .await?;

    let latest = row.map(|row| ThreadMessage {
        id: row.id as i64,
        thread_id: row.thread_id,
        user_id: row.user_id,
        user_name: row.user_name,
        is_anonymous: row.is_anonymous,
        dm_message_id: row.dm_message_id,
        inbox_message_id: row.inbox_message_id,
        message_number: row.message_number,
        created_at: row.created_at,
        content: row.content,
    });

    Ok(latest)
}

pub async fn insert_recovered_message(
    thread_id: &str,
    user_id: i64,
    user_name: &str,
    dm_message_id: &str,
    content: &str,
    pool: &SqlitePool,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO thread_messages (
            thread_id, user_id, user_name, is_anonymous, dm_message_id, content, thread_status
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?
        )
        "#,
        thread_id,
        user_id,
        user_name,
        false,
        dm_message_id,
        content,
        1
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_message_ids_by_message_id(
    message_id: &str,
    pool: &SqlitePool,
) -> Option<MessageIds> {
    let result = sqlx::query!(
        "SELECT dm_message_id, inbox_message_id FROM thread_messages WHERE dm_message_id = ? OR inbox_message_id = ?",
        message_id,
        message_id
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(row)) => Some(MessageIds {
            dm_message_id: row.dm_message_id,
            inbox_message_id: row.inbox_message_id,
        }),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Database error getting message IDs by message ID: {:?}", e);
            None
        }
    }
}

pub async fn insert_user_message_with_ids(
    dm_msg: &Message,
    thread_msg: &Message,
    thread_id: &str,
    is_anonymous: bool,
    pool: &SqlitePool,
    config: &Config,
) -> Result<(), Error> {
    let user_id = dm_msg.author.id.get() as i64;
    let dm_message_id = dm_msg.id.to_string();
    let inbox_message_id = thread_msg.id.to_string();

    let user_name = get_user_name_from_thread_id(thread_id, pool)
        .await
        .unwrap_or_else(|| dm_msg.author.name.clone());

    let content = extract_message_content(dm_msg, config);

    sqlx::query!(
        r#"
        INSERT INTO thread_messages (
            thread_id, user_id, user_name, is_anonymous, dm_message_id, inbox_message_id, content, thread_status
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
        thread_id,
        user_id,
        user_name,
        is_anonymous,
        dm_message_id,
        inbox_message_id,
        content,
        1
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ThreadMessage {
    pub id: i64,
    pub thread_id: String,
    pub user_id: i64,
    pub user_name: String,
    pub is_anonymous: bool,
    pub dm_message_id: Option<String>,
    pub inbox_message_id: Option<String>,
    pub message_number: Option<i64>,
    pub created_at: String,
    pub content: String,
}
