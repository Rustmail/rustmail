use super::{format_change_value, EmbedField};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::RoleAction;

pub struct RoleFormatter(pub RoleAction);

#[async_trait::async_trait]
impl AuditLogFormatter for RoleFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            RoleAction::Create => "🏷️",
            RoleAction::Update => "✏️",
            RoleAction::Delete => "🗑️",
            _ => "🏷️",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            RoleAction::Create => COLOR_SUCCESS,
            RoleAction::Update => COLOR_WARNING,
            RoleAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            RoleAction::Create => "audit_log.role.create",
            RoleAction::Update => "audit_log.role.update",
            RoleAction::Delete => "audit_log.role.delete",
            _ => "audit_log.role.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        let target_label = alc.translate("audit_log.target", None).await;

        let target = if let Some(target_id) = alc.target_id() {
            match self.0 {
                RoleAction::Delete => format!("`{}`", target_id),
                _ => format!("<@&{}> (`{}`)", target_id, target_id),
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
