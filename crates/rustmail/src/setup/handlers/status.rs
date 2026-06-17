use crate::setup::state::SharedSetupState;
use axum::Json;
use axum::extract::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct SetupStatusResponse {
    pub setup_required: bool,
    pub step: String,
    pub token_prefill: Option<String>,
}

pub async fn handle_setup_status(
    State(setup_state): State<SharedSetupState>,
) -> Json<SetupStatusResponse> {
    let state = setup_state.lock().await;

    let step = format!("{:?}", state.step).to_lowercase();

    let token_prefill = std::env::var("RUSTMAIL_BOT_TOKEN")
        .ok()
        .filter(|t| !t.is_empty());

    Json(SetupStatusResponse {
        setup_required: true,
        step,
        token_prefill,
    })
}
