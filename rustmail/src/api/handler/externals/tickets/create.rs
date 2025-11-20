use crate::db::repr::{ApiKey, Permission};
use crate::prelude::api::*;
use crate::types::BotState;
use axum::Json;
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use rustmail_types::CreateTicket;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_external_ticket_create(
    Extension(api_key): Extension<ApiKey>,
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(update): Json<CreateTicket>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    check_permission(&api_key, Permission::CreateTicket)
        .map_err(|e| (StatusCode::FORBIDDEN, format!("{:?}", e)))?;

    let _current_config = {
        let state = bot_state.lock().await;
        match &state.config {
            Some(c) => c.clone(),
            None => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Configuration not loaded".to_string(),
                ));
            }
        }
    };

    println!(
        "API Key #{} creating ticket for Discord ID: {:?}",
        api_key.id, update.discord_id
    );

    Ok(Json(serde_json::json!({
        "status": "ticket created",
        "message": "Ticket creation endpoint - implementation pending"
    })))
}
