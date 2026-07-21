use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serenity::all::GuildId;
use serenity::http::Http;
use sqlx::SqlitePool;
use std::fs;
use std::net::UdpSocket;
use std::sync::Arc;

pub use rustmail_types::*;
#[derive(Debug, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub command: CommandConfig,
    pub thread: ThreadConfig,
    pub language: LanguageConfig,
    pub error_handling: ErrorHandlingConfig,
    pub notifications: NotificationsConfig,
    pub reminders: ReminderConfig,
    pub logs: LogsConfig,

    pub db_pool: Option<SqlitePool>,
    pub error_handler: Option<Arc<ErrorHandler>>,
    pub thread_locks:
        Arc<std::sync::Mutex<std::collections::HashMap<u64, Arc<tokio::sync::Mutex<()>>>>>,
}

fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("1.1.1.1:80").ok()?;
    Some(socket.local_addr().ok()?.ip().to_string())
}

pub fn resolve_config_path(default: &str) -> String {
    std::env::var("RUSTMAIL_CONFIG_PATH").unwrap_or_else(|_| default.to_string())
}

pub fn resolve_db_path(default: &str) -> String {
    std::env::var("RUSTMAIL_DATABASE_URL")
        .or_else(|_| std::env::var("RUSTMAIL_DB_PATH"))
        .unwrap_or_else(|_| default.to_string())
}

pub fn resolve_bind_address(default: &str) -> String {
    std::env::var("RUSTMAIL_BIND_ADDRESS").unwrap_or_else(|_| default.to_string())
}

pub fn resolve_port(default: u16) -> u16 {
    std::env::var("RUSTMAIL_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(default)
}

fn apply_env_overrides(config: &mut ConfigResponse) {
    if let Ok(token) = std::env::var("RUSTMAIL_BOT_TOKEN")
        && !token.is_empty()
    {
        config.bot.token = token;
    }

    if let Ok(client_secret) = std::env::var("RUSTMAIL_BOT_CLIENT_SECRET")
        && !client_secret.is_empty()
    {
        config.bot.client_secret = client_secret;
    }

    if let Ok(client_id) = std::env::var("RUSTMAIL_BOT_CLIENT_ID")
        && let Ok(id) = client_id.parse::<u64>()
    {
        config.bot.client_id = id;
    }
}

pub fn load_config(path: &str) -> Option<Config> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let mut config_response: ConfigResponse = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse config.toml: {}", e);
            return None;
        }
    };

    apply_env_overrides(&mut config_response);

    let mut bot = config_response.bot;
    if bot.ip.is_none() {
        bot.ip = get_local_ip();
    }

    if u64::from_str_radix(&config_response.thread.user_message_color, 16).is_err() {
        eprintln!(
            "Incorrect user message color in the config.toml! Please put a color in hex format!"
        );
        return None;
    }

    if u64::from_str_radix(&config_response.thread.staff_message_color, 16).is_err() {
        eprintln!(
            "Incorrect staff message color in the config.toml! Please put a color in hex format!"
        );
        return None;
    }

    if u64::from_str_radix(&config_response.reminders.embed_color, 16).is_err() {
        eprintln!(
            "Incorrect reminder embed color in the config.toml! Please put a color in hex format!"
        );
        return None;
    }

    if !config_response
        .language
        .is_language_supported(config_response.language.get_default_language())
    {
        eprintln!(
            "Warning: Default language '{}' is not in supported languages list",
            config_response.language.default_language
        );
    }

    if let Err(e) = bot.validate_logs_config() {
        eprintln!("Invalid logs configuration: {}", e);
        return None;
    }

    if let Err(e) = bot.validate_features_config() {
        eprintln!("Invalid features configuration: {}", e);
        return None;
    }

    let default_lang = config_response.language.get_default_language();
    let fallback_lang = config_response.language.get_fallback_language();
    let error_handler = Arc::new(ErrorHandler::with_languages(default_lang, fallback_lang));

    Some(Config {
        bot,
        command: config_response.command,
        thread: config_response.thread,
        language: config_response.language,
        error_handling: config_response.error_handling,
        notifications: config_response.notifications,
        reminders: config_response.reminders,
        logs: config_response.logs,
        db_pool: None,
        error_handler: Some(error_handler),
        thread_locks: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    })
}

pub fn validate_config(config: &Config) -> Result<(), String> {
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

pub async fn save_config_with_backup(config: &Config, path: &str) -> Result<(), String> {
    if std::path::Path::new(path).exists() {
        let backup_path = format!("{}.backup", path);
        fs::copy(path, &backup_path).map_err(|e| format!("Failed to create backup: {}", e))?;
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

pub trait LanguageConfigExt {
    fn get_default_language(&self) -> Language;
    fn get_fallback_language(&self) -> Language;
    fn get_supported_languages(&self) -> Vec<Language>;
    fn is_language_supported(&self, language: Language) -> bool;
}

impl LanguageConfigExt for LanguageConfig {
    fn get_default_language(&self) -> Language {
        Language::from_str(&self.default_language).unwrap_or(Language::English)
    }

    fn get_fallback_language(&self) -> Language {
        Language::from_str(&self.fallback_language).unwrap_or(Language::English)
    }

    fn get_supported_languages(&self) -> Vec<Language> {
        self.supported_languages
            .iter()
            .filter_map(|s| Language::from_str(s))
            .collect()
    }

    fn is_language_supported(&self, language: Language) -> bool {
        self.get_supported_languages().contains(&language)
    }
}

impl Config {
    pub async fn validate_servers(&self, http: &Http) -> Result<(), String> {
        match &self.bot.mode {
            ServerMode::Single { guild_id } => {
                let guild_id = GuildId::new(*guild_id);
                if guild_id.to_partial_guild(http).await.is_err() {
                    return Err(format!("Serveur principal introuvable: {}", guild_id));
                }
            }
            ServerMode::Dual {
                community_guild_id,
                staff_guild_id,
            } => {
                let community_guild_id = GuildId::new(*community_guild_id);
                let staff_guild_id = GuildId::new(*staff_guild_id);

                if let Err(e) = community_guild_id.to_partial_guild(http).await {
                    eprintln!("Error fetching community guild: {}", e);
                    return Err(format!(
                        "Serveur communautaire introuvable: {}",
                        community_guild_id
                    ));
                }

                if staff_guild_id.to_partial_guild(http).await.is_err() {
                    return Err(format!("Serveur staff introuvable: {}", staff_guild_id));
                }

                if community_guild_id == staff_guild_id {
                    return Err(
                        "Les serveurs communautaire et staff doivent être différents".to_string(),
                    );
                }
            }
        }

        Ok(())
    }
}
