use crate::errors::{DictionaryMessage, ErrorDictionary};

pub fn load_spanish_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("Error al conectar con la base de datos"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("Formato de comando inválido"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("Mensaje no encontrado"),
    );
}
