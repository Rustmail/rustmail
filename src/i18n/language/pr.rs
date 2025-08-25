use crate::errors::{DictionaryMessage, ErrorDictionary};

pub fn load_portuguese_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("Falha na conexão com o banco de dados"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("Formato de comando inválido"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("Mensagem não encontrada"),
    );
}
