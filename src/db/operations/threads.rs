use serenity::all::{GuildChannel, Message, UserId};
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

use crate::db::repr::Thread;

pub async fn get_thread_channel_by_user_id(user_id: UserId, pool: &SqlitePool) -> Option<String> {
    sqlx::query_scalar("SELECT channel_id FROM threads WHERE user_id = ? AND status = 1")
        .bind(user_id.get() as i64)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error getting thread channel: {:?}", e);
            e
        })
        .ok()
        .flatten()
}

pub async fn get_thread_id_by_user_id(user_id: UserId, pool: &SqlitePool) -> Option<String> {
    sqlx::query_scalar("SELECT id FROM threads WHERE user_id = ? AND status = 1")
        .bind(user_id.get() as i64)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error getting thread ID: {:?}", e);
            e
        })
        .ok()
        .flatten()
}

pub async fn get_thread_by_channel_id(channel_id: &str, pool: &SqlitePool) -> Option<Thread> {
    sqlx::query_as!(
        Thread,
        "SELECT id, user_id, user_name, channel_id FROM threads WHERE channel_id = ? AND status = 1",
        channel_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error getting thread by channel ID: {:?}", e);
        e
    })
    .ok()
    .flatten()
}

pub async fn get_user_id_from_channel_id(channel_id: &str, pool: &SqlitePool) -> Option<i64> {
    sqlx::query_scalar("SELECT user_id FROM threads WHERE channel_id = ? AND status = 1")
        .bind(channel_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error getting user ID from channel: {:?}", e);
            e
        })
        .ok()
        .flatten()
}

pub async fn get_user_name_from_thread_id(thread_id: &str, pool: &SqlitePool) -> Option<String> {
    sqlx::query_scalar("SELECT user_name FROM threads WHERE id = ? AND status = 1")
        .bind(thread_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error getting user name: {:?}", e);
            e
        })
        .ok()
        .flatten()
}

pub async fn get_next_message_number(thread_id: &str, pool: &SqlitePool) -> u64 {
    sqlx::query_scalar("SELECT next_message_number FROM threads WHERE id = ?")
        .bind(thread_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error getting message number: {:?}", e);
            e
        })
        .ok()
        .flatten()
        .unwrap_or(1)
}

pub async fn increment_message_number(thread_id: &str, pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE threads SET next_message_number = next_message_number + 1 WHERE id = ?",
        thread_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_thread(
    channel: &GuildChannel,
    msg: &Message,
    pool: &SqlitePool,
) -> Result<String, Error> {
    let user_id = msg.author.id.get() as i64;
    let channel_id = channel.id.to_string();
    let thread_id = Uuid::new_v4().to_string();

    match sqlx::query!(
        "INSERT INTO threads (id, user_id, user_name, channel_id) VALUES (?, ?, ?, ?)",
        thread_id,
        user_id,
        msg.author.name,
        channel_id
    )
    .execute(pool)
    .await {
        Ok(_) => Ok(thread_id),
        Err(Error::Database(db_err)) if db_err.code() == Some(std::borrow::Cow::Borrowed("2067")) => {
            // SQLite constraint violation - thread already exists for this user
            // Get the existing thread ID
            if let Some(existing_thread_id) = sqlx::query_scalar("SELECT id FROM threads WHERE user_id = ? AND status = 1")
                .bind(user_id)
                .fetch_optional(pool)
                .await? {
                Ok(existing_thread_id)
            } else {
                Err(Error::Database(db_err))
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn close_thread(thread_id: &str, pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query!("UPDATE threads SET status = 0 WHERE id = ?", thread_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn thread_exists(user_id: UserId, pool: &SqlitePool) -> bool {
    get_thread_channel_by_user_id(user_id, pool).await.is_some()
}

pub async fn get_all_opened_threads(pool: &SqlitePool) -> Vec<Thread> {
    let rows = sqlx::query!(
        "SELECT id, user_id, user_name, channel_id FROM threads WHERE status = 1"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|row| Thread {
            id: row.id,
            user_id: row.user_id,
            user_name: row.user_name,
            channel_id: row.channel_id,
        })
        .collect()
}

// TODO: Implémenter quand la colonne user_left sera ajoutée à la base de données
pub async fn update_thread_user_left(_channel_id: &str, _pool: &SqlitePool) -> Result<(), Error> {
    // Temporairement désactivé jusqu'à ce que la colonne user_left soit ajoutée
    Ok(())
}

pub async fn is_user_left(_channel_id: &str, _pool: &SqlitePool) -> Result<bool, Error> {
    // Temporairement désactivé jusqu'à ce que la colonne user_left soit ajoutée
    Ok(false)
}
