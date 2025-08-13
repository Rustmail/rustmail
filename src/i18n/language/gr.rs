use crate::errors::{ErrorDictionary, ErrorMessage};

pub fn load_german_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("Datenbankverbindung fehlgeschlagen"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Ungültiges Befehlsformat"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Nachricht nicht gefunden"),
    );
}