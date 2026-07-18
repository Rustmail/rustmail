use crate::setup::state::SharedSetupState;
use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct SetupStatusResponse {
    pub setup_required: bool,
    pub step: String,
    pub token_prefill: Option<String>,
}

pub async fn handle_setup_status(
    State(setup_state): State<SharedSetupState>,
    headers: HeaderMap,
) -> Json<SetupStatusResponse> {
    let state = setup_state.lock().await;

    let step = format!("{:?}", state.step).to_lowercase();

    let has_valid_setup_token = headers
        .get("x-setup-token")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|token| token == state.token);

    let token_prefill = has_valid_setup_token
        .then(|| std::env::var("RUSTMAIL_BOT_TOKEN").ok())
        .flatten()
        .filter(|t| !t.is_empty());

    Json(SetupStatusResponse {
        setup_required: true,
        step,
        token_prefill,
    })
}
