use serenity::all::{Message, UserId};
use sqlx::{Error, SqlitePool};

use crate::config::Config;
use crate::db::operations::threads::{
    get_next_message_number, get_user_name_from_thread_id, increment_message_number,
};

#[derive(Debug, Clone)]
pub struct MessageIds {
    pub dm_message_id: Option<String>,
    pub inbox_message_id: Option<String>,
}

pub async fn insert_user_message(
    msg: &Message,
    thread_id: &str,
    is_anonymous: bool,
    pool: &SqlitePool,
    config: &Config,
) -> Result<(), Error> {
    let user_id = msg.author.id.get() as i64;
    let dm_message_id = msg.id.to_string();

    let user_name = get_user_name_from_thread_id(thread_id, pool)
        .await
        .unwrap_or_else(|| msg.author.name.clone());

    let content = extract_message_content(msg, config);

    sqlx::query!(
        r#"
        INSERT INTO thread_messages (
            thread_id, user_id, user_name, is_anonymous, dm_message_id, content
        ) VALUES (
            ?, ?, ?, ?, ?, ?
        )
        "#,
        thread_id,
        user_id,
        user_name,
        is_anonymous,
        dm_message_id,
        content
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_staff_message(
    inbox_msg: &Message,
    dm_msg_id: Option<String>,
    thread_id: &str,
    staff_user_id: UserId,
    is_anonymous: bool,
    pool: &SqlitePool,
    config: &Config,
) -> Result<(), Error> {
    let inbox_message_id = inbox_msg.id.to_string();
    let user_id = staff_user_id.get() as i64;

    let user_name = get_user_name_from_thread_id(thread_id, pool)
        .await
        .unwrap_or_else(|| "Unknown".to_string());

    let content = extract_message_content(inbox_msg, config);
    let message_number = get_next_message_number(thread_id, pool).await as i64;

    sqlx::query!(
        r#"
        INSERT INTO thread_messages (
            thread_id, user_id, user_name, is_anonymous, dm_message_id, inbox_message_id, message_number, content
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
        thread_id,
        user_id,
        user_name,
        is_anonymous,
        dm_msg_id,
        inbox_message_id,
        message_number,
        content
    )
    .execute(pool)
    .await?;

    increment_message_number(thread_id, pool).await?;

    Ok(())
}

pub async fn get_message_ids_by_number(
    message_number: i64,
    user_id: UserId,
    pool: &SqlitePool,
) -> Option<MessageIds> {
    let user_id_i64 = user_id.get() as i64;

    let result = sqlx::query!(
        "SELECT dm_message_id, inbox_message_id FROM thread_messages WHERE user_id = ? AND message_number = ?",
        user_id_i64,
        message_number
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

pub async fn get_message_number_by_id(message_id: &str, pool: &SqlitePool) -> Option<i64> {
    let result = sqlx::query!(
        "SELECT message_number FROM thread_messages WHERE inbox_message_id = ? OR dm_message_id = ?",
        message_id,
        message_id
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(row)) => row.message_number,
        Ok(None) => None,
        Err(e) => {
            eprintln!("Database error getting message number: {:?}", e);
            None
        }
    }
}

pub async fn get_thread_messages(
    thread_id: &str,
    pool: &SqlitePool,
) -> Result<Vec<ThreadMessage>, Error> {
    let messages = sqlx::query!(
        r#"
        SELECT id, thread_id, user_id, user_name, is_anonymous,
               dm_message_id, inbox_message_id, message_number,
               created_at as "created_at: String", content
        FROM thread_messages
        WHERE thread_id = ?
        ORDER BY created_at ASC
        "#,
        thread_id
    )
    .fetch_all(pool)
    .await?;

    let thread_messages = messages
        .into_iter()
        .map(|row| ThreadMessage {
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
        })
        .collect();

    Ok(thread_messages)
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
