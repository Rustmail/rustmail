use super::{format_change_value, EmbedField};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::ScheduledEventAction;

pub struct ScheduledEventFormatter(pub ScheduledEventAction);

#[async_trait::async_trait]
impl AuditLogFormatter for ScheduledEventFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            ScheduledEventAction::Create => "📅",
            ScheduledEventAction::Update => "✏️",
            ScheduledEventAction::Delete => "🗑️",
            _ => "📅",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            ScheduledEventAction::Create => COLOR_SUCCESS,
            ScheduledEventAction::Update => COLOR_WARNING,
            ScheduledEventAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            ScheduledEventAction::Create => "audit_log.scheduled_event.create",
            ScheduledEventAction::Update => "audit_log.scheduled_event.update",
            ScheduledEventAction::Delete => "audit_log.scheduled_event.delete",
            _ => "audit_log.scheduled_event.unknown",
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
