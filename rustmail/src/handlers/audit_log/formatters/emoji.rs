use super::{EmbedField, format_change_value};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::EmojiAction;

pub struct EmojiFormatter(pub EmojiAction);

#[async_trait::async_trait]
impl AuditLogFormatter for EmojiFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            EmojiAction::Create => "😀",
            EmojiAction::Update => "✏️",
            EmojiAction::Delete => "🗑️",
            _ => "😀",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            EmojiAction::Create => COLOR_SUCCESS,
            EmojiAction::Update => COLOR_WARNING,
            EmojiAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            EmojiAction::Create => "audit_log.emoji.create",
            EmojiAction::Update => "audit_log.emoji.update",
            EmojiAction::Delete => "audit_log.emoji.delete",
            _ => "audit_log.emoji.unknown",
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
