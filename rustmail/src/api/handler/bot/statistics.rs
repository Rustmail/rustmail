use crate::prelude::db::*;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Deserialize)]
pub struct StatisticsQuery {
    #[serde(default = "default_days")]
    pub days: i64,
}

fn default_days() -> i64 {
    30
}

pub async fn handle_statistics(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Query(query): Query<StatisticsQuery>,
) -> impl IntoResponse {
    let days = match query.days {
        7 | 30 | 90 => query.days,
        _ => 30,
    };

    let state_lock = bot_state.lock().await;

    let pool = match &state_lock.db_pool {
        Some(p) => p.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "Database not available"})),
            );
        }
    };

    drop(state_lock);

    match get_statistics(&pool, days).await {
        Ok(stats) => (StatusCode::OK, Json(serde_json::to_value(stats).unwrap())),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to fetch statistics: {}", e)})),
        ),
    }
}
