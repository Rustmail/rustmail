use crate::db::operations::ticket_categories::CATEGORY_BUTTON_HARD_LIMIT;
use crate::db::operations::{
    add_category_role, count_enabled_categories, create_category, delete_category,
    get_category_by_id, get_category_by_name, get_category_settings, list_all_categories,
    list_category_role_ids, remove_category_role, set_category_roles, update_category,
    update_category_settings,
};
use crate::db::repr::{TicketCategory, TicketCategorySettings};
use crate::prelude::types::*;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn pool(bot_state: &Arc<Mutex<BotState>>) -> Result<SqlitePool, (StatusCode, String)> {
    let state_lock = bot_state.lock().await;
    match &state_lock.db_pool {
        Some(p) => Ok(p.clone()),
        None => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database not initialized".to_string(),
        )),
    }
}

fn internal(e: impl ToString) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

#[derive(Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub discord_category_id: String,
    pub position: i64,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<TicketCategory> for CategoryDto {
    fn from(c: TicketCategory) -> Self {
        Self {
            id: c.id,
            name: c.name,
            description: c.description,
            emoji: c.emoji,
            discord_category_id: c.discord_category_id,
            position: c.position,
            enabled: c.enabled,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

pub async fn list_categories_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> Result<Json<Vec<CategoryDto>>, (StatusCode, String)> {
    let p = pool(&bot_state).await?;
    let cats = list_all_categories(&p).await.map_err(internal)?;
    Ok(Json(cats.into_iter().map(CategoryDto::from).collect()))
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub discord_category_id: String,
}

pub async fn create_category_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<Json<CategoryDto>, (StatusCode, String)> {
    let name = req.name.trim();
    if name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name required".to_string()));
    }
    if req.discord_category_id.parse::<u64>().is_err() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid discord_category_id".to_string(),
        ));
    }
    let p = pool(&bot_state).await?;

    let enabled_count = count_enabled_categories(&p).await.map_err(internal)?;
    if enabled_count as usize >= CATEGORY_BUTTON_HARD_LIMIT {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Maximum {} enabled categories", CATEGORY_BUTTON_HARD_LIMIT),
        ));
    }

    if get_category_by_name(name, &p)
        .await
        .map_err(internal)?
        .is_some()
    {
        return Err((
            StatusCode::CONFLICT,
            "Category with this name already exists".to_string(),
        ));
    }

    let created = create_category(
        name,
        req.description.as_deref(),
        req.emoji.as_deref(),
        &req.discord_category_id,
        &p,
    )
    .await
    .map_err(internal)?;
    Ok(Json(created.into()))
}

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub emoji: Option<Option<String>>,
    pub discord_category_id: Option<String>,
    pub position: Option<i64>,
    pub enabled: Option<bool>,
}

pub async fn update_category_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<Json<CategoryDto>, (StatusCode, String)> {
    let p = pool(&bot_state).await?;

    let existing = get_category_by_id(&id, &p)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "Category not found".to_string()))?;

    let trimmed_name = req.name.as_deref().map(str::trim);
    if let Some(name) = trimmed_name {
        if name.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Name required".to_string()));
        }

        if let Some(conflict) = get_category_by_name(name, &p).await.map_err(internal)? {
            if conflict.id != existing.id {
                return Err((
                    StatusCode::CONFLICT,
                    "Category with this name already exists".to_string(),
                ));
            }
        }
    }

    if let Some(true) = req.enabled {
        if !existing.enabled {
            let enabled_count = count_enabled_categories(&p).await.map_err(internal)?;
            if enabled_count as usize >= CATEGORY_BUTTON_HARD_LIMIT {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Maximum {} enabled categories", CATEGORY_BUTTON_HARD_LIMIT),
                ));
            }
        }
    }

    if let Some(ref did) = req.discord_category_id {
        if did.parse::<u64>().is_err() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid discord_category_id".to_string(),
            ));
        }
    }

    update_category(
        &id,
        trimmed_name,
        req.description.as_ref().map(|o| o.as_deref()),
        req.emoji.as_ref().map(|o| o.as_deref()),
        req.discord_category_id.as_deref(),
        req.position,
        req.enabled,
        &p,
    )
    .await
    .map_err(internal)?;

    let updated = get_category_by_id(&id, &p)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "Category not found".to_string()))?;
    Ok(Json(updated.into()))
}

