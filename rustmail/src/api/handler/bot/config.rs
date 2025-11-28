use crate::config::{load_config, Config, LanguageConfigExt};
use crate::prelude::types::*;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use rustmail_types::ConfigResponse;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;

fn mask_secret(secret: &str) -> String {
    let len = secret.chars().count();
    if len <= 8 {
        "*".repeat(len)
    } else {
        let chars: Vec<char> = secret.chars().collect();
        let start: String = chars.iter().take(4).collect();
        let end: String = chars.iter().skip(len - 4).collect();
        format!("{}...{}", start, end)
    }
}

pub async fn handle_get_config(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    let state = bot_state.lock().await;

    let config = match &state.config {
        Some(c) => c,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut masked_bot = config.bot.clone();
    masked_bot.token = mask_secret(&config.bot.token);
    masked_bot.client_secret = mask_secret(&config.bot.client_secret);

    let response = ConfigResponse {
        bot: masked_bot,
        command: config.command.clone(),
        thread: config.thread.clone(),
        language: config.language.clone(),
        error_handling: config.error_handling.clone(),
        notifications: config.notifications.clone(),
        reminders: config.reminders.clone(),
        logs: config.logs.clone(),
    };

    Ok(Json(response))
}

pub async fn handle_update_config(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(update): Json<ConfigResponse>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let current_config = {
        let state = bot_state.lock().await;
        match &state.config {
            Some(c) => c.clone(),
            None => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Configuration not loaded".to_string(),
                ))
            }
        }
    };

    let mut new_bot_config = update.bot.clone();

    if new_bot_config.token.contains("...") {
        new_bot_config.token = current_config.bot.token.clone();
    }

    if new_bot_config.client_secret.contains("...") {
        new_bot_config.client_secret = current_config.bot.client_secret.clone();
    }

    let new_config = Config {
        bot: new_bot_config,
        command: update.command,
        thread: update.thread,
        language: update.language,
        error_handling: update.error_handling,
        notifications: update.notifications,
        reminders: update.reminders,
        logs: update.logs,
        db_pool: None,
        error_handler: None,
        thread_locks: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    };

    if let Err(e) = validate_config(&new_config) {
        return Err((StatusCode::BAD_REQUEST, e));
    }

    if let Err(e) = save_config_with_backup(&new_config, "config.toml").await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e));
    }

    let mut state = bot_state.lock().await;
    state.config = load_config("config.toml");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Configuration saved successfully. Restart the bot to apply changes."
    })))
}

fn validate_config(config: &Config) -> Result<(), String> {
    if u64::from_str_radix(&config.thread.user_message_color, 16).is_err() {
        return Err("Invalid user message color format (must be hex)".to_string());
    }

    if u64::from_str_radix(&config.thread.staff_message_color, 16).is_err() {
        return Err("Invalid staff message color format (must be hex)".to_string());
    }

    if u64::from_str_radix(&config.reminders.embed_color, 16).is_err() {
        return Err("Invalid reminder embed color format (must be hex)".to_string());
    }

    config.bot.validate_logs_config()?;
    config.bot.validate_features_config()?;

    if !config
        .language
        .is_language_supported(config.language.get_default_language())
    {
        return Err(format!(
            "Default language '{}' is not in supported languages list",
            config.language.default_language
        ));
    }

    Ok(())
}

async fn save_config_with_backup(config: &Config, path: &str) -> Result<(), String> {
    if std::path::Path::new(path).exists() {
        let backup_path = format!("{}.backup", path);
        fs::copy(path, &backup_path)
            .map_err(|e| format!("Failed to create backup: {}", e))?;
    }

    let config_response = ConfigResponse {
        bot: config.bot.clone(),
        command: config.command.clone(),
        thread: config.thread.clone(),
        language: config.language.clone(),
        error_handling: config.error_handling.clone(),
        notifications: config.notifications.clone(),
        reminders: config.reminders.clone(),
        logs: config.logs.clone(),
    };

    let toml_content = toml::to_string_pretty(&config_response)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, toml_content).map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}
