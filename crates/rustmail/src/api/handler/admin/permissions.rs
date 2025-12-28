use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use chrono::Utc;
use rustmail_types::api::panel_permissions::*;
use sqlx::{Row, query};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_list_permissions(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> impl IntoResponse {
    let db_pool = {
        let state_lock = bot_state.lock().await;
        match &state_lock.db_pool {
            Some(pool) => pool.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Database not initialized"})),
                )
                    .into_response();
            }
        }
    };

    let rows = match query("SELECT * FROM panel_permissions ORDER BY granted_at DESC")
        .fetch_all(&db_pool)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Database error: {}", e)})),
            )
                .into_response();
        }
    };

    let mut permissions = Vec::new();
    for row in rows {
        if let (
            Ok(id),
            Ok(subject_type),
            Ok(subject_id),
            Ok(permission),
            Ok(granted_by),
            Ok(granted_at),
        ) = (
            row.try_get::<i64, _>("id"),
            row.try_get::<String, _>("subject_type"),
            row.try_get::<String, _>("subject_id"),
            row.try_get::<String, _>("permission"),
            row.try_get::<String, _>("granted_by"),
            row.try_get::<i64, _>("granted_at"),
        ) {
            if let (Some(st), Some(perm)) = (
                SubjectType::from_str(&subject_type),
                PanelPermission::from_str(&permission),
            ) {
                permissions.push(PanelPermissionEntry {
                    id,
                    subject_type: st,
                    subject_id,
                    permission: perm,
                    granted_by,
                    granted_at,
                });
            }
        }
    }

    (StatusCode::OK, Json(permissions)).into_response()
}

pub async fn handle_grant_permission(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    jar: CookieJar,
    Json(request): Json<GrantPermissionRequest>,
) -> impl IntoResponse {
    let (db_pool, user_id) = {
        let state_lock = bot_state.lock().await;
        let pool = match &state_lock.db_pool {
            Some(p) => p.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Database not initialized"})),
                )
                    .into_response();
            }
        };

        let session_cookie = jar.get("session_id");
        if session_cookie.is_none() {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"})),
            )
                .into_response();
        }

        let session_id = session_cookie.unwrap().value().to_string();
        let uid = get_user_id_from_session(&session_id, &pool).await;
        (pool, uid)
    };

    let subject_type_str = request.subject_type.as_str();
    let permission_str = request.permission.as_str();
    let now = Utc::now().timestamp();

    let result = query(
        "INSERT INTO panel_permissions (subject_type, subject_id, permission, granted_by, granted_at)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT(subject_type, subject_id, permission) DO UPDATE SET granted_by = ?, granted_at = ?"
    )
    .bind(subject_type_str)
    .bind(&request.subject_id)
    .bind(permission_str)
    .bind(&user_id)
    .bind(now)
    .bind(&user_id)
    .bind(now)
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Database error: {}", e)})),
        )
            .into_response(),
    }
}

pub async fn handle_revoke_permission(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path(permission_id): Path<i64>,
) -> impl IntoResponse {
    let db_pool = {
        let state_lock = bot_state.lock().await;
        match &state_lock.db_pool {
            Some(pool) => pool.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Database not initialized"})),
                )
                    .into_response();
            }
        }
    };

    let result = query("DELETE FROM panel_permissions WHERE id = ?")
        .bind(permission_id)
        .execute(&db_pool)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Database error: {}", e)})),
        )
            .into_response(),
    }
}
