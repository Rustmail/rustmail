use crate::errors::dictionary::DictionaryManager;
use crate::errors::types::{ModmailError, ModmailResult};
use crate::i18n::languages::{Language, LanguageDetector, LanguagePreferences};
use serenity::all::{ChannelId, Colour, Context, CreateEmbed, CreateMessage, Message, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct ErrorHandler {
    dictionary_manager: Arc<RwLock<DictionaryManager>>,
    user_languages: Arc<RwLock<HashMap<UserId, LanguagePreferences>>>,
    guild_languages: Arc<RwLock<HashMap<u64, Language>>>,
    default_language: Language,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            dictionary_manager: Arc::new(RwLock::new(DictionaryManager::new())),
            user_languages: Arc::new(RwLock::new(HashMap::new())),
            guild_languages: Arc::new(RwLock::new(HashMap::new())),
            default_language: Language::English,
        }
    }

    pub fn with_languages(default_language: Language, fallback_language: Language) -> Self {
        Self {
            dictionary_manager: Arc::new(RwLock::new(DictionaryManager::with_fallback_language(
                fallback_language,
            ))),
            user_languages: Arc::new(RwLock::new(HashMap::new())),
            guild_languages: Arc::new(RwLock::new(HashMap::new())),
            default_language,
        }
    }

    pub async fn get_user_language(&self, user_id: UserId, guild_id: Option<u64>) -> Language {
        if let Some(prefs) = self.user_languages.read().await.get(&user_id) {
            return prefs.primary;
        }

        if let Some(guild_id) = guild_id
            && let Some(lang) = self.guild_languages.read().await.get(&guild_id) {
                return *lang;
            }

        self.default_language
    }

    pub async fn set_user_language(&self, user_id: UserId, preferences: LanguagePreferences) {
        self.user_languages
            .write()
            .await
            .insert(user_id, preferences);
    }

    pub async fn set_guild_language(&self, guild_id: u64, language: Language) {
        self.guild_languages
            .write()
            .await
            .insert(guild_id, language);

        let mut dict_manager = self.dictionary_manager.write().await;
        dict_manager.load_language(language);
    }

    pub async fn detect_language_from_interaction(
        &self,
        user_id: UserId,
        locale: Option<&str>,
    ) -> Language {
        if let Some(locale) = locale
            && let Some(detected) = LanguageDetector::from_discord_locale(locale) {
                let prefs = LanguagePreferences::new(detected);
                self.set_user_language(user_id, prefs).await;
                return detected;
            }

        self.default_language
    }

    pub async fn handle_error(
        &self,
        error: &ModmailError,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
    ) -> FormattedError {
        let language = if let Some(uid) = user_id {
            self.get_user_language(uid, guild_id).await
        } else {
            self.default_language
        };

        let dict_manager = self.dictionary_manager.read().await;
        let message = dict_manager.translate_error(error, language);

        FormattedError {
            message,
            language,
            error_type: self.get_error_type(error),
            severity: self.get_error_severity(error),
            should_log: self.should_log_error(error),
            user_facing: true,
        }
    }

    pub async fn send_error_message(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
        error: &ModmailError,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
        ephemeral: bool,
    ) -> Result<Message, serenity::Error> {
        let formatted_error = self.handle_error(error, user_id, guild_id).await;

        let embed = self.create_error_embed(&formatted_error).await;
        let message = CreateMessage::new().embed(embed);

        if ephemeral {
            channel_id.send_message(&ctx.http, message).await
        } else {
            channel_id.send_message(&ctx.http, message).await
        }
    }

    pub async fn reply_with_error(
        &self,
        ctx: &Context,
        msg: &Message,
        error: &ModmailError,
    ) -> Result<Message, serenity::Error> {
        let formatted_error = self
            .handle_error(error, Some(msg.author.id), msg.guild_id.map(|g| g.get()))
            .await;

        let embed = self.create_error_embed(&formatted_error).await;
        msg.channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await
    }

    pub async fn create_success_message(
        &self,
        key: &str,
        params: Option<HashMap<String, String>>,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
    ) -> FormattedMessage {
        let language = if let Some(uid) = user_id {
            self.get_user_language(uid, guild_id).await
        } else {
            self.default_language
        };

        let dict_manager = self.dictionary_manager.read().await;
        let message = dict_manager.get_message(language, key, params.as_ref(), None);

        FormattedMessage {
            message,
            language,
            message_type: MessageType::Success,
        }
    }

    pub async fn send_success_message(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
        key: &str,
        params: Option<HashMap<String, String>>,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
    ) -> Result<Message, serenity::Error> {
        let formatted_message = self
            .create_success_message(key, params, user_id, guild_id)
            .await;

        let embed = self.create_success_embed(&formatted_message).await;
        channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await
    }

    async fn create_error_embed(&self, formatted_error: &FormattedError) -> CreateEmbed {
        let color = match formatted_error.severity {
            ErrorSeverity::Critical => Colour::DARK_RED,
            ErrorSeverity::High => Colour::RED,
            ErrorSeverity::Medium => Colour::ORANGE,
            ErrorSeverity::Low => Colour::GOLD,
        };

        let emoji = match formatted_error.severity {
            ErrorSeverity::Critical => "🚨",
            ErrorSeverity::High => "❌",
            ErrorSeverity::Medium => "⚠️",
            ErrorSeverity::Low => "ℹ️",
        };

        CreateEmbed::new()
            .title(format!("{} Error", emoji))
            .description(&formatted_error.message)
            .color(color)
            .footer(serenity::all::CreateEmbedFooter::new(format!(
                "Error Type: {} | Language: {}",
                formatted_error.error_type,
                formatted_error.language.code()
            )))
            .timestamp(serenity::all::Timestamp::now())
    }

    async fn create_success_embed(&self, formatted_message: &FormattedMessage) -> CreateEmbed {
        CreateEmbed::new()
            .title("✅ Success")
            .description(&formatted_message.message)
            .color(Colour::DARK_GREEN)
            .footer(serenity::all::CreateEmbedFooter::new(format!(
                "Language: {}",
                formatted_message.language.code()
            )))
            .timestamp(serenity::all::Timestamp::now())
    }

    fn get_error_type(&self, error: &ModmailError) -> String {
        match error {
            ModmailError::Database(_) => "Database".to_string(),
            ModmailError::Discord(_) => "Discord".to_string(),
            ModmailError::Command(_) => "Command".to_string(),
            ModmailError::Thread(_) => "Thread".to_string(),
            ModmailError::Message(_) => "Message".to_string(),
            ModmailError::Config(_) => "Config".to_string(),
            ModmailError::Validation(_) => "Validation".to_string(),
            ModmailError::Permission(_) => "Permission".to_string(),
            ModmailError::Generic(_) => "Generic".to_string(),
        }
    }

    pub fn get_error_severity(&self, error: &ModmailError) -> ErrorSeverity {
        match error {
            ModmailError::Database(_) => ErrorSeverity::High,
            ModmailError::Discord(discord_err) => {
                use crate::errors::types::DiscordError;
                match discord_err {
                    DiscordError::InvalidToken => ErrorSeverity::Critical,
                    DiscordError::PermissionDenied => ErrorSeverity::Medium,
                    DiscordError::RateLimited => ErrorSeverity::Medium,
                    _ => ErrorSeverity::Low,
                }
            }
            ModmailError::Command(_) => ErrorSeverity::Low,
            ModmailError::Thread(_) => ErrorSeverity::Medium,
            ModmailError::Message(_) => ErrorSeverity::Low,
            ModmailError::Config(_) => ErrorSeverity::High,
            ModmailError::Validation(_) => ErrorSeverity::Low,
            ModmailError::Permission(_) => ErrorSeverity::Medium,
            ModmailError::Generic(_) => ErrorSeverity::Medium,
        }
    }

    pub fn should_log_error(&self, error: &ModmailError) -> bool {
        match self.get_error_severity(error) {
            ErrorSeverity::Critical | ErrorSeverity::High => true,
            ErrorSeverity::Medium => true,
            ErrorSeverity::Low => false,
        }
    }

    pub async fn get_help_text(
        &self,
        command: &str,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
    ) -> String {
        let language = if let Some(uid) = user_id {
            self.get_user_language(uid, guild_id).await
        } else {
            self.default_language
        };

        let key = format!("help.{}", command);
        let dict_manager = self.dictionary_manager.read().await;
        dict_manager.get_message(language, &key, None, None)
    }

    pub async fn load_custom_dictionary(
        &self,
        language: Language,
        _file_path: &str,
    ) -> ModmailResult<()> {
        let mut dict_manager = self.dictionary_manager.write().await;
        dict_manager.load_language(language);
        Ok(())
    }

    pub fn get_supported_languages(&self) -> Vec<Language> {
        Language::all()
    }

    pub fn is_language_supported(&self, language: Language) -> bool {
        self.get_supported_languages().contains(&language)
    }

    pub async fn get_dictionary_message(
        &self,
        language: Language,
        key: &str,
        params: Option<&std::collections::HashMap<String, String>>,
        count: Option<i64>,
    ) -> String {
        let dict_manager = self.dictionary_manager.read().await;
        dict_manager.get_message(language, key, params, count)
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct FormattedError {
    pub message: String,
    pub language: Language,
    pub error_type: String,
    pub severity: ErrorSeverity,
    pub should_log: bool,
    pub user_facing: bool,
}

#[derive(Debug, Clone)]
pub struct FormattedMessage {
    pub message: String,
    pub language: Language,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Success,
    Info,
    Warning,
    Error,
}

#[async_trait::async_trait]
pub trait ErrorHandling {
    async fn handle_error_with_reply(
        &self,
        ctx: &Context,
        msg: &Message,
        error: &ModmailError,
        error_handler: &ErrorHandler,
    ) -> Result<(), serenity::Error>;

    async fn handle_result_with_reply<T: Send>(
        &self,
        ctx: &Context,
        msg: &Message,
        result: ModmailResult<T>,
        error_handler: &ErrorHandler,
    ) -> Option<T>;
}

#[async_trait::async_trait]
impl ErrorHandling for Message {
    async fn handle_error_with_reply(
        &self,
        ctx: &Context,
        msg: &Message,
        error: &ModmailError,
        error_handler: &ErrorHandler,
    ) -> Result<(), serenity::Error> {
        error_handler.reply_with_error(ctx, msg, error).await?;
        Ok(())
    }

    async fn handle_result_with_reply<T: Send>(
        &self,
        ctx: &Context,
        msg: &Message,
        result: ModmailResult<T>,
        error_handler: &ErrorHandler,
    ) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(error) => {
                let _ = self
                    .handle_error_with_reply(ctx, msg, &error, error_handler)
                    .await;
                None
            }
        }
    }
}

