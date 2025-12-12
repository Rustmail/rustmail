use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::{get, post, put};
use rustmail_types::api::panel_permissions::PanelPermission;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_bot_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let manage_bot_routes = Router::new()
        .route("/start", post(handle_start_bot))
        .route("/stop", post(handle_stop_bot))
        .route("/restart", post(handle_restart_bot))
        .route("/presence", post(handle_set_presence))
        .layer(axum::middleware::from_fn_with_state(
            bot_state.clone(),
            move |state, jar, req, next| {
                require_panel_permission(state, jar, req, next, PanelPermission::ManageBot)
            },
        ));

    let manage_config_routes = Router::new()
        .route("/config", put(handle_update_config))
        .layer(axum::middleware::from_fn_with_state(
            bot_state.clone(),
            move |state, jar, req, next| {
                require_panel_permission(state, jar, req, next, PanelPermission::ManageConfig)
            },
        ));

    let view_routes = Router::new()
        .route("/status", get(handle_status_bot))
        .route("/tickets", get(handle_tickets_bot))
        .route("/config", get(handle_get_config))
        .layer(axum::middleware::from_fn_with_state(
            bot_state.clone(),
            move |state, jar, req, next| {
                require_panel_permission(state, jar, req, next, PanelPermission::ViewPanel)
            },
        ));

    let bot_router = Router::new()
        .merge(manage_bot_routes)
        .merge(manage_config_routes)
        .merge(view_routes)
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    bot_router
}
