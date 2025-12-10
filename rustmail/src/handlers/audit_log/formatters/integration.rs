use super::{EmbedField, format_change_value};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::IntegrationAction;

pub struct IntegrationFormatter(pub IntegrationAction);

#[async_trait::async_trait]
impl AuditLogFormatter for IntegrationFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            IntegrationAction::Create => "🔗",
            IntegrationAction::Update => "✏️",
            IntegrationAction::Delete => "🗑️",
            _ => "🔗",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            IntegrationAction::Create => COLOR_SUCCESS,
            IntegrationAction::Update => COLOR_WARNING,
            IntegrationAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            IntegrationAction::Create => "audit_log.integration.create",
            IntegrationAction::Update => "audit_log.integration.update",
            IntegrationAction::Delete => "audit_log.integration.delete",
            _ => "audit_log.integration.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        let target_label = alc.translate("audit_log.target", None).await;

        let target = if let Some(target_id) = alc.target_id() {
            format!("`{}`", target_id)
        } else {
            alc.translate("audit_log.unknown", None).await
        };

        format!("**{}:** {}", target_label, target)
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
