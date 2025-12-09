use super::EmbedField;
use crate::handlers::audit_log::{AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_INFO};
use serenity::all::MessageAction;
use std::collections::HashMap;

pub struct MessageFormatter(pub MessageAction);

#[async_trait::async_trait]
impl AuditLogFormatter for MessageFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            MessageAction::Delete => "🗑️",
            MessageAction::BulkDelete => "🗑️",
            MessageAction::Pin => "📌",
            MessageAction::Unpin => "📍",
            _ => "💬",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            MessageAction::Delete | MessageAction::BulkDelete => COLOR_DANGER,
            MessageAction::Pin | MessageAction::Unpin => COLOR_INFO,
            _ => COLOR_INFO,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            MessageAction::Delete => "audit_log.message.delete",
            MessageAction::BulkDelete => "audit_log.message.bulk_delete",
            MessageAction::Pin => "audit_log.message.pin",
            MessageAction::Unpin => "audit_log.message.unpin",
            _ => "audit_log.message.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        let mut desc = String::new();

        if let Some(opts) = &alc.entry.options {
            match self.0 {
                MessageAction::Delete => {
                    if let Some(channel_id) = opts.channel_id {
                        let channel_label = alc.translate("audit_log.channel", None).await;
                        desc.push_str(&format!(
                            "**{}:** <#{}>\n",
                            channel_label,
                            channel_id.get()
                        ));
                    }
                }
                MessageAction::BulkDelete => {
                    if let Some(count) = opts.count {
                        let mut params = HashMap::new();
                        params.insert("count".to_string(), count.to_string());
                        let deleted = alc
                            .translate("audit_log.message.deleted_count", Some(&params))
                            .await;
                        desc.push_str(&format!("{}\n", deleted));
                    }
                    if let Some(channel_id) = opts.channel_id {
                        let channel_label = alc.translate("audit_log.channel", None).await;
                        desc.push_str(&format!(
                            "**{}:** <#{}>",
                            channel_label,
                            channel_id.get()
                        ));
                    }
                }
                MessageAction::Pin | MessageAction::Unpin => {
                    if let Some(channel_id) = opts.channel_id {
                        let channel_label = alc.translate("audit_log.channel", None).await;
                        desc.push_str(&format!(
                            "**{}:** <#{}>",
                            channel_label,
                            channel_id.get()
                        ));
                    }
                }
                _ => {}
            }
        }

        desc
    }

    async fn format_changes(&self, _alc: &AuditLogContext<'_>) -> Vec<EmbedField> {
        Vec::new()
    }
}
