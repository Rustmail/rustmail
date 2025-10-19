use crate::BotState;
use crate::api::handler::bot::start::handle_start_bot;
use crate::api::handler::bot::stop::handle_stop_bot;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::api::handler::bot::restart::handle_restart_bot;
use crate::api::handler::bot::status::handle_status_bot;
use crate::api::handler::bot::tickets::{handle_tickets_bot};
use crate::api::middleware::auth::auth_middleware;

pub fn create_bot_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let bot_router = Router::new()
        .route("/start", post(handle_start_bot))
        .route("/stop", post(handle_stop_bot))
        .route("/restart", post(handle_restart_bot))
        .route("/status", get(handle_status_bot))
        .route("/tickets", get(handle_tickets_bot))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    bot_router
}
