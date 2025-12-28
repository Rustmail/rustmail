use super::{EmbedField, format_change_value};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::ChannelAction;

pub struct ChannelFormatter(pub ChannelAction);

#[async_trait::async_trait]
impl AuditLogFormatter for ChannelFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            ChannelAction::Create => "📁",
            ChannelAction::Update => "✏️",
            ChannelAction::Delete => "🗑️",
            _ => "📁",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            ChannelAction::Create => COLOR_SUCCESS,
            ChannelAction::Update => COLOR_WARNING,
            ChannelAction::Delete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            ChannelAction::Create => "audit_log.channel.create",
            ChannelAction::Update => "audit_log.channel.update",
            ChannelAction::Delete => "audit_log.channel.delete",
            _ => "audit_log.channel.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        let target_label = alc.translate("audit_log.target", None).await;

        let target = if let Some(target_id) = alc.target_id() {
            match self.0 {
                ChannelAction::Delete => format!("`{}`", target_id),
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
