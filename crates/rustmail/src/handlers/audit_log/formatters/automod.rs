use super::{EmbedField, format_change_value};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_INFO, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::AutoModAction;

pub struct AutoModFormatter(pub AutoModAction);

#[async_trait::async_trait]
impl AuditLogFormatter for AutoModFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            AutoModAction::RuleCreate => "🛡️",
            AutoModAction::RuleUpdate => "✏️",
            AutoModAction::RuleDelete => "🗑️",
            AutoModAction::BlockMessage => "🚫",
            AutoModAction::FlagToChannel => "🚩",
            AutoModAction::UserCommunicationDisabled => "🔇",
            _ => "🛡️",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            AutoModAction::RuleCreate => COLOR_SUCCESS,
            AutoModAction::RuleUpdate => COLOR_WARNING,
            AutoModAction::RuleDelete => COLOR_DANGER,
            AutoModAction::BlockMessage => COLOR_DANGER,
            AutoModAction::FlagToChannel => COLOR_WARNING,
            AutoModAction::UserCommunicationDisabled => COLOR_INFO,
            _ => COLOR_INFO,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            AutoModAction::RuleCreate => "audit_log.automod.rule_create",
            AutoModAction::RuleUpdate => "audit_log.automod.rule_update",
            AutoModAction::RuleDelete => "audit_log.automod.rule_delete",
            AutoModAction::BlockMessage => "audit_log.automod.block_message",
            AutoModAction::FlagToChannel => "audit_log.automod.flag_to_channel",
            AutoModAction::UserCommunicationDisabled => "audit_log.automod.user_timeout",
            _ => "audit_log.automod.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        let target_label = alc.translate("audit_log.target", None).await;

        let target = if let Some(target_id) = alc.target_id() {
            match self.0 {
                AutoModAction::UserCommunicationDisabled => {
                    format!("<@{}> (`{}`)", target_id, target_id)
                }
                _ => format!("`{}`", target_id),
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
