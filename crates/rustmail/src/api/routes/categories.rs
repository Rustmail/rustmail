use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::{delete, get, patch, post, put};
use rustmail_types::api::panel_permissions::PanelPermission;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_categories_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    Router::new()
        .route("/", get(list_categories_handler))
        .route("/", post(create_category_handler))
        .route("/{id}", patch(update_category_handler))
        .route("/{id}", delete(delete_category_handler))
        .route("/settings", get(get_category_settings_handler))
        .route("/settings", put(update_category_settings_handler))
        .route("/{id}/roles", get(list_category_roles_handler))
        .route("/{id}/roles", post(add_category_role_handler))
        .route("/{id}/roles", put(set_category_roles_handler))
        .route("/{id}/roles", delete(clear_category_roles_handler))
        .route(
            "/{id}/roles/{role_id}",
            delete(remove_category_role_handler),
        )
        .layer(axum::middleware::from_fn_with_state(
            bot_state.clone(),
            move |state, jar, req, next| {
                require_panel_permission(state, jar, req, next, PanelPermission::ManageCategories)
            },
        ))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ))
}
