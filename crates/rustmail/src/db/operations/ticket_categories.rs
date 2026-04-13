use crate::db::repr::{PendingCategorySelection, TicketCategory, TicketCategorySettings};
use crate::prelude::errors::*;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub const CATEGORY_BUTTON_HARD_LIMIT: usize = 24;

pub async fn get_category_settings(pool: &SqlitePool) -> ModmailResult<TicketCategorySettings> {
    let row = sqlx::query(
        "SELECT enabled, selection_timeout_s FROM ticket_category_settings WHERE id = 1",
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch category settings: {e:?}");
        validation_failed("Failed to fetch category settings")
    })?;

    Ok(match row {
        Some(row) => TicketCategorySettings {
            enabled: row.get::<i64, _>("enabled") != 0,
            selection_timeout_s: row.get::<i64, _>("selection_timeout_s"),
        },
        None => TicketCategorySettings {
            enabled: false,
            selection_timeout_s: 300,
        },
    })
}

pub async fn update_category_settings(
    enabled: bool,
    selection_timeout_s: i64,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    sqlx::query(
        r#"
        INSERT INTO ticket_category_settings (id, enabled, selection_timeout_s)
        VALUES (1, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            enabled = excluded.enabled,
            selection_timeout_s = excluded.selection_timeout_s
        "#,
    )
    .bind(enabled as i64)
    .bind(selection_timeout_s)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to update category settings: {e:?}");
        validation_failed("Failed to update category settings")
    })?;

    Ok(())
}

fn row_to_category(row: sqlx::sqlite::SqliteRow) -> TicketCategory {
    TicketCategory {
        id: row.get::<String, _>("id"),
        name: row.get::<String, _>("name"),
        description: row.get::<Option<String>, _>("description"),
        emoji: row.get::<Option<String>, _>("emoji"),
        discord_category_id: row.get::<String, _>("discord_category_id"),
        position: row.get::<i64, _>("position"),
        enabled: row.get::<i64, _>("enabled") != 0,
        created_at: row.get::<i64, _>("created_at"),
        updated_at: row.get::<i64, _>("updated_at"),
    }
}

pub async fn list_all_categories(pool: &SqlitePool) -> ModmailResult<Vec<TicketCategory>> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, description, emoji, discord_category_id,
               position, enabled, created_at, updated_at
        FROM ticket_categories
        ORDER BY position ASC, created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to list categories: {e:?}");
        validation_failed("Failed to list categories")
    })?;

    Ok(rows.into_iter().map(row_to_category).collect())
}

pub async fn list_enabled_categories(pool: &SqlitePool) -> ModmailResult<Vec<TicketCategory>> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, description, emoji, discord_category_id,
               position, enabled, created_at, updated_at
        FROM ticket_categories
        WHERE enabled = 1
        ORDER BY position ASC, created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to list enabled categories: {e:?}");
        validation_failed("Failed to list enabled categories")
    })?;

    Ok(rows.into_iter().map(row_to_category).collect())
}

pub async fn count_enabled_categories(pool: &SqlitePool) -> ModmailResult<i64> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ticket_categories WHERE enabled = 1")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to count enabled categories: {e:?}");
            validation_failed("Failed to count enabled categories")
        })?;
    Ok(count)
}

pub async fn get_category_by_id(
    id: &str,
    pool: &SqlitePool,
) -> ModmailResult<Option<TicketCategory>> {
    let row = sqlx::query(
        r#"
        SELECT id, name, description, emoji, discord_category_id,
               position, enabled, created_at, updated_at
        FROM ticket_categories
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch category: {e:?}");
        validation_failed("Failed to fetch category")
    })?;

    Ok(row.map(row_to_category))
}

pub async fn get_category_by_name(
    name: &str,
    pool: &SqlitePool,
) -> ModmailResult<Option<TicketCategory>> {
    let row = sqlx::query(
        r#"
        SELECT id, name, description, emoji, discord_category_id,
               position, enabled, created_at, updated_at
        FROM ticket_categories
        WHERE name = ?
        LIMIT 1
        "#,
    )
    .bind(name)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch category by name: {e:?}");
        validation_failed("Failed to fetch category by name")
    })?;

    Ok(row.map(row_to_category))
}

pub async fn create_category(
    name: &str,
    description: Option<&str>,
    emoji: Option<&str>,
    discord_category_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<TicketCategory> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let max_position: Option<i64> =
        sqlx::query_scalar("SELECT MAX(position) FROM ticket_categories")
            .fetch_one(pool)
            .await
            .map_err(|e| {
                eprintln!("Failed to compute category position: {e:?}");
                validation_failed("Failed to compute category position")
            })?;
    let position = max_position.unwrap_or(-1) + 1;

    sqlx::query(
        r#"
        INSERT INTO ticket_categories
            (id, name, description, emoji, discord_category_id, position, enabled, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(name)
    .bind(description)
    .bind(emoji)
    .bind(discord_category_id)
    .bind(position)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create category: {e:?}");
        validation_failed("Failed to create category")
    })?;

    Ok(TicketCategory {
        id,
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        emoji: emoji.map(|s| s.to_string()),
        discord_category_id: discord_category_id.to_string(),
        position,
        enabled: true,
        created_at: now,
        updated_at: now,
    })
}