#[macro_export]
macro_rules! handle_error {
    ($ctx:expr, $msg:expr, $result:expr, $error_handler:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                let _ = $error_handler.reply_with_error($ctx, $msg, &error).await;
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! send_success {
    ($ctx:expr, $channel:expr, $key:expr, $error_handler:expr) => {
        $error_handler
            .send_success_message($ctx, $channel, $key, None, None, None)
            .await
    };
    ($ctx:expr, $channel:expr, $key:expr, $params:expr, $error_handler:expr) => {
        $error_handler
            .send_success_message($ctx, $channel, $key, Some($params), None, None)
            .await
    };
}

#[macro_export]
macro_rules! translate_error {
    ($error:expr, $language:expr, $dict_manager:expr) => {
        $dict_manager.translate_error($error, $language)
    };
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub command: Option<String>,
    pub user_id: Option<UserId>,
    pub channel_id: Option<ChannelId>,
    pub guild_id: Option<u64>,
    pub additional_info: HashMap<String, String>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            command: None,
            user_id: None,
            channel_id: None,
            guild_id: None,
            additional_info: HashMap::new(),
        }
    }

    pub fn with_command(mut self, command: String) -> Self {
        self.command = Some(command);
        self
    }

    pub fn with_user(mut self, user_id: UserId) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_channel(mut self, channel_id: ChannelId) -> Self {
        self.channel_id = Some(channel_id);
        self
    }

    pub fn with_guild(mut self, guild_id: u64) -> Self {
        self.guild_id = Some(guild_id);
        self
    }

    pub fn with_info(mut self, key: String, value: String) -> Self {
        self.additional_info.insert(key, value);
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}
