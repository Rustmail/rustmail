use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_panel_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let panel_router = Router::new()
        .route("/check", get(handle_panel_check))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    panel_router
}
