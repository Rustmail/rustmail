use crate::prelude::errors::*;
use crate::prelude::types::*;

pub async fn create_snippet(
    key: &str,
    content: &str,
    created_by: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO snippets (key, content, created_by)
        VALUES (?, ?, ?)
        "#,
        key,
        content,
        created_by
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_snippet_by_key(
    key: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<Option<Snippet>> {
    let snippet = sqlx::query_as!(
        Snippet,
        r#"
        SELECT id as "id!", key, content, created_by, created_at as "created_at: String", updated_at as "updated_at: String"
        FROM snippets
        WHERE key = ?
        "#,
        key
    )
    .fetch_optional(pool)
    .await?;

    Ok(snippet)
}

pub async fn get_all_snippets(pool: &sqlx::SqlitePool) -> ModmailResult<Vec<Snippet>> {
    let snippets = sqlx::query_as!(
        Snippet,
        r#"
        SELECT id as "id!", key, content, created_by, created_at as "created_at: String", updated_at as "updated_at: String"
        FROM snippets
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(snippets)
}

pub async fn update_snippet(
    key: &str,
    content: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    let result = sqlx::query!(
        r#"
        UPDATE snippets
        SET content = ?, updated_at = CURRENT_TIMESTAMP
        WHERE key = ?
        "#,
        content,
        key
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ModmailError::Database(DatabaseError::NotFound(
            "Snippet not found".to_string(),
        )));
    }

    Ok(())
}

pub async fn delete_snippet(key: &str, pool: &sqlx::SqlitePool) -> ModmailResult<()> {
    let result = sqlx::query!(
        r#"
        DELETE FROM snippets
        WHERE key = ?
        "#,
        key
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ModmailError::Database(DatabaseError::NotFound(
            "Snippet not found".to_string(),
        )));
    }

    Ok(())
}
