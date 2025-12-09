// ---------------------------------------------------------------------------- //
// Code integrated from: https://github.com/opensass/i18n-rs                    //
// Original Author: opensass                                                    //
// License: MIT                                                                 //
// Note: Internalized to allow Yew updates.                                     //
// ---------------------------------------------------------------------------- //

use serde_json::{self, Value};
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Configuration for the I18n module, specifying supported translations.
#[derive(Debug, Clone, PartialEq)]
pub struct I18nConfig {
    /// Mapping of language codes to raw JSON strings representing translation data.
    /// Example: `HashMap::from([("en", "{...}"), ("fr", "{...}")])`.
    pub translations: HashMap<&'static str, &'static str>,
}

/// Enum representing browser storage options for persisting the selected language.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum StorageType {
    /// Use the browser's `LocalStorage` for persisting data.
    #[default]
    LocalStorage,
    /// Use the browser's `SessionStorage` for persisting data.
    #[allow(dead_code)]
    SessionStorage,
}

/// This struct represents the state and methods for managing internationalization.
#[derive(Clone, PartialEq)]
pub struct I18n {
    /// Configuration for I18n, specifying supported translations.
    pub config: I18nConfig,
    /// The current language code being used for translations.
    current_language: String,
    /// Translations loaded for each supported language, represented as a mapping from
    /// language codes to JSON structures (`serde_json::Value`).
    translations: HashMap<String, Value>,
}

impl I18n {
    /// Initializes an `I18n` instance with a configuration and translations.
    ///
    /// # Arguments
    /// - `config`: The `I18nConfig` containing supported translations map.
    /// - `translations`: A `HashMap` containing language codes as keys and JSON strings as values.
    ///
    /// # Returns
    /// - `Ok(I18n)` if initialization is successful.
    /// - `Err(String)` if there is an error, such as missing translations or invalid JSON.
    pub fn new(config: I18nConfig, translations: HashMap<&str, &str>) -> Result<Self, String> {
        let translations = Self::load_translations(translations)?;

        let languages: Vec<&str> = translations
            .keys()
            .map(|arg: &String| String::as_str(arg))
            .collect();

        let current_language = languages
            .first()
            .cloned()
            .ok_or_else(|| "You must add at least one supported language".to_string())?;

        Ok(I18n {
            config,
            current_language: current_language.to_string(),
            translations,
        })
    }

    /// Loads translations for the given languages from a `HashMap` of raw JSON strings.
    ///
    /// # Arguments
    /// - `translations`: A `HashMap` containing language codes as keys and JSON strings as values.
    ///
    /// # Returns
    /// - `Ok(HashMap<String, Value>)` if all translations are valid.
    /// - `Err(String)` if any translation is missing or invalid.
    fn load_translations(
        translations: HashMap<&str, &str>,
    ) -> Result<HashMap<String, Value>, String> {
        let mut loaded_translations = HashMap::new();
        let languages: Vec<&str> = translations.keys().copied().collect();

        for language in &languages {
            if let Some(json_str) = translations.get(language) {
                let json: Value = serde_json::from_str(json_str)
                    .map_err(|err| format!("Invalid JSON for language {}: {}", language, err))?;
                loaded_translations.insert(language.to_string(), json);
            } else {
                return Err(format!("Translation data for '{}' not found", language));
            }
        }

        Ok(loaded_translations)
    }

    /// Sets the translation language and stores it in the browser's storage.
    ///
    /// # Arguments
    /// - `language`: The language code to set (e.g., `"en"`).
    /// - `storage_type`: The type of browser storage to use (`StorageType::LocalStorage` or `StorageType::SessionStorage`).
    /// - `storage_name`: The key to use for storing the selected language.
    ///
    /// # Returns
    /// - `Ok(())` if the language was successfully set.
    /// - `Err(String)` if the language is not supported or storage fails.
    pub fn set_translation_language(
        &mut self,
        language: &str,
        _storage_type: &StorageType,
        _storage_name: &str,
    ) -> Result<(), String> {
        let languages: Vec<&str> = self
            .translations
            .keys()
            .map(|arg: &String| arg.as_str())
            .collect();

        if !languages.contains(&language) {
            return Err(format!("Language '{}' is not supported", language));
        }

        self.current_language = language.to_string();

        #[cfg(target_arch = "wasm32")]
        {
            let result = match _storage_type {
                StorageType::LocalStorage => window()
                    .ok_or("No window available")?
                    .local_storage()
                    .map_err(|_| "Failed to access localStorage".to_string())?
                    .ok_or("localStorage not available")?
                    .set_item(_storage_name, language),
                StorageType::SessionStorage => window()
                    .ok_or("No window available")?
                    .session_storage()
                    .map_err(|_| "Failed to access sessionStorage".to_string())?
                    .ok_or("sessionStorage not available")?
                    .set_item(_storage_name, language),
            };

            result.map_err(|_| {
                format!(
                    "Failed to write to {}",
                    match _storage_type {
                        StorageType::LocalStorage => "LocalStorage",
                        StorageType::SessionStorage => "SessionStorage",
                    }
                )
            })?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // TODO: Add support for native
        }

        Ok(())
    }

    /// Retrieves the current language code.
    ///
    /// # Returns
    /// - A reference to the current language code as a `&str`.
    pub fn get_current_language(&self) -> &str {
        &self.current_language
    }

    /// Translates a given key using the current language.
    ///
    /// # Arguments
    /// - `key`: The translation key to retrieve (e.g., `"menu.file.open"`).
    ///
    /// # Returns
    /// - The translated string if the key exists.
    /// - A fallback message if the key or translation does not exist.
    pub fn t(&self, key: &str) -> String {
        let keys: Vec<&str> = key.split('.').collect();
        let languages: Vec<&str> = self.config.translations.keys().copied().collect();

        let first_language = languages[0];

        self.translations
            .get(&self.current_language)
            .and_then(|language_json| Self::get_nested_value(language_json, &keys))
            .or_else(|| {
                self.translations
                    .get(first_language)
                    .and_then(|default_json| Self::get_nested_value(default_json, &keys))
            })
            .map_or_else(
                || {
                    format!(
                        "Key '{}' not found for language '{}'",
                        key, self.current_language
                    )
                },
                |value| match value {
                    Value::String(s) => s.clone(),
                    _ => value.to_string(),
                },
            )
    }

    /// Retrieves a nested value from a JSON object using a sequence of keys.
    ///
    /// # Arguments
    /// - `json`: The root `serde_json::Value` object to search within.
    /// - `keys`: A slice of keys representing the path to the desired value.
    ///
    /// # Returns
    /// - `Some(&Value)` if the value exists at the specified path.
    /// - `None` if the path does not exist.
    fn get_nested_value<'a>(json: &'a Value, keys: &[&str]) -> Option<&'a Value> {
        keys.iter().try_fold(json, |current, key| current.get(key))
    }
}
