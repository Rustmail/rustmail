use crate::setup::state::SharedSetupState;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub async fn require_setup_token(
    State(setup_state): State<SharedSetupState>,
    req: Request,
    next: Next,
) -> Response {
    let expected_token = {
        let state = setup_state.lock().await;
        state.token.clone()
    };

    let provided_token = req
        .headers()
        .get("x-setup-token")
        .and_then(|value| value.to_str().ok());

    match provided_token {
        Some(token) if token == expected_token => next.run(req).await,
        _ => (
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "success": false,
                "error": "Invalid or missing setup token"
            })),
        )
            .into_response(),
    }
}
