use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_user_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let user_router = Router::new()
        .route("/avatar", get(handle_get_user_avatar))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    user_router
}
