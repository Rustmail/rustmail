pub mod languages;

pub use languages::{Language, LanguageDetector, LanguagePreferences, PluralForm, TextDirection};

use serde::{Deserialize, Serialize};
use serenity::all::UserId;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct I18nManager {
    default_language: Language,
    user_preferences: HashMap<UserId, LanguagePreferences>,
    guild_defaults: HashMap<u64, Language>,
}

impl I18nManager {
    pub fn new(default_language: Language) -> Self {
        Self {
            default_language,
            user_preferences: HashMap::new(),
            guild_defaults: HashMap::new(),
        }
    }

    pub fn set_user_language(&mut self, user_id: UserId, preferences: LanguagePreferences) {
        self.user_preferences.insert(user_id, preferences);
    }

    pub fn get_user_language(&self, user_id: UserId, guild_id: Option<u64>) -> Language {
        if let Some(prefs) = self.user_preferences.get(&user_id) {
            return prefs.primary;
        }

        if let Some(guild_id) = guild_id {
            if let Some(lang) = self.guild_defaults.get(&guild_id) {
                return *lang;
            }
        }

        self.default_language
    }

    pub fn set_guild_language(&mut self, guild_id: u64, language: Language) {
        self.guild_defaults.insert(guild_id, language);
    }

    pub fn get_guild_language(&self, guild_id: u64) -> Language {
        self.guild_defaults
            .get(&guild_id)
            .copied()
            .unwrap_or(self.default_language)
    }

    pub fn auto_detect_user_language(
        &mut self,
        user_id: UserId,
        discord_locale: Option<&str>,
    ) -> Language {
        if let Some(locale) = discord_locale {
            if let Some(detected_lang) = LanguageDetector::from_discord_locale(locale) {
                let preferences = LanguagePreferences::new(detected_lang);
                self.set_user_language(user_id, preferences);
                return detected_lang;
            }
        }
        self.default_language
    }

    pub fn get_best_language(
        &self,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
        discord_locale: Option<&str>,
    ) -> Language {
        if let Some(uid) = user_id {
            if let Some(prefs) = self.user_preferences.get(&uid) {
                return prefs.primary;
            }
        }

        if let Some(locale) = discord_locale {
            if let Some(detected) = LanguageDetector::from_discord_locale(locale) {
                return detected;
            }
        }

        if let Some(gid) = guild_id {
            if let Some(lang) = self.guild_defaults.get(&gid) {
                return *lang;
            }
        }

        self.default_language
    }

    pub fn format_timestamp(
        &self,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
        timestamp: std::time::SystemTime,
    ) -> String {
        let language = if let Some(uid) = user_id {
            self.get_user_language(uid, guild_id)
        } else {
            self.default_language
        };

        let date_format = language.date_format();
        let time_format = language.time_format();

        format!("Time: {:?}", timestamp)
    }

    pub fn get_plural_form(&self, language: Language, count: i64) -> PluralForm {
        language.plural_form(count)
    }

    pub fn get_configured_languages(&self) -> Vec<Language> {
        let mut languages = vec![self.default_language];

        for prefs in self.user_preferences.values() {
            if !languages.contains(&prefs.primary) {
                languages.push(prefs.primary);
            }
            if !languages.contains(&prefs.fallback) {
                languages.push(prefs.fallback);
            }
        }

        for &lang in self.guild_defaults.values() {
            if !languages.contains(&lang) {
                languages.push(lang);
            }
        }

        languages.sort_by_key(|l| l.code());
        languages.dedup();
        languages
    }

    pub fn export_user_preferences(&self) -> HashMap<String, LanguagePreferences> {
        self.user_preferences
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    pub fn import_user_preferences(
        &mut self,
        preferences: HashMap<String, LanguagePreferences>,
    ) -> Result<(), String> {
        for (user_id_str, prefs) in preferences {
            let user_id = user_id_str
                .parse::<u64>()
                .map_err(|_| format!("Invalid user ID: {}", user_id_str))?;
            self.user_preferences.insert(UserId::new(user_id), prefs);
        }
        Ok(())
    }

    pub fn get_language_stats(&self) -> LanguageStats {
        let mut stats = LanguageStats::new();

        stats.default_language = self.default_language;
        stats.total_users = self.user_preferences.len();
        stats.total_guilds = self.guild_defaults.len();

        for prefs in self.user_preferences.values() {
            *stats.user_languages.entry(prefs.primary).or_insert(0) += 1;
        }

        for &lang in self.guild_defaults.values() {
            *stats.guild_languages.entry(lang).or_insert(0) += 1;
        }

        stats
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageStats {
    pub default_language: Language,
    pub total_users: usize,
    pub total_guilds: usize,
    pub user_languages: HashMap<Language, usize>,
    pub guild_languages: HashMap<Language, usize>,
}

impl LanguageStats {
    fn new() -> Self {
        Self {
            default_language: Language::English,
            total_users: 0,
            total_guilds: 0,
            user_languages: HashMap::new(),
            guild_languages: HashMap::new(),
        }
    }

    pub fn most_popular_user_language(&self) -> Option<Language> {
        self.user_languages
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(&lang, _)| lang)
    }

    pub fn most_popular_guild_language(&self) -> Option<Language> {
        self.guild_languages
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(&lang, _)| lang)
    }

    pub fn user_language_diversity(&self) -> usize {
        self.user_languages.len()
    }

    pub fn guild_language_diversity(&self) -> usize {
        self.guild_languages.len()
    }
}

pub mod utils {
    use super::*;

