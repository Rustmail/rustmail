mod automod;
mod channel;
mod channel_overwrite;
mod creator_monetization;
mod emoji;
mod guild;
mod integration;
mod invite;
mod member;
mod message;
mod role;
mod scheduled_event;
mod stage_instance;
mod sticker;
mod thread;
mod unknown;
mod voice_channel_status;
mod webhook;

pub use automod::*;
pub use channel::*;
pub use channel_overwrite::*;
pub use creator_monetization::*;
pub use emoji::*;
pub use guild::*;
pub use integration::*;
pub use invite::*;
pub use member::*;
pub use message::*;
pub use role::*;
pub use scheduled_event::*;
pub use stage_instance::*;
pub use sticker::*;
pub use thread::*;
pub use unknown::*;
pub use voice_channel_status::*;
pub use webhook::*;

use super::{AuditLogContext, EmbedField};
use serenity::all::audit_log::Change;

pub async fn format_change_value(
    alc: &AuditLogContext<'_>,
    change: &Change,
) -> Option<EmbedField> {
    match change {
        Change::AfkChannelId { old, new } => {
            let name = alc.translate("audit_log.change.afk_channel", None).await;
            let old_val = old
                .map(|id| format!("<#{}>", id.get()))
                .unwrap_or_else(|| "None".to_string());
            let new_val = new
                .map(|id| format!("<#{}>", id.get()))
                .unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::AfkTimeout { old, new } => {
            let name = alc.translate("audit_log.change.afk_timeout", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Allow { old, new } => {
            let name = alc.translate("audit_log.change.permissions_allow", None).await;
            let old_val = old.map(|p| p.bits().to_string()).unwrap_or_else(|| "0".to_string());
            let new_val = new.map(|p| p.bits().to_string()).unwrap_or_else(|| "0".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::ApplicationId { new, .. } => {
            let name = alc.translate("audit_log.change.application", None).await;
            let val = new.map(|id| id.get().to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, val, true))
        }
        Change::Archived { old, new } => {
            let name = alc.translate("audit_log.change.archived", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Asset { new, .. } => {
            let name = alc.translate("audit_log.change.asset", None).await;
            let val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, val, true))
        }
        Change::AutoArchiveDuration { old, new } => {
            let name = alc.translate("audit_log.change.auto_archive_duration", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Available { old, new } => {
            let name = alc.translate("audit_log.change.available", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::AvatarHash { old, new } => {
            let name = alc.translate("audit_log.change.avatar", None).await;
            let old_val = old.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            let new_val = new.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::BannerHash { old, new } => {
            let name = alc.translate("audit_log.change.banner", None).await;
            let old_val = old.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            let new_val = new.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Bitrate { old, new } => {
            let name = alc.translate("audit_log.change.bitrate", None).await;
            let old_val = old.map(|v| format!("{}kbps", v / 1000)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{}kbps", v / 1000)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::ChannelId { old, new } => {
            let name = alc.translate("audit_log.change.channel", None).await;
            let old_val = old.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Code { old, new } => {
            let name = alc.translate("audit_log.change.invite_code", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Color { old, new } => {
            let name = alc.translate("audit_log.change.color", None).await;
            let old_val = old.map(|c| format!("#{:06X}", c)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|c| format!("#{:06X}", c)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::CommunicationDisabledUntil { old, new } => {
            let name = alc.translate("audit_log.change.timeout", None).await;
            let old_val = old.map(|t| format!("<t:{}:R>", t.unix_timestamp())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|t| format!("<t:{}:R>", t.unix_timestamp())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Deaf { old, new } => {
            let name = alc.translate("audit_log.change.deaf", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::DefaultAutoArchiveDuration { old, new } => {
            let name = alc.translate("audit_log.change.default_auto_archive", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::DefaultMessageNotifications { old, new } => {
            let name = alc.translate("audit_log.change.default_notifications", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Deny { old, new } => {
            let name = alc.translate("audit_log.change.permissions_deny", None).await;
            let old_val = old.map(|p| p.bits().to_string()).unwrap_or_else(|| "0".to_string());
            let new_val = new.map(|p| p.bits().to_string()).unwrap_or_else(|| "0".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Description { old, new } => {
            let name = alc.translate("audit_log.change.description", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            let old_truncated = if old_val.len() > 100 { format!("{}...", &old_val[..100]) } else { old_val };
            let new_truncated = if new_val.len() > 100 { format!("{}...", &new_val[..100]) } else { new_val };
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_truncated, new_truncated), false))
        }
        Change::DiscoverySplashHash { old, new } => {
            let name = alc.translate("audit_log.change.discovery_splash", None).await;
            let old_val = old.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            let new_val = new.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::EnableEmoticons { old, new } => {
            let name = alc.translate("audit_log.change.enable_emoticons", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::EntityType { old, new } => {
            let name = alc.translate("audit_log.change.entity_type", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::ExpireBehavior { old, new } => {
            let name = alc.translate("audit_log.change.expire_behavior", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::ExpireGracePeriod { old, new } => {
            let name = alc.translate("audit_log.change.expire_grace_period", None).await;
            let old_val = old.map(|v| format!("{} days", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{} days", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::ExplicitContentFilter { old, new } => {
            let name = alc.translate("audit_log.change.explicit_content_filter", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::FormatType { old, new } => {
            let name = alc.translate("audit_log.change.format_type", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::GuildId { new, .. } => {
            let name = alc.translate("audit_log.change.guild", None).await;
            let val = new.map(|id| id.get().to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, val, true))
        }
        Change::Hoist { old, new } => {
            let name = alc.translate("audit_log.change.hoist", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::IconHash { old, new } => {
            let name = alc.translate("audit_log.change.icon", None).await;
            let old_val = old.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            let new_val = new.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Id { new, .. } => {
            let name = alc.translate("audit_log.change.id", None).await;
            let val = new.map(|id| id.get().to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, val, true))
        }
        Change::ImageHash { old, new } => {
            let name = alc.translate("audit_log.change.image", None).await;
            let old_val = old.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            let new_val = new.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Invitable { old, new } => {
            let name = alc.translate("audit_log.change.invitable", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::InviterId { old, new } => {
            let name = alc.translate("audit_log.change.inviter", None).await;
            let old_val = old.map(|id| format!("<@{}>", id.get())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|id| format!("<@{}>", id.get())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Location { old, new } => {
            let name = alc.translate("audit_log.change.location", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Locked { old, new } => {
            let name = alc.translate("audit_log.change.locked", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::MaxAge { old, new } => {
            let name = alc.translate("audit_log.change.max_age", None).await;
            let old_val = old.map(|v| if v == 0 { "Never".to_string() } else { format!("{}s", v) }).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| if v == 0 { "Never".to_string() } else { format!("{}s", v) }).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::MaxUses { old, new } => {
            let name = alc.translate("audit_log.change.max_uses", None).await;
            let old_val = old.map(|v| if v == 0 { "Unlimited".to_string() } else { v.to_string() }).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| if v == 0 { "Unlimited".to_string() } else { v.to_string() }).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Mentionable { old, new } => {
            let name = alc.translate("audit_log.change.mentionable", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::MfaLevel { old, new } => {
            let name = alc.translate("audit_log.change.mfa_level", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Mute { old, new } => {
            let name = alc.translate("audit_log.change.mute", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Name { old, new } => {
            let name = alc.translate("audit_log.change.name", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Nick { old, new } => {
            let name = alc.translate("audit_log.change.nickname", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Nsfw { old, new } => {
            let name = alc.translate("audit_log.change.nsfw", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::OwnerId { old, new } => {
            let name = alc.translate("audit_log.change.owner", None).await;
            let old_val = old.map(|id| format!("<@{}>", id.get())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|id| format!("<@{}>", id.get())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::PermissionOverwrites { .. } => {
            let name = alc.translate("audit_log.change.permission_overwrites", None).await;
            Some(EmbedField::new(name, "Modified".to_string(), true))
        }
        Change::Permissions { old, new } => {
            let name = alc.translate("audit_log.change.permissions", None).await;
            let old_val = old.map(|p| p.bits().to_string()).unwrap_or_else(|| "0".to_string());
            let new_val = new.map(|p| p.bits().to_string()).unwrap_or_else(|| "0".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Position { old, new } => {
            let name = alc.translate("audit_log.change.position", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::PreferredLocale { old, new } => {
            let name = alc.translate("audit_log.change.preferred_locale", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::PrivacyLevel { old, new } => {
            let name = alc.translate("audit_log.change.privacy_level", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::PruneDeleteDays { old, new } => {
            let name = alc.translate("audit_log.change.prune_delete_days", None).await;
            let old_val = old.map(|v| format!("{} days", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{} days", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::PublicUpdatesChannelId { old, new } => {
            let name = alc.translate("audit_log.change.public_updates_channel", None).await;
            let old_val = old.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::RateLimitPerUser { old, new } => {
            let name = alc.translate("audit_log.change.slowmode", None).await;
            let old_val = old.map(|v| if v == 0 { "Off".to_string() } else { format!("{}s", v) }).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| if v == 0 { "Off".to_string() } else { format!("{}s", v) }).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Region { old, new } => {
            let name = alc.translate("audit_log.change.region", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::RolesAdded { new, .. } => {
            if let Some(roles) = new {
                let name = alc.translate("audit_log.change.roles_added", None).await;
                let roles_str = roles.iter().map(|r| format!("<@&{}>", r.id.get())).collect::<Vec<_>>().join(", ");
                Some(EmbedField::new(name, roles_str, false))
            } else {
                None
            }
        }
        Change::RolesRemove { new, .. } => {
            if let Some(roles) = new {
                let name = alc.translate("audit_log.change.roles_removed", None).await;
                let roles_str = roles.iter().map(|r| format!("<@&{}>", r.id.get())).collect::<Vec<_>>().join(", ");
                Some(EmbedField::new(name, roles_str, false))
            } else {
                None
            }
        }
        Change::RulesChannelId { old, new } => {
            let name = alc.translate("audit_log.change.rules_channel", None).await;
            let old_val = old.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::SplashHash { old, new } => {
            let name = alc.translate("audit_log.change.splash", None).await;
            let old_val = old.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            let new_val = new.as_ref().map(|h| format!("{:?}", h)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Status { old, new } => {
            let name = alc.translate("audit_log.change.status", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::SystemChannelId { old, new } => {
            let name = alc.translate("audit_log.change.system_channel", None).await;
            let old_val = old.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Tags { old, new } => {
            let name = alc.translate("audit_log.change.tags", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Temporary { old, new } => {
            let name = alc.translate("audit_log.change.temporary", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Topic { old, new } => {
            let name = alc.translate("audit_log.change.topic", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            let old_truncated = if old_val.len() > 100 { format!("{}...", &old_val[..100]) } else { old_val };
            let new_truncated = if new_val.len() > 100 { format!("{}...", &new_val[..100]) } else { new_val };
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_truncated, new_truncated), false))
        }
        Change::Type { old, new } => {
            let name = alc.translate("audit_log.change.type", None).await;
            let old_val = old.as_ref().map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.as_ref().map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::UnicodeEmoji { old, new } => {
            let name = alc.translate("audit_log.change.unicode_emoji", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::UserLimit { old, new } => {
            let name = alc.translate("audit_log.change.user_limit", None).await;
            let old_val = old.map(|v| if v == 0 { "Unlimited".to_string() } else { v.to_string() }).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| if v == 0 { "Unlimited".to_string() } else { v.to_string() }).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::Uses { old, new } => {
            let name = alc.translate("audit_log.change.uses", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::VanityUrlCode { old, new } => {
            let name = alc.translate("audit_log.change.vanity_url", None).await;
            let old_val = old.clone().unwrap_or_else(|| "None".to_string());
            let new_val = new.clone().unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::VerificationLevel { old, new } => {
            let name = alc.translate("audit_log.change.verification_level", None).await;
            let old_val = old.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| format!("{:?}", v)).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::WidgetChannelId { old, new } => {
            let name = alc.translate("audit_log.change.widget_channel", None).await;
            let old_val = old.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|id| format!("<#{}>", id.get())).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::WidgetEnabled { old, new } => {
            let name = alc.translate("audit_log.change.widget_enabled", None).await;
            let old_val = old.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("{} → {}", old_val, new_val), true))
        }
        Change::SystemChannelFlags { old, new } => {
            let name = alc.translate("audit_log.change.system_channel_flags", None).await;
            let old_val = old.map(|v| v.bits().to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new.map(|v| v.bits().to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name, format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Other { name, old_value, new_value } => {
            let old_val = old_value.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            let new_val = new_value.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            Some(EmbedField::new(name.clone(), format!("`{}` → `{}`", old_val, new_val), true))
        }
        Change::Unknown { .. } => None,
        _ => None,
    }
}
