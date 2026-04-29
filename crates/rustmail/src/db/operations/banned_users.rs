use crate::db::repr::{BannedUser, TrackedMember};
use crate::prelude::errors::*;
use sqlx::{Row, SqlitePool};

fn row_to_tracked_member(row: sqlx::sqlite::SqliteRow) -> TrackedMember {
    let roles_json: String = row.get("roles");
    let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

    TrackedMember {
        guild_id: row.get("guild_id"),
        user_id: row.get("user_id"),
        username: row.get("username"),
        global_name: row.get("global_name"),
        nickname: row.get("nickname"),
        avatar_url: row.get("avatar_url"),
        roles,
        joined_at: row.get("joined_at"),
        first_seen_at: row.get("first_seen_at"),
        last_seen_at: row.get("last_seen_at"),
    }
}

fn row_to_banned_user(row: sqlx::sqlite::SqliteRow) -> BannedUser {
    let roles_json: String = row.get("roles");
    let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();
    let roles_unknown: i64 = row.get("roles_unknown");

    BannedUser {
        guild_id: row.get("guild_id"),
        user_id: row.get("user_id"),
        username: row.get("username"),
        global_name: row.get("global_name"),
        nickname: row.get("nickname"),
        avatar_url: row.get("avatar_url"),
        roles,
        joined_at: row.get("joined_at"),
        banned_at: row.get("banned_at"),
        banned_by: row.get("banned_by"),
        ban_reason: row.get("ban_reason"),
        roles_unknown: roles_unknown != 0,
    }
}

pub async fn upsert_tracked_member(member: &TrackedMember, pool: &SqlitePool) -> ModmailResult<()> {
    let roles_json = serde_json::to_string(&member.roles)
        .map_err(|_| validation_failed("Failed to serialize member roles"))?;

    sqlx::query(
        r#"
        INSERT INTO tracked_members
            (guild_id, user_id, username, global_name, nickname, avatar_url, roles,
             joined_at, first_seen_at, last_seen_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(guild_id, user_id) DO UPDATE SET
            username = excluded.username,
            global_name = excluded.global_name,
            nickname = excluded.nickname,
            avatar_url = excluded.avatar_url,
            roles = excluded.roles,
            joined_at = COALESCE(excluded.joined_at, tracked_members.joined_at),
            last_seen_at = excluded.last_seen_at
        "#,
    )
    .bind(&member.guild_id)
    .bind(&member.user_id)
    .bind(&member.username)
    .bind(&member.global_name)
    .bind(&member.nickname)
    .bind(&member.avatar_url)
    .bind(&roles_json)
    .bind(member.joined_at)
    .bind(member.first_seen_at)
    .bind(member.last_seen_at)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to upsert tracked member: {e:?}");
        validation_failed("Failed to upsert tracked member")
    })?;

    Ok(())
}

pub async fn bulk_upsert_tracked_members(
    members: &[TrackedMember],
    pool: &SqlitePool,
) -> ModmailResult<()> {
    if members.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to begin tracked members transaction: {e:?}");
        validation_failed("Failed to begin tracked members transaction")
    })?;

    for member in members {
        let roles_json = serde_json::to_string(&member.roles)
            .map_err(|_| validation_failed("Failed to serialize member roles"))?;

        sqlx::query(
            r#"
            INSERT INTO tracked_members
                (guild_id, user_id, username, global_name, nickname, avatar_url, roles,
                 joined_at, first_seen_at, last_seen_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(guild_id, user_id) DO UPDATE SET
                username = excluded.username,
                global_name = excluded.global_name,
                nickname = excluded.nickname,
                avatar_url = excluded.avatar_url,
                roles = excluded.roles,
                joined_at = COALESCE(excluded.joined_at, tracked_members.joined_at),
                last_seen_at = excluded.last_seen_at
            "#,
        )
        .bind(&member.guild_id)
        .bind(&member.user_id)
        .bind(&member.username)
        .bind(&member.global_name)
        .bind(&member.nickname)
        .bind(&member.avatar_url)
        .bind(&roles_json)
        .bind(member.joined_at)
        .bind(member.first_seen_at)
        .bind(member.last_seen_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Failed to upsert tracked member in bulk: {e:?}");
            validation_failed("Failed to upsert tracked member in bulk")
        })?;
    }

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit tracked members transaction: {e:?}");
        validation_failed("Failed to commit tracked members transaction")
    })?;

    Ok(())
}

