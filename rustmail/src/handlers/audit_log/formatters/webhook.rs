use super::{format_change_value, EmbedField};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::WebhookAction;

pub struct WebhookFormatter(pub WebhookAction);

#[async_trait::async_trait]
impl AuditLogFormatter for WebhookFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            WebhookAction::Create => "🪝",
            WebhookAction::Update => "✏️",
            WebhookAction::Delete => "🗑️",
            _ => "🪝",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            WebhookAction::Create => COLOR_SUCCESS,
            WebhookAction::Update => COLOR_WARNING,
            WebhookAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            WebhookAction::Create => "audit_log.webhook.create",
            WebhookAction::Update => "audit_log.webhook.update",
            WebhookAction::Delete => "audit_log.webhook.delete",
            _ => "audit_log.webhook.unknown",
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
