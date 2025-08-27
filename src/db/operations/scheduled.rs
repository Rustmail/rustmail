use crate::errors::{ModmailResult, common};
use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone)]
pub struct ScheduledClosure {
    pub thread_id: String,
    pub close_at: i64,
    pub silent: bool,
}

pub async fn upsert_scheduled_closure(
    thread_id: &str,
    close_at: i64,
    silent: bool,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    sqlx::query(
        r#"INSERT INTO scheduled_closures(thread_id, close_at, silent) VALUES(?, ?, ?)
            ON CONFLICT(thread_id) DO UPDATE SET close_at=excluded.close_at, silent=excluded.silent"#,
    )
    .bind(thread_id)
    .bind(close_at)
    .bind(silent)
    .execute(pool)
    .await
    .map_err(|_| common::validation_failed("Failed to upsert scheduled closure"))?;
    Ok(())
}

pub async fn delete_scheduled_closure(thread_id: &str, pool: &SqlitePool) -> ModmailResult<bool> {
    let res = sqlx::query("DELETE FROM scheduled_closures WHERE thread_id = ?")
        .bind(thread_id)
        .execute(pool)
        .await
        .map_err(|_| common::validation_failed("Failed to delete scheduled closure"))?;
    Ok(res.rows_affected() > 0)
}

pub async fn get_scheduled_closure(
    thread_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<Option<ScheduledClosure>> {
    let row_opt = sqlx::query(
        "SELECT thread_id, close_at, silent FROM scheduled_closures WHERE thread_id = ?",
    )
    .bind(thread_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| common::validation_failed("Failed to fetch scheduled closure"))?;

    Ok(row_opt.map(|row| ScheduledClosure {
        thread_id: row.get::<String, _>(0),
        close_at: row.get::<i64, _>(1),
        silent: row.get::<i64, _>(2) != 0,
    }))
}

pub async fn get_all_scheduled_closures(pool: &SqlitePool) -> ModmailResult<Vec<ScheduledClosure>> {
    let rows = sqlx::query("SELECT thread_id, close_at, silent FROM scheduled_closures")
        .fetch_all(pool)
        .await
        .map_err(|_| common::validation_failed("Failed to fetch scheduled closures"))?;

    Ok(rows
        .into_iter()
        .map(|row| ScheduledClosure {
            thread_id: row.get::<String, _>(0),
            close_at: row.get::<i64, _>(1),
            silent: row.get::<i64, _>(2) != 0,
        })
        .collect())
}
