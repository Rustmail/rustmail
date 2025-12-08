use super::EmbedField;
use crate::handlers::audit_log::{AuditLogContext, AuditLogFormatter, COLOR_NEUTRAL};
use std::collections::HashMap;

pub struct UnknownFormatter(pub u8);

#[async_trait::async_trait]
impl AuditLogFormatter for UnknownFormatter {
    fn emoji(&self) -> &'static str {
        "❓"
    }

    fn color(&self) -> u32 {
        COLOR_NEUTRAL
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let mut params = HashMap::new();
        params.insert("code".to_string(), self.0.to_string());
        alc.translate("audit_log.unknown_action", Some(&params))
            .await
    }

    async fn description(&self, _alc: &AuditLogContext<'_>) -> String {
        String::new()
    }

    async fn format_changes(&self, _alc: &AuditLogContext<'_>) -> Vec<EmbedField> {
        Vec::new()
    }
}
