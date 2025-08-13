use crate::errors::{ErrorDictionary, ErrorMessage};

pub fn load_russian_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("Не удалось подключиться к базе данных"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Неверный формат команды"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Сообщение не найдено"),
    );
}