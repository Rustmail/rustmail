use super::EmbedField;
use crate::handlers::audit_log::{AuditLogContext, AuditLogFormatter, COLOR_SUCCESS};
use serenity::all::CreatorMonetizationAction;

pub struct CreatorMonetizationFormatter(pub CreatorMonetizationAction);

#[async_trait::async_trait]
impl AuditLogFormatter for CreatorMonetizationFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            CreatorMonetizationAction::RequestCreated => "💰",
            CreatorMonetizationAction::TermsAccepted => "✅",
            _ => "💰",
        }
    }

    fn color(&self) -> u32 {
        COLOR_SUCCESS
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            CreatorMonetizationAction::RequestCreated => {
                "audit_log.creator_monetization.request_created"
            }
            CreatorMonetizationAction::TermsAccepted => {
                "audit_log.creator_monetization.terms_accepted"
            }
            _ => "audit_log.creator_monetization.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, _alc: &AuditLogContext<'_>) -> String {
        String::new()
    }

    async fn format_changes(&self, _alc: &AuditLogContext<'_>) -> Vec<EmbedField> {
        Vec::new()
    }
}
