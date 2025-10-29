use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_auth_router() -> Router<Arc<Mutex<BotState>>> {
    let auth_router = Router::new()
        .route("/login", get(handle_login))
        .route("/callback", get(handle_callback))
        .route("/logout", get(handle_logout));

    auth_router
}