pub async fn update_category(
    id: &str,
    name: Option<&str>,
    description: Option<Option<&str>>,
    emoji: Option<Option<&str>>,
    discord_category_id: Option<&str>,
    position: Option<i64>,
    enabled: Option<bool>,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    let existing = get_category_by_id(id, pool)
        .await?
        .ok_or_else(|| validation_failed("Category not found"))?;

    let new_name = name.unwrap_or(&existing.name);
    let new_description = match description {
        Some(opt) => opt.map(|s| s.to_string()),
        None => existing.description.clone(),
    };
    let new_emoji = match emoji {
        Some(opt) => opt.map(|s| s.to_string()),
        None => existing.emoji.clone(),
    };
    let new_discord_category_id = discord_category_id.unwrap_or(&existing.discord_category_id);
    let new_position = position.unwrap_or(existing.position);
    let new_enabled = enabled.unwrap_or(existing.enabled);
    let now = Utc::now().timestamp();

    sqlx::query(
        r#"
        UPDATE ticket_categories
        SET name = ?, description = ?, emoji = ?, discord_category_id = ?,
            position = ?, enabled = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(new_name)
    .bind(new_description)
    .bind(new_emoji)
    .bind(new_discord_category_id)
    .bind(new_position)
    .bind(new_enabled as i64)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to update category: {e:?}");
        validation_failed("Failed to update category")
    })?;

    Ok(())
}

pub async fn delete_category(id: &str, pool: &SqlitePool) -> ModmailResult<bool> {
    let res = sqlx::query("DELETE FROM ticket_categories WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to delete category: {e:?}");
            validation_failed("Failed to delete category")
        })?;

    Ok(res.rows_affected() > 0)
}

pub async fn set_thread_category(
    thread_id: &str,
    category_id: Option<&str>,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    sqlx::query("UPDATE threads SET ticket_category_id = ? WHERE id = ?")
        .bind(category_id)
        .bind(thread_id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to set thread category: {e:?}");
            validation_failed("Failed to set thread category")
        })?;
    Ok(())
}

pub async fn upsert_pending_selection(
    user_id: i64,
    prompt_msg_id: &str,
    dm_channel_id: &str,
    started_at: i64,
    expires_at: i64,
    queued_msg_ids: &[String],
    pool: &SqlitePool,
) -> ModmailResult<()> {
    let queued_json = serde_json::to_string(queued_msg_ids)
        .map_err(|_| validation_failed("Failed to serialize queued messages"))?;

    sqlx::query(
        r#"
        INSERT INTO pending_category_selections
            (user_id, prompt_msg_id, dm_channel_id, started_at, expires_at, queued_msg_ids)
        VALUES (?, ?, ?, ?, ?, ?)
        ON CONFLICT(user_id) DO UPDATE SET
            prompt_msg_id = excluded.prompt_msg_id,
            dm_channel_id = excluded.dm_channel_id,
            started_at = excluded.started_at,
            expires_at = excluded.expires_at,
            queued_msg_ids = excluded.queued_msg_ids
        "#,
    )
    .bind(user_id)
    .bind(prompt_msg_id)
    .bind(dm_channel_id)
    .bind(started_at)
    .bind(expires_at)
    .bind(&queued_json)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to upsert pending selection: {e:?}");
        validation_failed("Failed to upsert pending selection")
    })?;

    Ok(())
}

fn row_to_pending(row: sqlx::sqlite::SqliteRow) -> PendingCategorySelection {
    let queued_json: String = row.get::<String, _>("queued_msg_ids");
    let queued_msg_ids: Vec<String> = serde_json::from_str(&queued_json).unwrap_or_default();

    PendingCategorySelection {
        user_id: row.get::<i64, _>("user_id"),
        prompt_msg_id: row.get::<String, _>("prompt_msg_id"),
        dm_channel_id: row.get::<String, _>("dm_channel_id"),
        started_at: row.get::<i64, _>("started_at"),
        expires_at: row.get::<i64, _>("expires_at"),
        queued_msg_ids,
    }
}

pub async fn get_pending_selection(
    user_id: i64,
    pool: &SqlitePool,
) -> ModmailResult<Option<PendingCategorySelection>> {
    let row = sqlx::query(
        r#"
        SELECT user_id, prompt_msg_id, dm_channel_id, started_at, expires_at, queued_msg_ids
        FROM pending_category_selections
        WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch pending selection: {e:?}");
        validation_failed("Failed to fetch pending selection")
    })?;

    Ok(row.map(row_to_pending))
}

pub async fn append_queued_message(
    user_id: i64,
    message_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    if let Some(mut pending) = get_pending_selection(user_id, pool).await? {
        if pending.queued_msg_ids.iter().any(|id| id == message_id) {
            return Ok(());
        }
        pending.queued_msg_ids.push(message_id.to_string());
        let queued_json = serde_json::to_string(&pending.queued_msg_ids)
            .map_err(|_| validation_failed("Failed to serialize queued messages"))?;
        sqlx::query("UPDATE pending_category_selections SET queued_msg_ids = ? WHERE user_id = ?")
            .bind(&queued_json)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| {
                eprintln!("Failed to append queued message: {e:?}");
                validation_failed("Failed to append queued message")
            })?;
    }
    Ok(())
}

pub async fn delete_pending_selection(user_id: i64, pool: &SqlitePool) -> ModmailResult<bool> {
    let res = sqlx::query("DELETE FROM pending_category_selections WHERE user_id = ?")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to delete pending selection: {e:?}");
            validation_failed("Failed to delete pending selection")
        })?;
    Ok(res.rows_affected() > 0)
}

pub async fn list_all_pending_selections(
    pool: &SqlitePool,
) -> ModmailResult<Vec<PendingCategorySelection>> {
    let rows = sqlx::query(
        r#"
        SELECT user_id, prompt_msg_id, dm_channel_id, started_at, expires_at, queued_msg_ids
        FROM pending_category_selections
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to list pending selections: {e:?}");
        validation_failed("Failed to list pending selections")
    })?;

    Ok(rows.into_iter().map(row_to_pending).collect())
}
