use crate::modules::update_checker::{
    CURRENT_VERSION, LAST_UPDATE_CHECK_KEY, LATEST_KNOWN_VERSION_KEY, LATEST_RELEASE_URL_KEY,
    is_newer,
};
use crate::prelude::db::*;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use rustmail_types::VersionInfo;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_get_version(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> Result<Json<VersionInfo>, StatusCode> {
    let (pool, check_enabled) = {
        let state = bot_state.lock().await;

        let check_enabled = state
            .config
            .as_ref()
            .map(|c| c.updates.enabled)
            .unwrap_or(false);

        (state.db_pool.clone(), check_enabled)
    };

    let Some(pool) = pool else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let latest = get_system_metadata(LATEST_KNOWN_VERSION_KEY, &pool)
        .await
        .ok()
        .flatten();
    let release_url = get_system_metadata(LATEST_RELEASE_URL_KEY, &pool)
        .await
        .ok()
        .flatten();
    let last_checked = get_system_metadata(LAST_UPDATE_CHECK_KEY, &pool)
        .await
        .ok()
        .flatten();

    let update_available = latest
        .as_deref()
        .map(|latest| is_newer(latest, CURRENT_VERSION))
        .unwrap_or(false);

    Ok(Json(VersionInfo {
        current: CURRENT_VERSION.to_string(),
        latest,
        update_available,
        release_url,
        check_enabled,
        last_checked,
    }))
}
