use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDictionary {
    pub language: Language,
    pub messages: HashMap<String, DictionaryMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryMessage {
    pub default: String,
    pub plurals: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub help: Option<String>,
}

impl DictionaryMessage {
    pub fn new(message: &str) -> Self {
        Self {
            default: message.to_string(),
            plurals: None,
            description: None,
            help: None,
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        self
    }

    pub fn get_message(&self, count: Option<i64>, language: Language) -> &str {
        if let (Some(count), Some(plurals)) = (count, &self.plurals) {
            let plural_form = language.plural_form(count);
            if let Some(plural_msg) = plurals.get(plural_form.as_str()) {
                return plural_msg;
            }
        }
        &self.default
    }
}

impl ErrorDictionary {
    pub fn new(language: Language) -> Self {
        let mut dictionary = Self {
            language,
            messages: HashMap::new(),
        };
        dictionary.load_default_messages();
        dictionary
    }

    pub fn get_message(&self, key: &str) -> Option<&DictionaryMessage> {
        self.messages.get(key)
    }

    pub fn get_formatted_message(
        &self,
        key: &str,
        params: Option<&HashMap<String, String>>,
        count: Option<i64>,
    ) -> String {
        if let Some(error_msg) = self.get_message(key) {
            let mut message = error_msg.get_message(count, self.language).to_string();

            if let Some(params) = params {
                for (param, value) in params {
                    message = message.replace(&format!("{{{}}}", param), value);
                }
            }

            message
        } else {
            format!("Unknown error: {}", key)
        }
    }

    fn load_default_messages(&mut self) {
        match self.language {
            Language::English => load_english_messages(self),
            Language::French => load_french_messages(self),
            Language::Spanish => load_spanish_messages(self),
            Language::German => load_german_messages(self),
            Language::Italian => load_italian_messages(self),
            Language::Portuguese => load_portuguese_messages(self),
            Language::Dutch => load_dutch_messages(self),
            Language::Russian => load_russian_messages(self),
            Language::Japanese => load_japanese_messages(self),
            Language::Korean => load_korean_messages(self),
            Language::Chinese => load_chinese_messages(self),
        }
    }
}

#[derive(Debug)]
pub struct DictionaryManager {
    dictionaries: HashMap<Language, ErrorDictionary>,
    fallback_language: Language,
}

impl DictionaryManager {
    pub fn new() -> Self {
        let mut manager = Self {
            dictionaries: HashMap::new(),
            fallback_language: Language::English,
        };

        manager.load_language(Language::English);
        manager.load_language(Language::French);

        manager
    }

    pub fn with_fallback_language(fallback: Language) -> Self {
        let mut manager = Self {
            dictionaries: HashMap::new(),
            fallback_language: fallback,
        };
        manager.load_language(Language::English);
        manager.load_language(Language::French);
        manager
    }

    pub fn load_language(&mut self, language: Language) -> &mut Self {
        self.dictionaries
            .insert(language, ErrorDictionary::new(language));
        self
    }

    pub fn get_dictionary(&self, language: Language) -> Option<&ErrorDictionary> {
        self.dictionaries.get(&language)
    }

    pub fn get_message(
        &self,
        language: Language,
        key: &str,
        params: Option<&HashMap<String, String>>,
        count: Option<i64>,
    ) -> String {
        if let Some(dict) = self.get_dictionary(language)
            && dict.get_message(key).is_some()
        {
            return dict.get_formatted_message(key, params, count);
        }

        if language != self.fallback_language
            && let Some(dict) = self.get_dictionary(self.fallback_language)
            && dict.get_message(key).is_some()
        {
            return dict.get_formatted_message(key, params, count);
        }

        format!("Missing translation: {}", key)
    }

    pub fn translate_error(&self, error: &ModmailError, language: Language) -> String {
        let (key, params) = self.error_to_key_and_params(error);
        self.get_message(language, &key, params.as_ref(), None)
    }

    fn error_to_key_and_params(
        &self,
        error: &ModmailError,
    ) -> (String, Option<HashMap<String, String>>) {
        match error {
            ModmailError::Database(db_err) => match db_err {
                DatabaseError::ConnectionFailed => ("database.connection_failed".to_string(), None),
                DatabaseError::QueryFailed(query) => {
                    let mut params = HashMap::new();
                    params.insert("error".to_string(), query.clone());
                    ("database.query_failed".to_string(), Some(params))
                }
                DatabaseError::NotFound(_) => ("database.not_found".to_string(), None),
                _ => ("database.query_failed".to_string(), None),
            },
            ModmailError::Discord(discord_err) => match discord_err {
                DiscordError::UserNotFound => ("discord.user_not_found".to_string(), None),
                DiscordError::UserIsABot => ("discord.user_is_a_bot".to_string(), None),
                DiscordError::ShardManagerNotFound => {
                    ("discord.shard_manager_not_found".to_string(), None)
                }
                _ => ("discord.api_error".to_string(), None),
            },
            ModmailError::Command(cmd_err) => match cmd_err {
                CommandError::InvalidFormat => ("command.invalid_format".to_string(), None),
                CommandError::MissingArguments => ("command.missing_arguments".to_string(), None),
                CommandError::InvalidArguments(args) => {
                    let mut params = HashMap::new();
                    params.insert("arguments".to_string(), args.clone());
                    ("command.invalid_arguments".to_string(), Some(params))
                }
                CommandError::UnknownCommand(cmd) => {
                    let mut params = HashMap::new();
                    params.insert("command".to_string(), cmd.clone());
                    ("command.unknown_command".to_string(), Some(params))
                }
                CommandError::UserHasAlreadyAThreadWithLink(user, channel_id) => {
                    let mut params = HashMap::new();
                    params.insert("user".to_string(), user.clone());
                    params.insert("channel_id".to_string(), channel_id.clone());
                    (
                        "new_thread.user_has_thread_with_link".to_string(),
                        Some(params),
                    )
                }
                CommandError::ClosureAlreadyScheduled => {
                    ("close.closure_already_scheduled".to_string(), None)
                }
                CommandError::NoSchedulableClosureToCancel => {
                    ("close.no_scheduled_closures_to_cancel".to_string(), None)
                }
                CommandError::SendDmFailed => ("reply.send_failed_dm".to_string(), None),
                CommandError::DiscordDeleteFailed => {
                    ("command.discord_delete_failed".to_string(), None)
                }
                CommandError::ReminderAlreadyExists => {
                    ("reminder.reminder_already_exists".to_string(), None)
                }
                CommandError::NotInThread() => ("command.not_in_thread".to_string(), None),
                CommandError::AlertDoesNotExist => ("alert.alert_not_found".to_string(), None),
                CommandError::InvalidReminderFormat => ("add_reminder.helper".to_string(), None),
                CommandError::TicketAlreadyTaken => ("take.ticket_already_taken".to_string(), None),
                CommandError::TicketAlreadyReleased => {
                    ("release.ticket_already_taken".to_string(), None)
                }
                CommandError::AlertSetFailed => ("alert.alert_set_failed".to_string(), None),
                CommandError::SnippetNotFound(snippet) => {
                    let mut params = HashMap::new();
                    params.insert("key".to_string(), snippet.clone());
                    ("snippet.not_found".to_string(), Some(params))
                }
                CommandError::InvalidSnippetKeyFormat => {
                    ("snippet.invalid_key_format".to_string(), None)
                }
                CommandError::SnippetContentTooLong => {
                    ("snippet.content_too_long".to_string(), None)
                }
                CommandError::SnippetAlreadyExists(snippet) => {
                    let mut params = HashMap::new();
                    params.insert("key".to_string(), snippet.clone());
                    ("snippet.already_exist".to_string(), Some(params))
                }
                CommandError::StatusIsMissing => ("status.status_is_missing".to_string(), None),
                CommandError::InvalidStatusValue => ("status.invalid_status".to_string(), None),
                CommandError::MaintenanceModeNotAllowed => {
                    ("status.maintenance_not_allowed".to_string(), None)
                }
                CommandError::ReminderAlreadySubscribed(role) => {
                    let mut params = HashMap::new();
                    params.insert("role".to_string(), role.clone());
                    (
                        "reminder_subscription.already_subscribed".to_string(),
                        Some(params),
                    )
                }
                CommandError::ReminderAlreadyUnsubscribed(role) => {
                    let mut params = HashMap::new();
                    params.insert("role".to_string(), role.clone());
                    (
                        "reminder_subscription.already_unsubscribed".to_string(),
                        Some(params),
                    )
                }
                CommandError::ReminderRoleRequired(role) => {
                    let mut params = HashMap::new();
                    params.insert("role".to_string(), role.clone());
                    (
                        "reminder_subscription.role_required".to_string(),
                        Some(params),
                    )
                }
                CommandError::ReminderRoleNotFound(role) => {
                    let mut params = HashMap::new();
                    params.insert("role".to_string(), role.clone());
                    (
                        "reminder_subscription.role_not_found".to_string(),
                        Some(params),
                    )
                }
                CommandError::ReminderAlreadyCompleted(reminder_id) => {
                    let mut params = HashMap::new();
                    params.insert("reminder_id".to_string(), reminder_id.clone());
                    ("reminder.already_complete".to_string(), Some(params))
                }
                _ => ("command.invalid_format".to_string(), None),
            },
            ModmailError::Thread(thread_err) => match thread_err {
                ThreadError::ThreadNotFound => ("thread.not_found".to_string(), None),
                ThreadError::UserStillInServer => ("thread.user_still_in_server".to_string(), None),
                ThreadError::NotAThreadChannel => ("thread.not_a_thread_channel".to_string(), None),
                ThreadError::CategoryNotFound => ("thread.category_not_found".to_string(), None),
                _ => ("thread.not_found".to_string(), None),
            },
            ModmailError::Message(msg_err) => match msg_err {
                MessageError::MessageNotFound(msg) => {
                    let mut params = HashMap::new();
                    params.insert("message".to_string(), msg.to_string());
                    ("message.not_found".to_string(), Some(params))
                }
                MessageError::EditFailed(_) => ("message.edit_failed".to_string(), None),
                MessageError::MessageEmpty => ("message.empty".to_string(), None),
                _ => ("message.not_found".to_string(), None),
            },
            ModmailError::Validation(validation_err) => match validation_err {
                ValidationError::InvalidInput(input) => {
                    let mut params = HashMap::new();
                    params.insert("input".to_string(), input.clone());
                    ("validation.invalid_input".to_string(), Some(params))
                }
                ValidationError::RequiredFieldMissing(field) => {
                    let mut params = HashMap::new();
                    params.insert("field".to_string(), field.clone());
                    (
                        "validation.required_field_missing".to_string(),
                        Some(params),
                    )
                }
            },
            ModmailError::Permission(_) => {
                ("permission.insufficient_permissions".to_string(), None)
            }
            ModmailError::Config(_) => ("config.invalid_configuration".to_string(), None),
            ModmailError::Generic(msg) => {
                let mut params = HashMap::new();
                params.insert("message".to_string(), msg.clone());
                ("general.unknown_error".to_string(), Some(params))
            }
        }
    }
}

impl Default for DictionaryManager {
    fn default() -> Self {
        Self::new()
    }
}
