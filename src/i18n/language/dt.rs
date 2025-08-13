use crate::errors::{ErrorDictionary, ErrorMessage};

pub fn load_dutch_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("Databaseverbinding mislukt"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Ongeldig commandoformaat"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Bericht niet gevonden"),
    );
}