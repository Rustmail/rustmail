use super::{EmbedField, format_change_value};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::InviteAction;

pub struct InviteFormatter(pub InviteAction);

#[async_trait::async_trait]
impl AuditLogFormatter for InviteFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            InviteAction::Create => "📨",
            InviteAction::Update => "✏️",
            InviteAction::Delete => "🗑️",
            _ => "📨",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            InviteAction::Create => COLOR_SUCCESS,
            InviteAction::Update => COLOR_WARNING,
            InviteAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            InviteAction::Create => "audit_log.invite.create",
            InviteAction::Update => "audit_log.invite.update",
            InviteAction::Delete => "audit_log.invite.delete",
            _ => "audit_log.invite.unknown",
        };
        alc.translate(key, None).await
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
