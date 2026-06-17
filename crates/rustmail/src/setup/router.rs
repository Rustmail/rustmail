use crate::setup::handlers::{
    handle_setup_save, handle_setup_status, handle_validate_channel, handle_validate_guild,
    handle_validate_token,
};
use crate::setup::state::SharedSetupState;
use axum::Router;
use axum::routing::{get, post};

pub fn create_setup_router(setup_state: SharedSetupState) -> Router {
    Router::new()
        .route("/api/setup/status", get(handle_setup_status))
        .route("/api/setup/validate-token", post(handle_validate_token))
        .route("/api/setup/validate-guild", post(handle_validate_guild))
        .route("/api/setup/validate-channel", post(handle_validate_channel))
        .route("/api/setup/save", post(handle_setup_save))
        .with_state(setup_state)
}