    pub fn format_with_plural(
        language: Language,
        singular: &str,
        plural: &str,
        count: i64,
    ) -> String {
        let form = language.plural_form(count);
        match form {
            PluralForm::One => singular.replace("{count}", &count.to_string()),
            _ => plural.replace("{count}", &count.to_string()),
        }
    }

    pub fn get_direction_class(language: Language) -> &'static str {
        match language.direction() {
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
        }
    }

    pub fn detect_language_hints(text: &str) -> Vec<Language> {
        let mut hints = Vec::new();

        if text.contains("the ") || text.contains("and ") || text.contains("is ") {
            hints.push(Language::English);
        }

        if text.contains("le ") || text.contains("la ") || text.contains("est ") {
            hints.push(Language::French);
        }

        if text.contains("el ") || text.contains("la ") || text.contains("es ") {
            hints.push(Language::Spanish);
        }

        if text.contains("der ") || text.contains("die ") || text.contains("das ") {
            hints.push(Language::German);
        }

        hints
    }

    pub fn truncate_text(text: &str, max_length: usize, language: Language) -> String {
        if text.len() <= max_length {
            return text.to_string();
        }

        let ellipsis = match language {
            Language::Japanese | Language::Chinese => "…",
            _ => "...",
        };

        let truncate_to = max_length.saturating_sub(ellipsis.len());

        match language.direction() {
            TextDirection::LeftToRight => {
                if let Some(pos) = text[..truncate_to].rfind(' ') {
                    format!("{}{}", &text[..pos], ellipsis)
                } else {
                    format!("{}{}", &text[..truncate_to], ellipsis)
                }
            }
            TextDirection::RightToLeft => {
                format!("{}{}", &text[..truncate_to], ellipsis)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::all::UserId;

    #[test]
    fn test_i18n_manager_basic() {
        let mut manager = I18nManager::new(Language::English);
        let user_id = UserId::new(12345);

        assert_eq!(manager.get_user_language(user_id, None), Language::English);

        let prefs = LanguagePreferences::new(Language::French);
        manager.set_user_language(user_id, prefs);
        assert_eq!(manager.get_user_language(user_id, None), Language::French);
    }

    #[test]
    fn test_guild_language() {
        let mut manager = I18nManager::new(Language::English);
        let guild_id = 67890;

        assert_eq!(manager.get_guild_language(guild_id), Language::English);

        manager.set_guild_language(guild_id, Language::Spanish);
        assert_eq!(manager.get_guild_language(guild_id), Language::Spanish);
    }

    #[test]
    fn test_auto_detect_language() {
        let mut manager = I18nManager::new(Language::English);
        let user_id = UserId::new(12345);

        let detected = manager.auto_detect_user_language(user_id, Some("fr"));
        assert_eq!(detected, Language::French);
        assert_eq!(manager.get_user_language(user_id, None), Language::French);
    }

    #[test]
    fn test_best_language_priority() {
        let mut manager = I18nManager::new(Language::English);
        let user_id = UserId::new(12345);
        let guild_id = 67890;

        assert_eq!(
            manager.get_best_language(Some(user_id), Some(guild_id), None),
            Language::English
        );

        manager.set_guild_language(guild_id, Language::German);
        assert_eq!(
            manager.get_best_language(Some(user_id), Some(guild_id), None),
            Language::German
        );

        assert_eq!(
            manager.get_best_language(Some(user_id), Some(guild_id), Some("es-ES")),
            Language::Spanish
        );

        let prefs = LanguagePreferences::new(Language::French);
        manager.set_user_language(user_id, prefs);
        assert_eq!(
            manager.get_best_language(Some(user_id), Some(guild_id), Some("es-ES")),
            Language::French
        );
    }

    #[test]
    fn test_language_stats() {
        let mut manager = I18nManager::new(Language::English);

        manager.set_user_language(UserId::new(1), LanguagePreferences::new(Language::French));
        manager.set_user_language(UserId::new(2), LanguagePreferences::new(Language::French));
        manager.set_user_language(UserId::new(3), LanguagePreferences::new(Language::Spanish));

        manager.set_guild_language(100, Language::German);
        manager.set_guild_language(200, Language::French);

        let stats = manager.get_language_stats();

        assert_eq!(stats.total_users, 3);
        assert_eq!(stats.total_guilds, 2);
        assert_eq!(stats.user_languages.get(&Language::French), Some(&2));
        assert_eq!(stats.user_languages.get(&Language::Spanish), Some(&1));
        assert_eq!(stats.most_popular_user_language(), Some(Language::French));
    }

    #[test]
    fn test_utils_plural() {
        let result =
            utils::format_with_plural(Language::English, "{count} item", "{count} items", 1);
        assert_eq!(result, "1 item");

        let result =
            utils::format_with_plural(Language::English, "{count} item", "{count} items", 5);
        assert_eq!(result, "5 items");
    }

    #[test]
    fn test_utils_truncate() {
        let text = "This is a long text that needs to be truncated";
        let result = utils::truncate_text(text, 20, Language::English);
        assert!(result.len() <= 20);
        assert!(result.ends_with("..."));
    }
}
