use crate::db::repr::Thread;
use crate::errors::ModmailResult;
use serenity::all::{ChannelId, GuildChannel, UserId};
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

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

pub async fn get_thread_by_user_id(user_id: UserId, pool: &SqlitePool) -> Option<Thread> {
    let user_id_i64 = user_id.get() as i64;

    sqlx::query_as!(
        Thread,
        "SELECT id, user_id, user_name, channel_id FROM threads WHERE user_id = ? AND status = 1",
        user_id_i64
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

pub async fn create_thread_for_user(
    channel: &GuildChannel,
    user_id: i64,
    user_name: &str,
    pool: &SqlitePool,
) -> Result<String, Error> {
    let channel_id = channel.id.to_string();
    let thread_id = Uuid::new_v4().to_string();

    match sqlx::query!(
        "INSERT INTO threads (id, user_id, user_name, channel_id) VALUES (?, ?, ?, ?)",
        thread_id,
        user_id,
        user_name,
        channel_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(thread_id),
        Err(Error::Database(db_err))
            if db_err.code() == Some(std::borrow::Cow::Borrowed("2067")) =>
        {
            if let Some(existing_thread_id) =
                sqlx::query_scalar("SELECT id FROM threads WHERE user_id = ? AND status = 1")
                    .bind(user_id)
                    .fetch_optional(pool)
                    .await?
            {
                Ok(existing_thread_id)
            } else {
                Err(Error::Database(db_err))
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn close_thread(thread_id: &str, pool: &SqlitePool) -> ModmailResult<()> {
    sqlx::query!("UPDATE threads SET status = 0 WHERE id = ?", thread_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn thread_exists(user_id: UserId, pool: &SqlitePool) -> bool {
    get_thread_channel_by_user_id(user_id, pool).await.is_some()
}

pub async fn is_a_ticket_channel(channel_id: ChannelId, pool: &SqlitePool) -> bool {
    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM threads WHERE channel_id = ?)")
        .bind(channel_id.to_string())
        .fetch_one(pool)
        .await
        .unwrap_or(false)
}

pub async fn get_all_opened_threads(pool: &SqlitePool) -> Vec<Thread> {
    let rows =
        sqlx::query!("SELECT id, user_id, user_name, channel_id FROM threads WHERE status = 1")
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

pub async fn update_thread_user_left(channel_id: &str, pool: &SqlitePool) -> ModmailResult<()> {
    sqlx::query!(
        "UPDATE threads SET user_left = 1 WHERE channel_id = ?",
        channel_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn is_user_left(channel_id: &str, pool: &SqlitePool) -> Result<bool, Error> {
    let thread = sqlx::query!(
        "SELECT user_left FROM threads WHERE channel_id = ?",
        channel_id
    )
    .fetch_all(pool)
    .await?;

    if let Some(row) = thread.first() {
        Ok(row.user_left)
    } else {
        Ok(false)
    }
}

pub async fn cancel_alert_for_staff(
    staff_user_id: serenity::all::UserId,
    thread_user_id: i64,
    pool: &SqlitePool,
) -> Result<(), Error> {
    let staff_user_id_i64 = staff_user_id.get() as i64;
    sqlx::query!(
        "DELETE FROM staff_alerts WHERE staff_user_id = ? AND thread_user_id = ?",
        staff_user_id_i64,
        thread_user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_alert_for_staff(
    staff_user_id: serenity::all::UserId,
    thread_user_id: i64,
    pool: &SqlitePool,
) -> Result<(), Error> {
    let staff_user_id_i64 = staff_user_id.get() as i64;
    sqlx::query!(
        "DELETE FROM staff_alerts WHERE staff_user_id = ? AND thread_user_id = ? AND used = FALSE",
        staff_user_id_i64,
        thread_user_id
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "INSERT INTO staff_alerts (staff_user_id, thread_user_id) VALUES (?, ?)",
        staff_user_id_i64,
        thread_user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_staff_alerts_for_user(
    thread_user_id: i64,
    pool: &SqlitePool,
) -> Result<Vec<i64>, Error> {
    let alerts = sqlx::query!(
        "SELECT staff_user_id FROM staff_alerts WHERE thread_user_id = ? AND used = FALSE",
        thread_user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(alerts.into_iter().map(|row| row.staff_user_id).collect())
}

pub async fn mark_alert_as_used(
    staff_user_id: i64,
    thread_user_id: i64,
    pool: &SqlitePool,
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE staff_alerts SET used = TRUE WHERE staff_user_id = ? AND thread_user_id = ? AND used = FALSE",
        staff_user_id,
        thread_user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn allocate_next_message_number(
    thread_id: &str,
    pool: &SqlitePool,
) -> Result<u64, Error> {
    let mut tx = pool.begin().await?;

    let current: Option<i64> =
        sqlx::query_scalar("SELECT next_message_number FROM threads WHERE id = ?")
            .bind(thread_id)
            .fetch_optional(&mut *tx)
            .await?;

    let num = current.unwrap_or(1);

    sqlx::query!(
        "UPDATE threads SET next_message_number = next_message_number + 1 WHERE id = ?",
        thread_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(num as u64)
}

pub async fn is_orphaned_thread_channel(
    channel_id: ChannelId,
    pool: &SqlitePool,
) -> Result<bool, Error> {
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM threads WHERE channel_id = ? AND status = 0 AND user_left = 1)")
        .bind(channel_id.to_string())
        .fetch_one(pool)
        .await?;

    Ok(exists)
}
