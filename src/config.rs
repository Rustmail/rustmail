use crate::errors::handler::ErrorHandler;
use crate::i18n::languages::Language;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::fs;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub command: CommandConfig,
    pub thread: ThreadConfig,
    pub language: LanguageConfig,
    pub error_handling: ErrorHandlingConfig,
    #[serde(skip)]
    pub db_pool: Option<SqlitePool>,
    #[serde(skip)]
    pub error_handler: Option<Arc<ErrorHandler>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub token: String,
    pub guild_id: u64,
    pub status: String,
    pub typing_proxy_from_user: bool,
    pub typing_proxy_from_staff: bool,
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct LanguageConfig {
    pub default_language: String,
    pub auto_detect: bool,
    pub fallback_language: String,
    pub supported_languages: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorHandlingConfig {
    pub show_detailed_errors: bool,
    pub log_errors: bool,
    pub send_error_embeds: bool,
    pub auto_delete_error_messages: bool,
    pub error_message_ttl: Option<u64>,
}

pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path)
        .expect("No configuration file found ! Add 'config.toml' file at the root!");

    let mut config: Config =
        toml::from_str(&content).expect("The format of the config.toml is not correct!");

    let _ = u64::from_str_radix(&config.thread.user_message_color, 16).expect(
        "Incorect user message color in the config.toml! Please put a color in hex format!",
    );
    let _ = u64::from_str_radix(&config.thread.staff_message_color, 16).expect(
        "Incorect staff message color in the config.toml! Please put a color in hex format!",
    );

    if !config
        .language
        .is_language_supported(config.language.get_default_language())
    {
        eprintln!(
            "Warning: Default language '{}' is not in supported languages list",
            config.language.default_language
        );
    }

    let error_handler = Arc::new(ErrorHandler::new());
    config.error_handler = Some(error_handler);

    config
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            default_language: "en".to_string(),
            auto_detect: true,
            fallback_language: "en".to_string(),
            supported_languages: vec!["en".to_string(), "fr".to_string()],
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
