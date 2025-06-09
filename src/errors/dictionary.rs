use crate::errors::types::*;
use crate::i18n::languages::{Language, PluralForm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            Language::English => self.load_english_messages(),
            Language::French => self.load_french_messages(),
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

    fn load_english_messages(&mut self) {
        self.messages.insert(
            "database.connection_failed".to_string(),
            ErrorMessage::new("Failed to connect to the database")
                .with_description("The bot couldn't establish a connection to the database")
                .with_help(
                    "Check database configuration and ensure the database server is running",
                ),
        );

        self.messages.insert(
            "database.query_failed".to_string(),
            ErrorMessage::new("Database query failed: {error}")
                .with_description("A database operation failed unexpectedly"),
        );

        self.messages.insert(
            "database.not_found".to_string(),
            ErrorMessage::new("Record not found in database")
                .with_description("The requested data could not be found"),
        );

        self.messages.insert(
            "discord.channel_not_found".to_string(),
            ErrorMessage::new("Channel not found").with_description(
                "The specified channel doesn't exist or the bot doesn't have access to it",
            ),
        );

        self.messages.insert(
            "discord.user_not_found".to_string(),
            ErrorMessage::new("User not found")
                .with_description("The specified user doesn't exist or is not accessible"),
        );

        self.messages.insert(
            "discord.permission_denied".to_string(),
            ErrorMessage::new("Permission denied").with_description(
                "The bot doesn't have the required permissions to perform this action",
            ),
        );

        self.messages.insert(
            "discord.dm_creation_failed".to_string(),
            ErrorMessage::new("Failed to create DM channel")
                .with_description("Couldn't create a direct message channel with the user"),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Invalid command format")
                .with_description("The command syntax is incorrect")
                .with_help("Use `{prefix}help` to see the correct command format"),
        );

        self.messages.insert(
            "command.missing_arguments".to_string(),
            ErrorMessage::new("Missing required arguments")
                .with_description("This command requires additional parameters"),
        );

        self.messages.insert(
            "command.invalid_arguments".to_string(),
            ErrorMessage::new("Invalid arguments: {arguments}")
                .with_description("One or more arguments are invalid"),
        );

        self.messages.insert(
            "command.unknown_command".to_string(),
            ErrorMessage::new("Unknown command: {command}")
                .with_description("The specified command doesn't exist")
                .with_help("Use `{prefix}help` to see available commands"),
        );

        self.messages.insert(
            "command.insufficient_permissions".to_string(),
            ErrorMessage::new("Insufficient permissions")
                .with_description("You don't have the required permissions to use this command"),
        );

        self.messages.insert(
            "thread.not_found".to_string(),
            ErrorMessage::new("Thread not found")
                .with_description("No active thread found for this user or channel"),
        );

        self.messages.insert(
            "thread.already_exists".to_string(),
            ErrorMessage::new("Thread already exists")
                .with_description("You already have an active support thread"),
        );

        self.messages.insert(
            "thread.creation_failed".to_string(),
            ErrorMessage::new("Failed to create thread")
                .with_description("An error occurred while creating the support thread"),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Message not found")
                .with_description("The specified message could not be found"),
        );

        self.messages.insert(
            "message.number_not_found".to_string(),
            ErrorMessage::new("Message #{number} not found")
                .with_description("No message with this number exists"),
        );

        self.messages.insert(
            "message.edit_failed".to_string(),
            ErrorMessage::new("Failed to edit message")
                .with_description("An error occurred while editing the message"),
        );

        self.messages.insert(
            "message.send_failed".to_string(),
            ErrorMessage::new("Failed to send message")
                .with_description("An error occurred while sending the message"),
        );

        self.messages.insert(
            "message.too_long".to_string(),
            ErrorMessage::new("Message is too long")
                .with_description("Discord messages cannot exceed 2000 characters"),
        );

        self.messages.insert(
            "message.empty".to_string(),
            ErrorMessage::new("Message cannot be empty")
                .with_description("Please provide a message to send"),
        );

        self.messages.insert(
            "validation.invalid_input".to_string(),
            ErrorMessage::new("Invalid input: {input}")
                .with_description("The provided input is not valid"),
        );

        self.messages.insert(
            "validation.out_of_range".to_string(),
            ErrorMessage::new("Value out of range: {range}")
                .with_description("The value must be within the specified range"),
        );

        self.messages.insert(
            "validation.required_field_missing".to_string(),
            ErrorMessage::new("Required field missing: {field}")
                .with_description("This field is required and cannot be empty"),
        );

        self.messages.insert(
            "permission.not_staff_member".to_string(),
            ErrorMessage::new("You are not a staff member")
                .with_description("This command is only available to staff members"),
        );

        self.messages.insert(
            "permission.user_blocked".to_string(),
            ErrorMessage::new("User is blocked")
                .with_description("This user has been blocked from using the support system"),
        );

        self.messages.insert(
            "success.message_sent".to_string(),
            ErrorMessage::new("Message sent successfully! (Message #{number})")
                .with_description("Your message has been delivered")
                .with_help("Use `{prefix}edit {number}` to modify this message"),
        );

        self.messages.insert(
            "success.message_edited".to_string(),
            ErrorMessage::new("Message edited successfully")
                .with_description("The message has been updated in both the thread and DM"),
        );

        self.messages.insert(
            "success.thread_created".to_string(),
            ErrorMessage::new("Support thread created")
                .with_description("A new support thread has been created for you"),
        );

        self.messages.insert(
            "general.loading".to_string(),
            ErrorMessage::new("Loading...")
                .with_description("Please wait while the operation completes"),
        );

        self.messages.insert(
            "general.processing".to_string(),
            ErrorMessage::new("Processing your request...")
                .with_description("This may take a few moments"),
        );
    }

    fn load_french_messages(&mut self) {
        self.messages.insert("database.connection_failed".to_string(),
            ErrorMessage::new("Échec de connexion à la base de données")
                .with_description("Le bot n'a pas pu établir une connexion à la base de données")
                .with_help("Vérifiez la configuration de la base de données et assurez-vous que le serveur est en marche"));

        self.messages.insert(
            "database.query_failed".to_string(),
            ErrorMessage::new("Échec de la requête de base de données : {error}").with_description(
                "Une opération de base de données a échoué de manière inattendue",
            ),
        );

        self.messages.insert(
            "database.not_found".to_string(),
            ErrorMessage::new("Enregistrement non trouvé dans la base de données")
                .with_description("Les données demandées n'ont pas pu être trouvées"),
        );

        self.messages.insert(
            "discord.channel_not_found".to_string(),
            ErrorMessage::new("Canal non trouvé")
                .with_description("Le canal spécifié n'existe pas ou le bot n'y a pas accès"),
        );

        self.messages.insert(
            "discord.user_not_found".to_string(),
            ErrorMessage::new("Utilisateur non trouvé")
                .with_description("L'utilisateur spécifié n'existe pas ou n'est pas accessible"),
        );

        self.messages.insert(
            "discord.permission_denied".to_string(),
            ErrorMessage::new("Permission refusée").with_description(
                "Le bot n'a pas les permissions requises pour effectuer cette action",
            ),
        );

        self.messages.insert(
            "discord.dm_creation_failed".to_string(),
            ErrorMessage::new("Échec de création du canal DM").with_description(
                "Impossible de créer un canal de message privé avec l'utilisateur",
            ),
        );

        self.messages.insert(
            "command.invalid_format".to_string(),
            ErrorMessage::new("Format de commande invalide")
                .with_description("La syntaxe de la commande est incorrecte")
                .with_help("Utilisez `{prefix}help` pour voir le format correct de la commande"),
        );

        self.messages.insert(
            "command.missing_arguments".to_string(),
            ErrorMessage::new("Arguments requis manquants")
                .with_description("Cette commande nécessite des paramètres supplémentaires"),
        );

        self.messages.insert(
            "command.invalid_arguments".to_string(),
            ErrorMessage::new("Arguments invalides : {arguments}")
                .with_description("Un ou plusieurs arguments sont invalides"),
        );

        self.messages.insert(
            "command.unknown_command".to_string(),
            ErrorMessage::new("Commande inconnue : {command}")
                .with_description("La commande spécifiée n'existe pas")
                .with_help("Utilisez `{prefix}help` pour voir les commandes disponibles"),
        );

        self.messages.insert(
            "command.insufficient_permissions".to_string(),
            ErrorMessage::new("Permissions insuffisantes").with_description(
                "Vous n'avez pas les permissions requises pour utiliser cette commande",
            ),
        );

        self.messages.insert(
            "thread.not_found".to_string(),
            ErrorMessage::new("Thread non trouvé")
                .with_description("Aucun thread actif trouvé pour cet utilisateur ou ce canal"),
        );

        self.messages.insert(
            "thread.already_exists".to_string(),
            ErrorMessage::new("Thread existe déjà")
                .with_description("Vous avez déjà un thread de support actif"),
        );

        self.messages.insert(
            "thread.creation_failed".to_string(),
            ErrorMessage::new("Échec de création du thread").with_description(
                "Une erreur s'est produite lors de la création du thread de support",
            ),
        );

        self.messages.insert(
            "message.not_found".to_string(),
            ErrorMessage::new("Message non trouvé")
                .with_description("Le message spécifié n'a pas pu être trouvé"),
        );

        self.messages.insert(
            "message.number_not_found".to_string(),
            ErrorMessage::new("Message #{number} non trouvé")
                .with_description("Aucun message avec ce numéro n'existe"),
        );

        self.messages.insert(
            "message.edit_failed".to_string(),
            ErrorMessage::new("Échec de modification du message")
                .with_description("Une erreur s'est produite lors de la modification du message"),
        );

        self.messages.insert(
            "message.send_failed".to_string(),
            ErrorMessage::new("Échec d'envoi du message")
                .with_description("Une erreur s'est produite lors de l'envoi du message"),
        );

        self.messages.insert(
            "message.too_long".to_string(),
            ErrorMessage::new("Message trop long")
                .with_description("Les messages Discord ne peuvent pas dépasser 2000 caractères"),
        );

        self.messages.insert(
            "message.empty".to_string(),
            ErrorMessage::new("Le message ne peut pas être vide")
                .with_description("Veuillez fournir un message à envoyer"),
        );

        self.messages.insert(
            "success.message_sent".to_string(),
            ErrorMessage::new("Message envoyé avec succès ! (Message #{number})")
                .with_description("Votre message a été livré")
                .with_help("Utilisez `{prefix}edit {number}` pour modifier ce message"),
        );

        self.messages.insert(
            "success.message_edited".to_string(),
            ErrorMessage::new("Message modifié avec succès")
                .with_description("Le message a été mis à jour dans le thread et en DM"),
        );

        self.messages.insert(
            "success.thread_created".to_string(),
            ErrorMessage::new("Thread de support créé")
                .with_description("Un nouveau thread de support a été créé pour vous"),
        );
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
