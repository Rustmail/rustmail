use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::{get, post, put};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_bot_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let bot_router = Router::new()
        .route("/start", post(handle_start_bot))
        .route("/stop", post(handle_stop_bot))
        .route("/restart", post(handle_restart_bot))
        .route("/status", get(handle_status_bot))
        .route("/tickets", get(handle_tickets_bot))
        .route("/config", get(handle_get_config))
        .route("/config", put(handle_update_config))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    bot_router
}
