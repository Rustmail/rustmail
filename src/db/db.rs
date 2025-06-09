pub use crate::db::operations::*;

use crate::config::Config;
use serenity::all::{GuildChannel, Message, UserId};
use sqlx::SqlitePool;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    crate::db::operations::init_database().await
}

pub async fn get_thread_by_id(user_id: UserId, pool: &SqlitePool) -> Option<String> {
    crate::db::operations::get_thread_channel_by_user_id(user_id, pool).await
}

pub async fn get_thread_id_by_id(user_id: UserId, pool: &SqlitePool) -> Option<String> {
    crate::db::operations::get_thread_id_by_user_id(user_id, pool).await
}

pub async fn get_thread_message_number_by_id(thread_id: &String, pool: &SqlitePool) -> u64 {
    crate::db::operations::get_next_message_number(thread_id, pool).await
}

pub async fn update_thread_message_number_from_id(
    thread_id: &String,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    crate::db::operations::increment_message_number(thread_id, pool).await
}

pub async fn create_channel_in_db(channel: &GuildChannel, msg: &Message, pool: SqlitePool) {
    match crate::db::operations::create_thread(channel, msg, &pool).await {
        Ok(_) => println!("Thread created successfully"),
        Err(err) => println!("Error creating thread: {}", err),
    }
}

pub async fn get_user_id_from_thread_id(thread_id: String, pool: &SqlitePool) -> Option<i64> {
    crate::db::operations::get_user_id_from_channel_id(&thread_id, pool).await
}

pub async fn get_user_name_from_thread_id(thread_id: String, pool: &SqlitePool) -> Option<String> {
    crate::db::operations::get_user_name_from_thread_id(&thread_id, pool).await
}

pub async fn insert_thread_message_in_db(
    msg: &Message,
    user_id: UserId,
    thread_id: String,
    is_anonymous: bool,
    is_staff_message: bool,
    config: &Config,
) {
    let pool = if let Some(pool) = &config.db_pool {
        pool
    } else {
        eprintln!("Database pool is not set in config.");
        return;
    };

    if is_staff_message {
        if let Err(e) = crate::db::operations::insert_staff_message(
            msg,
            None,
            &thread_id,
            user_id,
            is_anonymous,
            pool,
            config,
        )
        .await
        {
            eprintln!("Error inserting staff message: {}", e);
        }
    } else {
        if let Err(e) =
            crate::db::operations::insert_user_message(msg, &thread_id, is_anonymous, pool, config)
                .await
        {
            eprintln!("Error inserting user message: {}", e);
        }
    }
}

pub async fn insert_staff_thread_message_with_dm_in_db(
    inbox_msg: &Message,
    dm_msg: &Message,
    user_id: UserId,
    thread_id: String,
    is_anonymous: bool,
    config: &Config,
) {
    let pool = if let Some(pool) = &config.db_pool {
        pool
    } else {
        eprintln!("Database pool is not set in config.");
        return;
    };

    if let Err(e) = crate::db::operations::insert_staff_message(
        inbox_msg,
        Some(dm_msg.id.to_string()),
        &thread_id,
        user_id,
        is_anonymous,
        pool,
        config,
    )
    .await
    {
        eprintln!("Error inserting staff message with DM: {}", e);
    }
}

pub async fn get_thread_message_ids_from_message_number(
    message_number: i64,
    user_id: UserId,
    pool: &SqlitePool,
) -> Option<(Option<String>, Option<String>)> {
    let message_ids =
        crate::db::operations::get_message_ids_by_number(message_number, user_id, pool).await?;
    Some((message_ids.dm_message_id, message_ids.inbox_message_id))
}
