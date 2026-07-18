use crate::setup::handlers::{
    handle_setup_restart, handle_setup_save, handle_setup_status, handle_setup_verify,
    handle_validate_channel, handle_validate_guild, handle_validate_oauth2, handle_validate_token,
};
use crate::setup::middleware::require_setup_token;
use crate::setup::state::SharedSetupState;
use axum::Router;
use axum::middleware;
use axum::routing::{get, post};

pub fn create_setup_router(setup_state: SharedSetupState) -> Router {
    let protected = Router::new()
        .route("/api/setup/verify", get(handle_setup_verify))
        .route("/api/setup/validate-token", post(handle_validate_token))
        .route("/api/setup/validate-guild", post(handle_validate_guild))
        .route("/api/setup/validate-channel", post(handle_validate_channel))
        .route("/api/setup/validate-oauth2", post(handle_validate_oauth2))
        .route("/api/setup/save", post(handle_setup_save))
        .route("/api/setup/restart", post(handle_setup_restart))
        .layer(middleware::from_fn_with_state(
            setup_state.clone(),
            require_setup_token,
        ));

    Router::new()
        .route("/api/setup/status", get(handle_setup_status))
        .merge(protected)
        .with_state(setup_state)
}