pub async fn get_tracked_member(
    guild_id: &str,
    user_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<Option<TrackedMember>> {
    let row = sqlx::query(
        r#"
        SELECT guild_id, user_id, username, global_name, nickname, avatar_url,
               roles, joined_at, first_seen_at, last_seen_at
        FROM tracked_members
        WHERE guild_id = ? AND user_id = ?
        "#,
    )
    .bind(guild_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch tracked member: {e:?}");
        validation_failed("Failed to fetch tracked member")
    })?;

    Ok(row.map(row_to_tracked_member))
}

pub async fn delete_tracked_member(
    guild_id: &str,
    user_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    sqlx::query("DELETE FROM tracked_members WHERE guild_id = ? AND user_id = ?")
        .bind(guild_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to delete tracked member: {e:?}");
            validation_failed("Failed to delete tracked member")
        })?;
    Ok(())
}

pub async fn save_banned_user(user: &BannedUser, pool: &SqlitePool) -> ModmailResult<()> {
    let roles_json = serde_json::to_string(&user.roles)
        .map_err(|_| validation_failed("Failed to serialize banned user roles"))?;

    sqlx::query(
        r#"
        INSERT INTO banned_users
            (guild_id, user_id, username, global_name, nickname, avatar_url, roles,
             joined_at, banned_at, banned_by, ban_reason, roles_unknown)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(guild_id, user_id) DO UPDATE SET
            username = excluded.username,
            global_name = excluded.global_name,
            nickname = excluded.nickname,
            avatar_url = excluded.avatar_url,
            roles = excluded.roles,
            joined_at = COALESCE(excluded.joined_at, banned_users.joined_at),
            banned_at = excluded.banned_at,
            banned_by = excluded.banned_by,
            ban_reason = excluded.ban_reason,
            roles_unknown = excluded.roles_unknown
        "#,
    )
    .bind(&user.guild_id)
    .bind(&user.user_id)
    .bind(&user.username)
    .bind(&user.global_name)
    .bind(&user.nickname)
    .bind(&user.avatar_url)
    .bind(&roles_json)
    .bind(user.joined_at)
    .bind(user.banned_at)
    .bind(&user.banned_by)
    .bind(&user.ban_reason)
    .bind(user.roles_unknown as i64)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to save banned user: {e:?}");
        validation_failed("Failed to save banned user")
    })?;

    Ok(())
}

pub async fn get_banned_user_by_id(
    guild_id: &str,
    user_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<Option<BannedUser>> {
    let row = sqlx::query(
        r#"
        SELECT guild_id, user_id, username, global_name, nickname, avatar_url,
               roles, joined_at, banned_at, banned_by, ban_reason, roles_unknown
        FROM banned_users
        WHERE guild_id = ? AND user_id = ?
        "#,
    )
    .bind(guild_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch banned user: {e:?}");
        validation_failed("Failed to fetch banned user")
    })?;

    Ok(row.map(row_to_banned_user))
}

pub async fn find_banned_users_by_username(
    guild_id: &str,
    username: &str,
    pool: &SqlitePool,
) -> ModmailResult<Vec<BannedUser>> {
    let rows = sqlx::query(
        r#"
        SELECT guild_id, user_id, username, global_name, nickname, avatar_url,
               roles, joined_at, banned_at, banned_by, ban_reason, roles_unknown
        FROM banned_users
        WHERE guild_id = ?
          AND (username = ? COLLATE NOCASE
               OR global_name = ? COLLATE NOCASE
               OR nickname = ? COLLATE NOCASE)
        ORDER BY banned_at DESC
        LIMIT 10
        "#,
    )
    .bind(guild_id)
    .bind(username)
    .bind(username)
    .bind(username)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to search banned users: {e:?}");
        validation_failed("Failed to search banned users")
    })?;

    Ok(rows.into_iter().map(row_to_banned_user).collect())
}

pub async fn get_all_banned_users(
    guild_id: &str,
    pool: &SqlitePool,
) -> ModmailResult<Vec<BannedUser>> {
    let rows = sqlx::query(
        r#"
        SELECT guild_id, user_id, username, global_name, nickname, avatar_url,
               roles, joined_at, banned_at, banned_by, ban_reason, roles_unknown
        FROM banned_users
        WHERE guild_id = ?
        ORDER BY banned_at DESC
        "#,
    )
    .bind(guild_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch all banned users: {e:?}");
        validation_failed("Failed to fetch all banned users")
    })?;

    Ok(rows.into_iter().map(row_to_banned_user).collect())
}
