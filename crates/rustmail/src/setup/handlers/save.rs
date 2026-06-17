use crate::config::{Config, resolve_config_path, save_config_with_backup, validate_config};
use crate::setup::state::SharedSetupState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rustmail_types::{
    BotConfig, CommandConfig, ErrorHandlingConfig, LanguageConfig, LogsConfig, NotificationsConfig,
    ReminderConfig, ServerMode, ThreadConfig,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SaveConfigRequest {
    pub token: String,
    pub bot_status: String,
    pub welcome_message: String,
    pub close_message: String,
    pub typing_proxy_from_user: bool,
    pub typing_proxy_from_staff: bool,
    pub server_mode: String,
    pub guild_id: Option<u64>,
    pub community_guild_id: Option<u64>,
    pub staff_guild_id: Option<u64>,
    pub enable_rustmail_logs: bool,
    pub enable_discord_logs: bool,
    pub logs_channel_id: Option<u64>,
    pub enable_features: bool,
    pub features_channel_id: Option<u64>,
    pub enable_panel: bool,
    pub api_port: u16,
    pub client_id: Option<u64>,
    pub client_secret: Option<String>,
    pub redirect_url: Option<String>,
    pub panel_super_admin_users: Vec<u64>,
    pub inbox_category_id: u64,
    pub command_prefix: String,
    pub user_message_color: String,
    pub staff_message_color: String,
    pub system_message_color: String,
    pub embedded_message: bool,
    pub block_quote: bool,
    pub time_to_close_thread: u64,
    pub create_ticket_by_create_channel: bool,
    pub close_on_leave: bool,
    pub auto_archive_duration: u16,
    pub default_language: String,
    pub fallback_language: String,
    pub timezone: String,
}

pub async fn handle_setup_save(
    State(setup_state): State<SharedSetupState>,
    Json(payload): Json<SaveConfigRequest>,
) -> impl IntoResponse {
    let mode = match payload.server_mode.as_str() {
        "dual" => {
            let community = match payload.community_guild_id {
                Some(id) => id,
                None => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "success": false,
                            "error": "community_guild_id is required for dual mode"
                        })),
                    );
                }
            };
            let staff = match payload.staff_guild_id {
                Some(id) => id,
                None => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "success": false,
                            "error": "staff_guild_id is required for dual mode"
                        })),
                    );
                }
            };
            ServerMode::Dual {
                community_guild_id: community,
                staff_guild_id: staff,
            }
        }
        _ => {
            let id = match payload.guild_id {
                Some(id) => id,
                None => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "success": false,
                            "error": "guild_id is required for single mode"
                        })),
                    );
                }
            };
            ServerMode::Single { guild_id: id }
        }
    };

    let bot_config = BotConfig {
        token: payload.token,
        mode,
        status: payload.bot_status,
        welcome_message: payload.welcome_message,
        close_message: payload.close_message,
        typing_proxy_from_user: payload.typing_proxy_from_user,
        typing_proxy_from_staff: payload.typing_proxy_from_staff,
        enable_rustmail_logs: payload.enable_rustmail_logs,
        enable_discord_logs: payload.enable_discord_logs,
        enable_features: payload.enable_features,
        enable_panel: payload.enable_panel,
        client_id: payload.client_id.unwrap_or(0),
        client_secret: payload.client_secret.unwrap_or_default(),
        redirect_url: payload.redirect_url.unwrap_or_default(),
        timezone: payload.timezone.parse().unwrap_or(chrono_tz::UTC),
        logs_channel_id: payload.logs_channel_id,
        features_channel_id: payload.features_channel_id,
        ip: None,
        panel_super_admin_users: payload.panel_super_admin_users,
        panel_super_admin_roles: vec![],
        panel_port: payload.api_port,
    };

    let config = Config {
        bot: bot_config,
        command: CommandConfig {
            prefix: payload.command_prefix,
        },
        thread: ThreadConfig {
            inbox_category_id: payload.inbox_category_id,
            embedded_message: payload.embedded_message,
            user_message_color: payload.user_message_color,
            staff_message_color: payload.staff_message_color,
            system_message_color: payload.system_message_color,
            block_quote: payload.block_quote,
            time_to_close_thread: payload.time_to_close_thread,
            create_ticket_by_create_channel: payload.create_ticket_by_create_channel,
            close_on_leave: payload.close_on_leave,
            auto_archive_duration: payload.auto_archive_duration,
        },
        language: LanguageConfig {
            default_language: payload.default_language.clone(),
            fallback_language: payload.fallback_language,
            supported_languages: vec![payload.default_language],
        },
        error_handling: ErrorHandlingConfig::default(),
        notifications: NotificationsConfig::default(),
        reminders: ReminderConfig::default(),
        logs: LogsConfig::default(),
        db_pool: None,
        error_handler: None,
        thread_locks: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    };

    if let Err(e) = validate_config(&config) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": e })),
        );
    }

    let config_path = resolve_config_path("config.toml");

    if let Err(e) = save_config_with_backup(&config, &config_path).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": e })),
        );
    }

    let mut state = setup_state.lock().await;
    state.step = crate::setup::state::SetupStep::Review;

    (StatusCode::OK, Json(serde_json::json!({ "success": true })))
}
