use crate::db::operations::revoke_api_key;
use crate::prelude::types::*;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn revoke_api_key_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
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

    match revoke_api_key(&db_pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}
