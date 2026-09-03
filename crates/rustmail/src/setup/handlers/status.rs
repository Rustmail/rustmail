use crate::setup::state::{SetupStep, SharedSetupState};
use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use rustmail_types::SETUP_TOKEN_HEADER;
use serde::Serialize;
use subtle::ConstantTimeEq;

#[derive(Serialize)]
pub struct SetupStatusResponse {
    pub setup_required: bool,
    pub step: String,
    pub completed: bool,
    pub token_prefill: Option<String>,
    pub panel_url: Option<String>,
    pub api_port: Option<u16>,
}

pub async fn handle_setup_status(
    State(setup_state): State<SharedSetupState>,
    headers: HeaderMap,
) -> Json<SetupStatusResponse> {
    let state = setup_state.lock().await;

    let step = format!("{:?}", state.step).to_lowercase();
    let completed = matches!(state.step, SetupStep::Review);

    let has_valid_setup_token = headers
        .get(SETUP_TOKEN_HEADER)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|token| bool::from(token.as_bytes().ct_eq(state.token.as_bytes())));

    let token_prefill = has_valid_setup_token
        .then(|| std::env::var("RUSTMAIL_BOT_TOKEN").ok())
        .flatten()
        .filter(|t| !t.is_empty());

    Json(SetupStatusResponse {
        setup_required: true,
        step,
        completed,
        token_prefill,
        panel_url: state.panel_url.clone(),
        api_port: state.api_port,
    })
}
