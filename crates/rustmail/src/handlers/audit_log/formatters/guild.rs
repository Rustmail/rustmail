use super::{EmbedField, format_change_value};
use crate::handlers::audit_log::{AuditLogContext, AuditLogFormatter, COLOR_WARNING};

pub enum GuildFormatter {
    Update,
}

#[async_trait::async_trait]
impl AuditLogFormatter for GuildFormatter {
    fn emoji(&self) -> &'static str {
        "⚙️"
    }

    fn color(&self) -> u32 {
        COLOR_WARNING
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        alc.translate("audit_log.guild.update", None).await
    }

    async fn description(&self, _alc: &AuditLogContext<'_>) -> String {
        String::new()
    }

    async fn format_changes(&self, alc: &AuditLogContext<'_>) -> Vec<EmbedField> {
        let mut fields = Vec::new();
        for change in alc.changes() {
            if let Some(field) = format_change_value(alc, change).await {
                fields.push(field);
            }
        }
        fields
    }
}
