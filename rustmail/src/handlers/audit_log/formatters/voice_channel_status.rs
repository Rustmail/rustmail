use super::EmbedField;
use crate::handlers::audit_log::{AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_WARNING};
use serenity::all::VoiceChannelStatusAction;

pub struct VoiceChannelStatusFormatter(pub VoiceChannelStatusAction);

#[async_trait::async_trait]
impl AuditLogFormatter for VoiceChannelStatusFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            VoiceChannelStatusAction::StatusUpdate => "🔊",
            VoiceChannelStatusAction::StatusDelete => "🔇",
            _ => "🔊",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            VoiceChannelStatusAction::StatusUpdate => COLOR_WARNING,
            VoiceChannelStatusAction::StatusDelete => COLOR_DANGER,
            _ => COLOR_WARNING,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            VoiceChannelStatusAction::StatusUpdate => "audit_log.voice_channel_status.update",
            VoiceChannelStatusAction::StatusDelete => "audit_log.voice_channel_status.delete",
            _ => "audit_log.voice_channel_status.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        let target_label = alc.translate("audit_log.target", None).await;

        let target = if let Some(target_id) = alc.target_id() {
            format!("<#{}> (`{}`)", target_id, target_id)
        } else {
            alc.translate("audit_log.unknown", None).await
        };

        format!("**{}:** {}", target_label, target)
    }

    async fn format_changes(&self, _alc: &AuditLogContext<'_>) -> Vec<EmbedField> {
        Vec::new()
    }
}
