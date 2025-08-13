use crate::errors::{ErrorDictionary, ErrorMessage};

pub fn load_italian_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("Connessione al database fallita"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Formato comando non valido"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Messaggio non trovato"),
    );
}