pub async fn delete_category_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let p = pool(&bot_state).await?;
    let existed = delete_category(&id, &p).await.map_err(internal)?;
    if existed {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Category not found".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct CategorySettingsDto {
    pub enabled: bool,
    pub selection_timeout_s: i64,
}

impl From<TicketCategorySettings> for CategorySettingsDto {
    fn from(s: TicketCategorySettings) -> Self {
        Self {
            enabled: s.enabled,
            selection_timeout_s: s.selection_timeout_s,
        }
    }
}

pub async fn get_category_settings_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> Result<Json<CategorySettingsDto>, (StatusCode, String)> {
    let p = pool(&bot_state).await?;
    let s = get_category_settings(&p).await.map_err(internal)?;
    Ok(Json(s.into()))
}

pub async fn update_category_settings_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(req): Json<CategorySettingsDto>,
) -> Result<Json<CategorySettingsDto>, (StatusCode, String)> {
    if req.selection_timeout_s < 30 {
        return Err((
            StatusCode::BAD_REQUEST,
            "selection_timeout_s must be >= 30".to_string(),
        ));
    }
    let p = pool(&bot_state).await?;
    update_category_settings(req.enabled, req.selection_timeout_s, &p)
        .await
        .map_err(internal)?;
    let s = get_category_settings(&p).await.map_err(internal)?;
    Ok(Json(s.into()))
}

#[derive(Serialize, Deserialize)]
pub struct CategoryRolesDto {
    pub role_ids: Vec<String>,
}

async fn ensure_category_exists(
    p: &SqlitePool,
    id: &str,
) -> Result<TicketCategory, (StatusCode, String)> {
    get_category_by_id(id, p)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "Category not found".to_string()))
}

fn validate_role_id(raw: &str) -> Result<String, (StatusCode, String)> {
    let trimmed = raw.trim();
    trimmed
        .parse::<u64>()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid role_id".to_string()))?;
    Ok(trimmed.to_string())
}

pub async fn list_category_roles_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path(id): Path<String>,
) -> Result<Json<CategoryRolesDto>, (StatusCode, String)> {
    let p = pool(&bot_state).await?;
    let _ = ensure_category_exists(&p, &id).await?;
    let role_ids = list_category_role_ids(&id, &p).await.map_err(internal)?;
    Ok(Json(CategoryRolesDto { role_ids }))
}

#[derive(Deserialize)]
pub struct CategoryRoleRequest {
    pub role_id: String,
}

pub async fn add_category_role_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path(id): Path<String>,
    Json(req): Json<CategoryRoleRequest>,
) -> Result<Json<CategoryRolesDto>, (StatusCode, String)> {
    let p = pool(&bot_state).await?;
    let _ = ensure_category_exists(&p, &id).await?;
    let role_id = validate_role_id(&req.role_id)?;
    add_category_role(&id, &role_id, &p)
        .await
        .map_err(internal)?;
    let role_ids = list_category_role_ids(&id, &p).await.map_err(internal)?;
    Ok(Json(CategoryRolesDto { role_ids }))
}

pub async fn set_category_roles_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path(id): Path<String>,
    Json(req): Json<CategoryRolesDto>,
) -> Result<Json<CategoryRolesDto>, (StatusCode, String)> {
    let p = pool(&bot_state).await?;
    let _ = ensure_category_exists(&p, &id).await?;
    let mut validated: Vec<String> = Vec::with_capacity(req.role_ids.len());
    for raw in &req.role_ids {
        validated.push(validate_role_id(raw)?);
    }
    set_category_roles(&id, &validated, &p)
        .await
        .map_err(internal)?;
    let role_ids = list_category_role_ids(&id, &p).await.map_err(internal)?;
    Ok(Json(CategoryRolesDto { role_ids }))
}

pub async fn remove_category_role_handler(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Path((id, role_id)): Path<(String, String)>,
) -> Result<Json<CategoryRolesDto>, (StatusCode, String)> {
    let p = pool(&bot_state).await?;
    let _ = ensure_category_exists(&p, &id).await?;
    let role_id = validate_role_id(&role_id)?;
    remove_category_role(&id, &role_id, &p)
        .await
        .map_err(internal)?;
    let role_ids = list_category_role_ids(&id, &p).await.map_err(internal)?;
    Ok(Json(CategoryRolesDto { role_ids }))
}
