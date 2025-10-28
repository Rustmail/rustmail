use crate::errors::handler::ErrorHandler;
use crate::i18n::languages::Language;
use chrono_tz::Tz;
use serde::Deserialize;
use serenity::all::GuildId;
use serenity::http::Http;
use sqlx::SqlitePool;
use std::fs;
use std::net::UdpSocket;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub command: CommandConfig,
    pub thread: ThreadConfig,
    pub language: LanguageConfig,
    pub error_handling: ErrorHandlingConfig,
    pub notifications: NotificationsConfig,
    pub reminders: ReminderConfig,
    pub logs: LogsConfig,
    #[serde(skip)]
    pub db_pool: Option<SqlitePool>,
    #[serde(skip)]
    pub error_handler: Option<Arc<ErrorHandler>>,
    #[serde(skip)]
    pub thread_locks:
        Arc<std::sync::Mutex<std::collections::HashMap<u64, Arc<tokio::sync::Mutex<()>>>>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub token: String,
    pub mode: ServerMode,
    pub status: String,
    pub welcome_message: String,
    pub close_message: String,
    pub typing_proxy_from_user: bool,
    pub typing_proxy_from_staff: bool,
    pub enable_logs: bool,
    pub enable_features: bool,
    pub enable_panel: bool,
    pub client_id: u64,
    pub client_secret: String,
    pub redirect_url: String,
    #[serde(
        default = "default_timezone",
        deserialize_with = "deserialize_timezone"
    )]
    pub timezone: Tz,
    #[serde(default)]
    pub logs_channel_id: Option<u64>,
    #[serde(default)]
    pub features_channel_id: Option<u64>,
    #[serde(default)]
    pub ip: Option<String>,
}

fn deserialize_timezone<'de, D>(deserializer: D) -> Result<Tz, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<Tz>().map_err(serde::de::Error::custom)
}

fn default_timezone() -> Tz {
    chrono_tz::UTC
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerMode {
    Single {
        guild_id: u64,
    },
    Dual {
        community_guild_id: u64,
        staff_guild_id: u64,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommandConfig {
    pub prefix: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadConfig {
    pub inbox_category_id: u64,
    pub embedded_message: bool,
    pub user_message_color: String,
    pub staff_message_color: String,
    pub system_message_color: String,
    pub block_quote: bool,
    pub time_to_close_thread: u64,
    pub create_ticket_by_create_channel: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NotificationsConfig {
    pub show_success_on_edit: bool,
    pub show_partial_success_on_edit: bool,
    pub show_failure_on_edit: bool,
    pub show_success_on_reply: bool,
    pub show_success_on_delete: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogsConfig {
    pub show_log_on_edit: bool,
    pub show_log_on_delete: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReminderConfig {
    pub embed_color: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LanguageConfig {
    pub default_language: String,
    pub fallback_language: String,
    pub supported_languages: Vec<String>,
    pub error_message_ttl: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorHandlingConfig {
    pub show_detailed_errors: bool,
    pub log_errors: bool,
    pub send_error_embeds: bool,
    pub auto_delete_error_messages: bool,
    pub error_message_ttl: Option<u64>,
}

fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;

    socket.connect("1.1.1.1:80").ok()?;
    Some(socket.local_addr().ok()?.ip().to_string())
}

pub fn load_config(path: &str) -> Option<Config> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let mut config: Config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse config.toml: {}", e);
            return None;
        }
    };

    if config.bot.ip.is_none() {
        config.bot.ip = get_local_ip();
    }

    match u64::from_str_radix(&config.thread.user_message_color, 16) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "Incorect user message color in the config.toml! Please put a color in hex format!"
            );
            return None;
        }
    }

    match u64::from_str_radix(&config.thread.staff_message_color, 16) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "Incorect staff message color in the config.toml! Please put a color in hex format!"
            );
            return None;
        }
    }

    match u64::from_str_radix(&config.reminders.embed_color, 16) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "Incorect reminder embed color in the config.toml! Please put a color in hex format!"
            );
            return None;
        }
    }

    if !config
        .language
        .is_language_supported(config.language.get_default_language())
    {
        eprintln!(
            "Warning: Default language '{}' is not in supported languages list",
            config.language.default_language
        );
    }

    match config.bot.validate_logs_config() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Invalid logs configuration: {}", e);
            return None;
        }
    }

    match config.bot.validate_features_config() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Invalid features configuration: {}", e);
            return None;
        }
    }

    let default_lang = config.language.get_default_language();
    let fallback_lang = config.language.get_fallback_language();
    let error_handler = Arc::new(ErrorHandler::with_languages(default_lang, fallback_lang));
    config.error_handler = Some(error_handler);

    config.thread_locks = Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));

    Some(config)
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            default_language: "en".to_string(),
            fallback_language: "en".to_string(),
            supported_languages: vec!["en".to_string(), "fr".to_string()],
            error_message_ttl: Some(30),
        }
    }
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            embed_color: "ffcc00".to_string(),
        }
    }
}

