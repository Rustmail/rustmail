use crate::errors::{DictionaryMessage, ErrorDictionary};

pub fn load_italian_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("Connessione al database fallita"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("Formato comando non valido"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("Messaggio non trovato"),
    );
}
