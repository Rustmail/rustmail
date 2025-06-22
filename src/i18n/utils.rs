use crate::config::Config;
use serenity::all::UserId;
use std::collections::HashMap;

pub async fn get_translated_message(
    config: &Config,
    key: &str,
    params: Option<&HashMap<String, String>>,
    user_id: Option<UserId>,
    guild_id: Option<u64>,
    count: Option<i64>,
) -> String {
    if let Some(error_handler) = &config.error_handler {
        let language = if let Some(uid) = user_id {
            error_handler.get_user_language(uid, guild_id).await
        } else {
            config.language.get_default_language()
        };
        error_handler.get_dictionary_message(language, key, params, count).await
    } else {
        format!("[{}]", key)
    }
} 