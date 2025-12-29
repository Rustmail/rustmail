use super::{EmbedField, format_change_value};
use crate::handlers::audit_log::{
    AuditLogContext, AuditLogFormatter, COLOR_DANGER, COLOR_INFO, COLOR_SUCCESS, COLOR_WARNING,
};
use serenity::all::MemberAction;
use std::collections::HashMap;

pub struct MemberFormatter(pub MemberAction);

#[async_trait::async_trait]
impl AuditLogFormatter for MemberFormatter {
    fn emoji(&self) -> &'static str {
        match self.0 {
            MemberAction::Kick => "👢",
            MemberAction::Prune => "🧹",
            MemberAction::BanAdd => "🔨",
            MemberAction::BanRemove => "🔓",
            MemberAction::Update => "✏️",
            MemberAction::RoleUpdate => "🎭",
            MemberAction::MemberMove => "↔️",
            MemberAction::MemberDisconnect => "📵",
            MemberAction::BotAdd => "🤖",
            _ => "👤",
        }
    }

    fn color(&self) -> u32 {
        match self.0 {
            MemberAction::Kick | MemberAction::BanAdd | MemberAction::Prune => COLOR_DANGER,
            MemberAction::BanRemove | MemberAction::BotAdd => COLOR_SUCCESS,
            MemberAction::Update | MemberAction::RoleUpdate => COLOR_WARNING,
            MemberAction::MemberMove | MemberAction::MemberDisconnect => COLOR_INFO,
            _ => COLOR_INFO,
        }
    }

    async fn title(&self, alc: &AuditLogContext<'_>) -> String {
        let key = match self.0 {
            MemberAction::Kick => "audit_log.member.kick",
            MemberAction::Prune => "audit_log.member.prune",
            MemberAction::BanAdd => "audit_log.member.ban_add",
            MemberAction::BanRemove => "audit_log.member.ban_remove",
            MemberAction::Update => "audit_log.member.update",
            MemberAction::RoleUpdate => "audit_log.member.role_update",
            MemberAction::MemberMove => "audit_log.member.move",
            MemberAction::MemberDisconnect => "audit_log.member.disconnect",
            MemberAction::BotAdd => "audit_log.member.bot_add",
            _ => "audit_log.member.unknown",
        };
        alc.translate(key, None).await
    }

    async fn description(&self, alc: &AuditLogContext<'_>) -> String {
        match self.0 {
            MemberAction::MemberMove => {
                if let Some(opts) = &alc.entry.options {
                    if let Some(channel_id) = opts.channel_id {
                        let mut params = HashMap::new();
                        params.insert("channel".to_string(), format!("<#{}>", channel_id.get()));
                        if let Some(count) = opts.count {
                            params.insert("count".to_string(), count.to_string());
                        }
                        return alc
                            .translate("audit_log.member.moved_to", Some(&params))
                            .await;
                    }
                }
                return String::new();
            }
            MemberAction::MemberDisconnect => {
                if let Some(opts) = &alc.entry.options {
                    if let Some(count) = opts.count {
                        let mut params = HashMap::new();
                        params.insert("count".to_string(), count.to_string());
                        return alc
                            .translate("audit_log.member.disconnected_count", Some(&params))
                            .await;
                    }
                }
                return String::new();
            }
            _ => {}
        }

        let target_label = alc.translate("audit_log.target", None).await;

        let target = if let Some(target_id) = alc.target_id() {
            format!("<@{}> (`{}`)", target_id, target_id)
        } else {
            alc.translate("audit_log.unknown", None).await
        };

        let mut desc = format!("**{}:** {}", target_label, target);

        if let Some(opts) = &alc.entry.options {
            match self.0 {
                MemberAction::Prune => {
                    if let Some(count) = opts.members_removed {
                        let mut params = HashMap::new();
                        params.insert("count".to_string(), count.to_string());
                        let pruned = alc
                            .translate("audit_log.member.pruned_count", Some(&params))
                            .await;
                        desc.push_str(&format!("\n{}", pruned));
                    }
                }
                MemberAction::BanAdd => {
                    if let Some(days) = opts.delete_member_days {
                        if days > 0 {
                            let mut params = HashMap::new();
                            params.insert("days".to_string(), days.to_string());
                            let deleted = alc
                                .translate("audit_log.member.messages_deleted", Some(&params))
                                .await;
                            desc.push_str(&format!("\n{}", deleted));
                        }
                    }
                }
                _ => {}
            }
        }

        desc
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
