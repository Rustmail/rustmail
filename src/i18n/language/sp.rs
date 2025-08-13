use crate::errors::{ErrorDictionary, ErrorMessage};

pub fn load_spanish_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("Error al conectar con la base de datos"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Formato de comando inválido"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Mensaje no encontrado"),
    );
}