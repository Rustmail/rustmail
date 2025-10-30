use crate::prelude::errors::*;

pub fn load_german_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("Datenbankverbindung fehlgeschlagen"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("Ungültiges Befehlsformat"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("Nachricht nicht gefunden"),
    );
}
