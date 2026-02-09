use crate::db::repr::{ApiKey, Permission};
use chrono::Utc;
use hex;
use rand::RngExt;
use sha2::{Digest, Sha256};
use sqlx::{Row, SqlitePool};

const API_KEY_PREFIX: &str = "rustmail";
const API_KEY_LENGTH: usize = 32;

pub fn generate_api_key() -> Result<(String, String), String> {
    let mut rng = rand::rng();
    let random_bytes: Vec<u8> = (0..API_KEY_LENGTH).map(|_| rng.random::<u8>()).collect();
    let random_part = hex::encode(random_bytes);

    let plain_key = format!("{}_{}", API_KEY_PREFIX, random_part);
    let key_hash = hash_api_key(&plain_key);

    Ok((plain_key, key_hash))
}

pub fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn _validate_api_key(key: &str, stored_hash: &str) -> bool {
    let key_hash = hash_api_key(key);
    use subtle::ConstantTimeEq;
    key_hash
        .as_bytes()
        .ct_eq(stored_hash.as_bytes())
        .unwrap_u8()
        == 1
}

pub async fn create_api_key(
    pool: &SqlitePool,
    key_hash: String,
    name: String,
    permissions: Vec<Permission>,
    expires_at: Option<i64>,
) -> Result<ApiKey, String> {
    let created_at = Utc::now().timestamp();
    let permissions_json = serde_json::to_string(&permissions)
        .map_err(|e| format!("Failed to serialize permissions: {}", e))?;

    let result = sqlx::query(
        "INSERT INTO api_keys (key_hash, name, permissions, created_at, expires_at, is_active)
         VALUES (?, ?, ?, ?, ?, 1)",
    )
    .bind(&key_hash)
    .bind(&name)
    .bind(&permissions_json)
    .bind(created_at)
    .bind(expires_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to insert API key: {}", e))?;

    let id = result.last_insert_rowid();

    Ok(ApiKey {
        id,
        key_hash,
        name,
        permissions,
        created_at,
        expires_at,
        last_used_at: None,
        is_active: true,
    })
}

pub async fn get_api_key_by_hash(
    pool: &SqlitePool,
    key_hash: &str,
) -> Result<Option<ApiKey>, String> {
    let row = sqlx::query(
        "SELECT id, key_hash, name, permissions, created_at, expires_at, last_used_at, is_active
         FROM api_keys WHERE key_hash = ? AND is_active = 1",
    )
    .bind(key_hash)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch API key: {}", e))?;

    match row {
        Some(row) => {
            let permissions_json: String = row.get("permissions");
            let permissions: Vec<Permission> = serde_json::from_str(&permissions_json)
                .map_err(|e| format!("Failed to deserialize permissions: {}", e))?;

            Ok(Some(ApiKey {
                id: row.get("id"),
                key_hash: row.get("key_hash"),
                name: row.get("name"),
                permissions,
                created_at: row.get("created_at"),
                expires_at: row.get("expires_at"),
                last_used_at: row.get("last_used_at"),
                is_active: row.get::<i64, _>("is_active") == 1,
            }))
        }
        None => Ok(None),
    }
}

pub async fn list_api_keys(pool: &SqlitePool) -> Result<Vec<ApiKey>, String> {
    let rows = sqlx::query(
        "SELECT id, key_hash, name, permissions, created_at, expires_at, last_used_at, is_active
         FROM api_keys ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch API keys: {}", e))?;

    let mut keys = Vec::new();
    for row in rows {
        let permissions_json: String = row.get("permissions");
        let permissions: Vec<Permission> = serde_json::from_str(&permissions_json)
            .map_err(|e| format!("Failed to deserialize permissions: {}", e))?;

        keys.push(ApiKey {
            id: row.get("id"),
            key_hash: row.get("key_hash"),
            name: row.get("name"),
            permissions,
            created_at: row.get("created_at"),
            expires_at: row.get("expires_at"),
            last_used_at: row.get("last_used_at"),
            is_active: row.get::<i64, _>("is_active") == 1,
        });
    }

    Ok(keys)
}

pub async fn revoke_api_key(pool: &SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE api_keys SET is_active = 0 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to revoke API key: {}", e))?;

    Ok(())
}

pub async fn delete_api_key(pool: &SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM api_keys WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete API key: {}", e))?;

    Ok(())
}

pub async fn update_last_used(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let now = Utc::now().timestamp();

    sqlx::query("UPDATE api_keys SET last_used_at = ? WHERE id = ?")
        .bind(now)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update last_used_at: {}", e))?;

    Ok(())
}

impl ApiKey {
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = Utc::now().timestamp();
            now > expires_at
        } else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired()
    }
}
