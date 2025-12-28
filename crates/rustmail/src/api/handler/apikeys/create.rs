use crate::db::operations::{create_api_key, generate_api_key};
use crate::db::repr::Permission;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: Vec<Permission>,
    pub expires_at: Option<i64>,
}

#[derive(Serialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub id: i64,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

pub async fn create_api_key_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, (StatusCode, String)> {
    if req.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name cannot be empty".to_string()));
    }

    if req.permissions.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "At least one permission is required".to_string(),
        ));
    }

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

    let (plain_key, key_hash) = match generate_api_key() {
        Ok(keys) => keys,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    let api_key = match create_api_key(
        &db_pool,
        key_hash,
        req.name,
        req.permissions,
        req.expires_at,
    )
    .await
    {
        Ok(key) => key,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    Ok(Json(CreateApiKeyResponse {
        api_key: plain_key,
        id: api_key.id,
        name: api_key.name,
        permissions: api_key.permissions,
        created_at: api_key.created_at,
        expires_at: api_key.expires_at,
    }))
}
