use crate::api::{auth_middleware, handle_external_ticket_create};
use crate::types::BotState;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_external_ticket_router(
    bot_state: Arc<Mutex<BotState>>,
) -> Router<Arc<Mutex<BotState>>> {
    let bot_router = Router::new()
        .route("/create", post(handle_external_ticket_create))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    bot_router
}
