use serde::{Deserialize, Serialize};
use std::fmt;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    French,
    Spanish,
    German,
    Italian,
    Portuguese,
    Dutch,
    Russian,
    Japanese,
    Korean,
    Chinese,
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::French,
            Language::Spanish,
            Language::German,
            Language::Italian,
            Language::Portuguese,
            Language::Dutch,
            Language::Russian,
            Language::Japanese,
            Language::Korean,
            Language::Chinese,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Spanish => "es",
            Language::German => "de",
            Language::Italian => "it",
            Language::Portuguese => "pt",
            Language::Dutch => "nl",
            Language::Russian => "ru",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Chinese => "zh",
        }
    }

    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::German => "Deutsch",
            Language::Italian => "Italiano",
            Language::Portuguese => "Português",
            Language::Dutch => "Nederlands",
            Language::Russian => "Русский",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::Chinese => "中文",
        }
    }

    pub fn english_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "French",
            Language::Spanish => "Spanish",
            Language::German => "German",
            Language::Italian => "Italian",
            Language::Portuguese => "Portuguese",
            Language::Dutch => "Dutch",
            Language::Russian => "Russian",
            Language::Japanese => "Japanese",
            Language::Korean => "Korean",
            Language::Chinese => "Chinese",
        }
    }

    pub fn from_str(s: &str) -> Option<Language> {
        let s = s.to_lowercase();
        match s.as_str() {
            "en" | "english" | "eng" => Some(Language::English),
            "fr" | "french" | "français" | "francais" => Some(Language::French),
            "es" | "spanish" | "español" | "espanol" => Some(Language::Spanish),
            "de" | "german" | "deutsch" => Some(Language::German),
            "it" | "italian" | "italiano" => Some(Language::Italian),
            "pt" | "portuguese" | "português" | "portugues" => Some(Language::Portuguese),
            "nl" | "dutch" | "nederlands" => Some(Language::Dutch),
            "ru" | "russian" | "русский" => Some(Language::Russian),
            "ja" | "japanese" | "日本語" => Some(Language::Japanese),
            "ko" | "korean" | "한국어" => Some(Language::Korean),
            "zh" | "chinese" | "中文" => Some(Language::Chinese),
            _ => None,
        }
    }

    pub fn direction(&self) -> TextDirection {
        match self {
            Language::English
            | Language::French
            | Language::Spanish
            | Language::German
            | Language::Italian
            | Language::Portuguese
            | Language::Dutch
            | Language::Russian
            | Language::Japanese
            | Language::Korean
            | Language::Chinese => TextDirection::LeftToRight,
        }
    }

    pub fn date_format(&self) -> &'static str {
        match self {
            Language::English => "%m/%d/%Y",
            Language::French => "%d/%m/%Y",
            Language::Spanish => "%d/%m/%Y",
            Language::German => "%d.%m.%Y",
            Language::Italian => "%d/%m/%Y",
            Language::Portuguese => "%d/%m/%Y",
            Language::Dutch => "%d-%m-%Y",
            Language::Russian => "%d.%m.%Y",
            Language::Japanese => "%Y/%m/%d",
            Language::Korean => "%Y.%m.%d",
            Language::Chinese => "%Y-%m-%d",
        }
    }

    pub fn time_format(&self) -> &'static str {
        match self {
            Language::English => "%I:%M %p",
            Language::French => "%H:%M",
            Language::Spanish => "%H:%M",
            Language::German => "%H:%M",
            Language::Italian => "%H:%M",
            Language::Portuguese => "%H:%M",
            Language::Dutch => "%H:%M",
            Language::Russian => "%H:%M",
            Language::Japanese => "%H:%M",
            Language::Korean => "%H:%M",
            Language::Chinese => "%H:%M",
        }
    }

    pub fn plural_form(&self, count: i64) -> PluralForm {
        match self {
            Language::English => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            Language::French => {
                if count <= 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            Language::Spanish => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            Language::German => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            Language::Italian => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            Language::Portuguese => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            Language::Dutch => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            Language::Russian => {
                let n = count % 100;
                if n % 10 == 1 && n != 11 {
                    PluralForm::One
                } else if (2..=4).contains(&(n % 10)) && !(12..=14).contains(&n) {
                    PluralForm::Few
                } else {
                    PluralForm::Many
                }
            }
            Language::Japanese => PluralForm::Other,
            Language::Korean => PluralForm::Other,
            Language::Chinese => PluralForm::Other,
        }
    }

    pub fn flag_emoji(&self) -> &'static str {
        match self {
            Language::English => "🇺🇸",
            Language::French => "🇫🇷",
            Language::Spanish => "🇪🇸",
            Language::German => "🇩🇪",
            Language::Italian => "🇮🇹",
            Language::Portuguese => "🇵🇹",
            Language::Dutch => "🇳🇱",
            Language::Russian => "🇷🇺",
            Language::Japanese => "🇯🇵",
            Language::Korean => "🇰🇷",
            Language::Chinese => "🇨🇳",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.native_name())
    }
}

