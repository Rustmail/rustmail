use crate::types::BotState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use rustmail_types::CreateTicket;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_external_ticket_create(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(update): Json<CreateTicket>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let current_config = {
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
        "Discor ID : {:?} | Api key: {:?}",
        update.discord_id, update.api_key
    );

    Ok(Json(serde_json::json!({"status": "ticket created"})))
}
