use crate::prelude::db::*;
use crate::prelude::errors::*;
use sqlx::SqlitePool;

pub async fn fetch_thread(db_pool: &SqlitePool, channel_id: &str) -> ModmailResult<Thread> {
    get_thread_by_channel_id(channel_id, db_pool)
        .await
        .ok_or_else(thread_not_found)
}
