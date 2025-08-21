use sqlx::SqlitePool;

pub async fn get_feature_message(
    feature_key: &str,
    pool: &SqlitePool,
) -> Result<Option<(String, String)>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String, String)>(
        r#"SELECT channel_id, message_id FROM features_messages WHERE feature_key = ?"#,
    )
    .bind(feature_key)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn upsert_feature_message(
    feature_key: &str,
    channel_id: &str,
    message_id: &str,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO features_messages (feature_key, channel_id, message_id, updated_at)
        VALUES (?, ?, ?, datetime('now'))
        ON CONFLICT(feature_key) DO UPDATE SET
            channel_id = excluded.channel_id,
            message_id = excluded.message_id,
            updated_at = datetime('now')
        "#,
    )
    .bind(feature_key)
    .bind(channel_id)
    .bind(message_id)
    .execute(pool)
    .await?;
    Ok(())
}
