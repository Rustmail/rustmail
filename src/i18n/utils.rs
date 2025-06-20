use crate::config::Config;
use serenity::all::UserId;
use std::collections::HashMap;

/// Récupère un message traduit à partir d'une clé, des paramètres, et du contexte utilisateur/guild.
/// - `config` : la configuration globale (pour accéder à ErrorHandler)
/// - `key` : la clé du message dans le dictionnaire
/// - `params` : les paramètres à injecter dans le message (optionnel)
/// - `user_id` : l'utilisateur cible (optionnel)
/// - `guild_id` : la guilde cible (optionnel)
/// - `count` : pour la gestion du pluriel (optionnel)
pub async fn get_translated_message(
    config: &Config,
    key: &str,
    params: Option<&HashMap<String, String>>,
    user_id: Option<UserId>,
    guild_id: Option<u64>,
    count: Option<i64>,
) -> String {
    if let Some(error_handler) = &config.error_handler {
        // On récupère la langue de l'utilisateur/guilde
        let language = if let Some(uid) = user_id {
            error_handler.get_user_language(uid, guild_id).await
        } else {
            // Si pas d'utilisateur, on prend la langue par défaut
            error_handler.get_user_language(UserId::new(0), None).await
        };
        error_handler.get_dictionary_message(language, key, params, count).await
    } else {
        // Fallback si jamais le handler n'est pas dispo
        format!("[{}]", key)
    }
} 