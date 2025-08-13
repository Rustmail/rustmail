use crate::errors::{ErrorDictionary, ErrorMessage};

pub fn load_portuguese_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("Falha na conexão com o banco de dados"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Formato de comando inválido"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Mensagem não encontrada"),
    );
}