use crate::prelude::errors::*;
use crate::prelude::types::*;
use sqlx::SqlitePool;

pub async fn get_logs_from_user_id(
    user_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<Vec<TicketLog>> {
    let logs = sqlx::query_as!(
        TicketLog,
        r#"
        SELECT
            (COUNT(*) OVER ())
              - ROW_NUMBER() OVER (ORDER BY created_at DESC) + 1 AS id,
            id AS ticket_id,
            user_id AS "user_id: String",
            created_at AS "created_at: String"
        FROM threads
        WHERE user_id = ? AND status = 0
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!(
            "Database error getting logs for user ID {}: {:?}",
            user_id, e
        );
        ModmailError::Database(DatabaseError::QueryFailed(e.to_string()))
    })?;

    Ok(logs)
}
