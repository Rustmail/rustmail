use crate::prelude::errors::*;

pub fn load_chinese_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("数据库连接失败"),
    );

    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("命令格式无效"),
    );

    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("未找到消息"),
    );
}
