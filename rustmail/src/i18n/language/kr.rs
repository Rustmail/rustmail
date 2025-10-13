use crate::errors::{DictionaryMessage, ErrorDictionary};

pub fn load_korean_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("데이터베이스 연결 실패"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("잘못된 명령어 형식"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("메시지를 찾을 수 없습니다"),
    );
}
