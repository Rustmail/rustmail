use crate::prelude::config::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use std::collections::HashMap;

pub async fn build_version_content(config: &Config) -> String {
    let mut params = HashMap::new();
    params.insert("version".to_string(), CURRENT_VERSION.to_string());
    params.insert("repository".to_string(), REPOSITORY_URL.to_string());

    get_translated_message(
        config,
        "version_command.current",
        Some(&params),
        None,
        None,
        None,
    )
    .await
}