impl str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Language::from_str(s).ok_or_else(|| format!("Unknown language: {}", s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluralForm {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

impl PluralForm {
    pub fn as_str(&self) -> &'static str {
        match self {
            PluralForm::Zero => "zero",
            PluralForm::One => "one",
            PluralForm::Two => "two",
            PluralForm::Few => "few",
            PluralForm::Many => "many",
            PluralForm::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePreferences {
    pub primary: Language,
    pub fallback: Language,
    pub timezone: Option<String>,
    pub date_format: Option<String>,
    pub time_format: Option<String>,
}

impl Default for LanguagePreferences {
    fn default() -> Self {
        Self {
            primary: Language::English,
            fallback: Language::English,
            timezone: None,
            date_format: None,
            time_format: None,
        }
    }
}

impl LanguagePreferences {
    pub fn new(language: Language) -> Self {
        Self {
            primary: language,
            fallback: Language::English,
            timezone: None,
            date_format: None,
            time_format: None,
        }
    }

    pub fn with_fallback(language: Language, fallback: Language) -> Self {
        Self {
            primary: language,
            fallback,
            timezone: None,
            date_format: None,
            time_format: None,
        }
    }

    pub fn get_date_format(&self) -> &str {
        self.date_format
            .as_deref()
            .unwrap_or_else(|| self.primary.date_format())
    }

    pub fn get_time_format(&self) -> &str {
        self.time_format
            .as_deref()
            .unwrap_or_else(|| self.primary.time_format())
    }
}

pub struct LanguageDetector;

impl LanguageDetector {
    pub fn from_discord_locale(locale: &str) -> Option<Language> {
        match locale {
            "en-US" | "en-GB" => Some(Language::English),
            "fr" => Some(Language::French),
            "es-ES" => Some(Language::Spanish),
            "de" => Some(Language::German),
            "it" => Some(Language::Italian),
            "pt-BR" => Some(Language::Portuguese),
            "nl" => Some(Language::Dutch),
            "ru" => Some(Language::Russian),
            "ja" => Some(Language::Japanese),
            "ko" => Some(Language::Korean),
            "zh-CN" | "zh-TW" => Some(Language::Chinese),
            _ => None,
        }
    }

    pub fn get_preferred_language(
        user_preference: Option<Language>,
        discord_locale: Option<&str>,
        guild_default: Option<Language>,
    ) -> Language {
        if let Some(lang) = user_preference {
            return lang;
        }

        if let Some(locale) = discord_locale
            && let Some(lang) = Self::from_discord_locale(locale)
        {
            return lang;
        }

        if let Some(lang) = guild_default {
            return lang;
        }

        Language::English
    }
}

pub mod codes {
    pub const ENGLISH: &str = "en";
    pub const FRENCH: &str = "fr";
    pub const SPANISH: &str = "es";
    pub const GERMAN: &str = "de";
    pub const ITALIAN: &str = "it";
    pub const PORTUGUESE: &str = "pt";
    pub const DUTCH: &str = "nl";
    pub const RUSSIAN: &str = "ru";
    pub const JAPANESE: &str = "ja";
    pub const KOREAN: &str = "ko";
    pub const CHINESE: &str = "zh";
}
