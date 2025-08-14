use sqlx::SqlitePool;
use crate::{
    db::{get_thread_by_channel_id, repr::Thread},
    errors::{ModmailResult, common},
};

pub async fn fetch_thread(db_pool: &SqlitePool, channel_id: &str) -> ModmailResult<Thread> {
    get_thread_by_channel_id(channel_id, db_pool)
        .await
        .ok_or_else(|| common::thread_not_found())
}
