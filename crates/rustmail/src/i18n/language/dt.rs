use crate::prelude::errors::*;

pub fn load_dutch_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("Databaseverbinding mislukt"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("Ongeldig commandoformaat"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("Bericht niet gevonden"),
    );
}
