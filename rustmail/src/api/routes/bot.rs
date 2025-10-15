use crate::api::handler::bot::start::handle_start_bot;
use crate::api::handler::bot::stop::handle_stop_bot;
use crate::BotState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_bot_router() -> Router<Arc<Mutex<BotState>>> {
    let bot_router = Router::new()
        .route("/start", post(handle_start_bot))
        .route("/stop", post(handle_stop_bot));

    bot_router
}
