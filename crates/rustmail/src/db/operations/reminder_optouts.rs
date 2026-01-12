use crate::prelude::errors::*;

pub async fn insert_reminder_optout(
    guild_id: i64,
    user_id: i64,
    role_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO reminder_optouts (guild_id, user_id, role_id)
        VALUES (?, ?, ?)
        "#,
        guild_id,
        user_id,
        role_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_reminder_optout(
    guild_id: i64,
    user_id: i64,
    role_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<bool> {
    let result = sqlx::query!(
        r#"
        DELETE FROM reminder_optouts
        WHERE guild_id = ? AND user_id = ? AND role_id = ?
        "#,
        guild_id,
        user_id,
        role_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_optouts_for_role(
    guild_id: i64,
    role_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<Vec<i64>> {
    let rows = sqlx::query_scalar!(
        r#"
        SELECT user_id FROM reminder_optouts
        WHERE guild_id = ? AND role_id = ?
        "#,
        guild_id,
        role_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn is_user_opted_out(
    guild_id: i64,
    user_id: i64,
    role_id: i64,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<bool> {
    let result = sqlx::query!(
        r#"
        SELECT id FROM reminder_optouts
        WHERE guild_id = ? AND user_id = ? AND role_id = ?
        "#,
        guild_id,
        user_id,
        role_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}
