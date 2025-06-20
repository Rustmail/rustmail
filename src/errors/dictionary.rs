use crate::errors::types::*;
use crate::i18n::languages::{Language, PluralForm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::i18n::language::en::load_english_messages;
use crate::i18n::language::fr::load_french_messages;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDictionary {
    pub language: Language,
    pub messages: HashMap<String, ErrorMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub default: String,
    pub plurals: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub help: Option<String>,
}

impl ErrorMessage {
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

    pub fn with_plurals(mut self, plurals: HashMap<String, String>) -> Self {
        self.plurals = Some(plurals);
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

    pub fn get_message(&self, key: &str) -> Option<&ErrorMessage> {
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
            Language::Spanish => self.load_spanish_messages(),
            Language::German => self.load_german_messages(),
            Language::Italian => self.load_italian_messages(),
            Language::Portuguese => self.load_portuguese_messages(),
            Language::Dutch => self.load_dutch_messages(),
            Language::Russian => self.load_russian_messages(),
            Language::Japanese => self.load_japanese_messages(),
            Language::Korean => self.load_korean_messages(),
            Language::Chinese => self.load_chinese_messages(),
        }
    }

    fn load_spanish_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("Error al conectar con la base de datos"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Formato de comando inválido"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Mensaje no encontrado"),
        );
    }

    fn load_german_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("Datenbankverbindung fehlgeschlagen"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Ungültiges Befehlsformat"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Nachricht nicht gefunden"),
        );
    }

    fn load_italian_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("Connessione al database fallita"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Formato comando non valido"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Messaggio non trovato"),
        );
    }

    fn load_portuguese_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("Falha na conexão com o banco de dados"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Formato de comando inválido"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Mensagem não encontrada"),
        );
    }

    fn load_dutch_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("Databaseverbinding mislukt"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Ongeldig commandoformaat"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Bericht niet gevonden"),
        );
    }

    fn load_russian_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("Не удалось подключиться к базе данных"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Неверный формат команды"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Сообщение не найдено"),
        );
    }

    fn load_japanese_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("データベース接続に失敗しました"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("無効なコマンド形式"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("メッセージが見つかりません"),
        );
    }

    fn load_korean_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("데이터베이스 연결 실패"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("잘못된 명령어 형식"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("메시지를 찾을 수 없습니다"),
        );
    }

    fn load_chinese_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("数据库连接失败"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("命令格式无效"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("未找到消息"),
        );
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
        if let Some(dict) = self.get_dictionary(language) {
            if dict.get_message(key).is_some() {
                return dict.get_formatted_message(key, params, count);
            }
        }

        if language != self.fallback_language {
            if let Some(dict) = self.get_dictionary(self.fallback_language) {
                if dict.get_message(key).is_some() {
                    return dict.get_formatted_message(key, params, count);
                }
            }
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
                DiscordError::ChannelNotFound => ("discord.channel_not_found".to_string(), None),
                DiscordError::UserNotFound => ("discord.user_not_found".to_string(), None),
                DiscordError::PermissionDenied => ("discord.permission_denied".to_string(), None),
                DiscordError::DmCreationFailed => ("discord.dm_creation_failed".to_string(), None),
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
                CommandError::InsufficientPermissions => {
                    ("command.insufficient_permissions".to_string(), None)
                }
                _ => ("command.invalid_format".to_string(), None),
            },
            ModmailError::Thread(thread_err) => match thread_err {
                ThreadError::ThreadNotFound => ("thread.not_found".to_string(), None),
                ThreadError::ThreadAlreadyExists => ("thread.already_exists".to_string(), None),
                ThreadError::ThreadCreationFailed => ("thread.creation_failed".to_string(), None),
                _ => ("thread.not_found".to_string(), None),
            },
            ModmailError::Message(msg_err) => match msg_err {
                MessageError::MessageNotFound => ("message.not_found".to_string(), None),
                MessageError::MessageNumberNotFound(num) => {
                    let mut params = HashMap::new();
                    params.insert("number".to_string(), num.to_string());
                    ("message.number_not_found".to_string(), Some(params))
                }
                MessageError::EditFailed(_) => ("message.edit_failed".to_string(), None),
                MessageError::SendFailed(_) => ("message.send_failed".to_string(), None),
                MessageError::MessageTooLong => ("message.too_long".to_string(), None),
                MessageError::MessageEmpty => ("message.empty".to_string(), None),
                _ => ("message.not_found".to_string(), None),
            },
            ModmailError::Permission(perm_err) => match perm_err {
                PermissionError::NotStaffMember => {
                    ("permission.not_staff_member".to_string(), None)
                }
                PermissionError::UserBlocked => ("permission.user_blocked".to_string(), None),
                _ => ("permission.insufficient_permissions".to_string(), None),
            },
            _ => ("general.unknown_error".to_string(), None),
        }
    }
}

impl Default for DictionaryManager {
    fn default() -> Self {
        Self::new()
    }
}
