use crate::errors::{ErrorDictionary, ErrorMessage};

pub fn load_japanese_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("データベース接続に失敗しました"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("無効なコマンド形式"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("メッセージが見つかりません"),
    );
}