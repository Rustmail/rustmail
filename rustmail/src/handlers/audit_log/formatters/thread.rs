use super::{format_change_value, EmbedField};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::ThreadAction;

pub struct ThreadFormatter(pub ThreadAction);

#[async_trait::async_trait]
impl AuditLogFormatter for ThreadFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            ThreadAction::Create => "🧵",
            ThreadAction::Update => "✏️",
            ThreadAction::Delete => "🗑️",
            _ => "🧵",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            ThreadAction::Create => COLOR_SUCCESS,
            ThreadAction::Update => COLOR_WARNING,
            ThreadAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            ThreadAction::Create => "audit_log.thread.create",
            ThreadAction::Update => "audit_log.thread.update",
            ThreadAction::Delete => "audit_log.thread.delete",
            _ => "audit_log.thread.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        let target_label = alc.translate("audit_log.target", None).await;

        let target = if let Some(target_id) = alc.target_id() {
            match self.0 {
                ThreadAction::Delete => format!("`{}`", target_id),
                _ => format!("<#{}> (`{}`)", target_id, target_id),
            }
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
