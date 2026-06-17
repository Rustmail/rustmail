use crate::setup::state::SharedSetupState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::time::Duration;

pub async fn handle_setup_restart(
    State(setup_state): State<SharedSetupState>,
) -> impl IntoResponse {
    let tx = {
        let mut state = setup_state.lock().await;
        state.shutdown_tx.take()
    };

    // Spawn a task to send the shutdown signal after a short delay
    // This allows the HTTP response to be sent to the frontend first
    if let Some(tx) = tx {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let _ = tx.send(()).await;
        });
    }

    (StatusCode::OK, Json(serde_json::json!({ "success": true })))
}
