use crate::db::operations::list_api_keys;
use crate::db::repr::{ApiKey, Permission};
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct ApiKeyListItem {
    pub id: i64,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub last_used_at: Option<i64>,
    pub is_active: bool,
    pub key_preview: String,
}

impl From<ApiKey> for ApiKeyListItem {
    fn from(key: ApiKey) -> Self {
        let key_preview = if key.key_hash.len() > 12 {
            format!("{}...", &key.key_hash[..12])
        } else {
            key.key_hash.clone()
        };

        ApiKeyListItem {
            id: key.id,
            name: key.name,
            permissions: key.permissions,
            created_at: key.created_at,
            expires_at: key.expires_at,
            last_used_at: key.last_used_at,
            is_active: key.is_active,
            key_preview,
        }
    }
}

pub async fn list_api_keys_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> Result<Json<Vec<ApiKeyListItem>>, (StatusCode, String)> {
    let db_pool = {
        let state_lock = bot_state.lock().await;
        match &state_lock.db_pool {
            Some(pool) => pool.clone(),
            None => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database not initialized".to_string(),
                ));
            }
        }
    };

    let api_keys = match list_api_keys(&db_pool).await {
        Ok(keys) => keys,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    let response: Vec<ApiKeyListItem> = api_keys.into_iter().map(|k| k.into()).collect();

    Ok(Json(response))
}
