use axum::Json;
use serde_json::{Value, json};

pub async fn handle_setup_verify() -> Json<Value> {
    Json(json!({ "success": true }))
}
