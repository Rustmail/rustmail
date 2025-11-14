use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LanguageConfig {
    pub default_language: String,
    pub fallback_language: String,
    pub supported_languages: Vec<String>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            default_language: "en".to_string(),
            fallback_language: "en".to_string(),
            supported_languages: vec!["en".to_string(), "fr".to_string()],
        }
    }
}