impl Default for LogsConfig {
    fn default() -> Self {
        Self {
            show_log_on_edit: true,
            show_log_on_delete: true,
        }
    }
}

impl Default for ErrorHandlingConfig {
    fn default() -> Self {
        Self {
            show_detailed_errors: false,
            log_errors: true,
            send_error_embeds: true,
            auto_delete_error_messages: true,
            error_message_ttl: Some(30),
        }
    }
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            show_success_on_edit: true,
            show_partial_success_on_edit: true,
            show_failure_on_edit: true,
            show_success_on_reply: true,
            show_success_on_delete: true,
        }
    }
}

impl LanguageConfig {
    pub fn get_default_language(&self) -> Language {
        Language::from_str(&self.default_language).unwrap_or(Language::English)
    }

    pub fn get_fallback_language(&self) -> Language {
        Language::from_str(&self.fallback_language).unwrap_or(Language::English)
    }

    pub fn get_supported_languages(&self) -> Vec<Language> {
        self.supported_languages
            .iter()
            .filter_map(|s| Language::from_str(s))
            .collect()
    }

    pub fn is_language_supported(&self, language: Language) -> bool {
        self.get_supported_languages().contains(&language)
    }
}

impl BotConfig {
    pub fn get_community_guild_id(&self) -> u64 {
        match &self.mode {
            ServerMode::Single { guild_id } => *guild_id,
            ServerMode::Dual {
                community_guild_id, ..
            } => *community_guild_id,
        }
    }

    pub fn get_staff_guild_id(&self) -> u64 {
        match &self.mode {
            ServerMode::Single { guild_id } => *guild_id,
            ServerMode::Dual { staff_guild_id, .. } => *staff_guild_id,
        }
    }

    pub fn is_dual_mode(&self) -> bool {
        matches!(self.mode, ServerMode::Dual { .. })
    }

    pub fn validate_logs_config(&self) -> Result<(), String> {
        match (self.enable_logs, self.logs_channel_id) {
            (true, None) => {
                Err("'logs_channel_id' field is required if 'enable_logs' is true".to_string())
            }
            (false, Some(_)) => {
                Err("'logs_channel_id' must not be filled in if 'enable_logs' is false".to_string())
            }
            (true, Some(_)) => Ok(()),
            (false, None) => Ok(()),
        }
    }

    pub fn validate_features_config(&self) -> Result<(), String> {
        match (self.enable_features, self.features_channel_id) {
            (true, None) => Err(
                "'features_channel_id' field is required if 'enable_features' is true".to_string(),
            ),
            (false, Some(_)) => Err(
                "'features_channel_id' must not be filled in if 'enable_features' is false"
                    .to_string(),
            ),
            (true, Some(_)) => Ok(()),
            (false, None) => Ok(()),
        }
    }

    pub fn is_community_guild(&self, p0: u64) -> bool {
        match &self.mode {
            ServerMode::Single { guild_id } => *guild_id == p0,
            ServerMode::Dual {
                community_guild_id, ..
            } => *community_guild_id == p0,
        }
    }
}

impl Config {
    pub async fn validate_servers(&self, http: &Http) -> Result<(), String> {
        match &self.bot.mode {
            ServerMode::Single { guild_id } => {
                let guild_id = GuildId::new(*guild_id);
                if let Err(_) = guild_id.to_partial_guild(http).await {
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

                if let Err(_) = staff_guild_id.to_partial_guild(http).await {
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
