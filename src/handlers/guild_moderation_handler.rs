pub use crate::config::Config;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::audit_log::Action;
use serenity::all::{
    AutoModAction, ChannelAction, ChannelOverwriteAction, CreatorMonetizationAction, EmojiAction,
    IntegrationAction, InviteAction, MemberAction, MessageAction, RoleAction, ScheduledEventAction,
    StageInstanceAction, StickerAction, ThreadAction, VoiceChannelStatusAction, WebhookAction,
};
use serenity::{
    all::{AuditLogEntry, ChannelId, Context, EventHandler, GuildId},
    async_trait,
};

pub struct GuildModerationHandler {
    pub config: Config,
}

impl GuildModerationHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

pub fn format_audit_log(entry: &AuditLogEntry) -> String {
    match (&entry.action, &entry.options) {
        (Action::GuildUpdate, _) => "Server settings were updated".to_string(),

        (Action::Channel(ChannelAction::Create), _) => "Channel was created".to_string(),
        (Action::Channel(ChannelAction::Update), _) => "Channel settings were updated".to_string(),
        (Action::Channel(ChannelAction::Delete), _) => "Channel was deleted".to_string(),

        (Action::ChannelOverwrite(ChannelOverwriteAction::Create), _) => {
            "Permission overwrite was added to a channel".to_string()
        }
        (Action::ChannelOverwrite(ChannelOverwriteAction::Update), _) => {
            "Permission overwrite was updated for a channel".to_string()
        }
        (Action::ChannelOverwrite(ChannelOverwriteAction::Delete), _) => {
            "Permission overwrite was deleted from a channel".to_string()
        }

        (Action::Member(MemberAction::Kick), _) => "Member was removed from server".to_string(),
        (Action::Member(MemberAction::Prune), Some(opts)) => {
            let count = opts.members_removed.unwrap_or_default();
            format!("{} members were pruned from server", count)
        }
        (Action::Member(MemberAction::BanAdd), Some(opts)) => {
            let days = opts.delete_member_days.unwrap_or_default();
            if days > 0 {
                format!(
                    "Member was banned (deleted messages from last {} days)",
                    days
                )
            } else {
                "Member was banned from server".to_string()
            }
        }
        (Action::Member(MemberAction::BanRemove), _) => {
            "Server ban was lifted for a member".to_string()
        }
        (Action::Member(MemberAction::Update), _) => "Member was updated in server".to_string(),
        (Action::Member(MemberAction::RoleUpdate), _) => {
            "Member was added or removed from a role".to_string()
        }
        (Action::Member(MemberAction::MemberMove), Some(opts)) => {
            let channel = opts
                .channel_id
                .map(|id| id.get().to_string())
                .unwrap_or_else(|| "unknown channel".to_string());
            let count = opts.count.unwrap_or_default();
            format!("Moved {} member(s) to channel {}", count, channel)
        }
        (Action::Member(MemberAction::MemberDisconnect), Some(opts)) => {
            let count = opts.count.unwrap_or_default();
            format!("Disconnected {} member(s) from voice channel", count)
        }
        (Action::Member(MemberAction::BotAdd), _) => "Bot user was added to server".to_string(),

        (Action::Role(RoleAction::Create), _) => "Role was created".to_string(),
        (Action::Role(RoleAction::Update), _) => "Role was updated".to_string(),
        (Action::Role(RoleAction::Delete), _) => "Role was deleted".to_string(),

        (Action::Invite(InviteAction::Create), _) => "Server invite was created".to_string(),
        (Action::Invite(InviteAction::Update), _) => "Server invite was updated".to_string(),
        (Action::Invite(InviteAction::Delete), _) => "Server invite was deleted".to_string(),

        (Action::Webhook(WebhookAction::Create), _) => "Webhook was created".to_string(),
        (Action::Webhook(WebhookAction::Update), _) => {
            "Webhook properties or channel were updated".to_string()
        }
        (Action::Webhook(WebhookAction::Delete), _) => "Webhook was deleted".to_string(),

        (Action::Emoji(EmojiAction::Create), _) => "Emoji was created".to_string(),
        (Action::Emoji(EmojiAction::Update), _) => "Emoji name was updated".to_string(),
        (Action::Emoji(EmojiAction::Delete), _) => "Emoji was deleted".to_string(),

        (Action::Message(MessageAction::Delete), Some(opts)) => {
            let channel = opts
                .channel_id
                .map(|id| id.get().to_string())
                .unwrap_or_else(|| "unknown channel".to_string());
            format!("Message was deleted in channel {}", channel)
        }
        (Action::Message(MessageAction::BulkDelete), Some(opts)) => {
            let count = opts.count.unwrap_or_default();
            format!("{} messages were bulk deleted", count)
        }
        (Action::Message(MessageAction::Pin), _) => "Message was pinned to a channel".to_string(),
        (Action::Message(MessageAction::Unpin), _) => {
            "Message was unpinned from a channel".to_string()
        }

        (Action::Integration(IntegrationAction::Create), _) => {
            "App was added to server".to_string()
        }
        (Action::Integration(IntegrationAction::Update), _) => {
            "App was updated (e.g., scopes updated)".to_string()
        }
        (Action::Integration(IntegrationAction::Delete), _) => {
            "App was removed from server".to_string()
        }

        (Action::StageInstance(StageInstanceAction::Create), _) => {
            "Stage instance was created (stage channel goes live)".to_string()
        }
        (Action::StageInstance(StageInstanceAction::Update), _) => {
            "Stage instance details were updated".to_string()
        }
        (Action::StageInstance(StageInstanceAction::Delete), _) => {
            "Stage instance was deleted (no longer live)".to_string()
        }

        (Action::Sticker(StickerAction::Create), _) => "Sticker was created".to_string(),
        (Action::Sticker(StickerAction::Update), _) => "Sticker details were updated".to_string(),
        (Action::Sticker(StickerAction::Delete), _) => "Sticker was deleted".to_string(),

        (Action::ScheduledEvent(ScheduledEventAction::Create), _) => {
            "Event was created".to_string()
        }
        (Action::ScheduledEvent(ScheduledEventAction::Update), _) => {
            "Event was updated".to_string()
        }
        (Action::ScheduledEvent(ScheduledEventAction::Delete), _) => {
            "Event was cancelled".to_string()
        }

        (Action::Thread(ThreadAction::Create), _) => "Thread was created in a channel".to_string(),
        (Action::Thread(ThreadAction::Update), _) => "Thread was updated".to_string(),
        (Action::Thread(ThreadAction::Delete), _) => "Thread was deleted".to_string(),

        (Action::AutoMod(AutoModAction::RuleCreate), _) => {
            "Auto Moderation rule was created".to_string()
        }
        (Action::AutoMod(AutoModAction::RuleUpdate), _) => {
            "Auto Moderation rule was updated".to_string()
        }
        (Action::AutoMod(AutoModAction::RuleDelete), _) => {
            "Auto Moderation rule was deleted".to_string()
        }
        (Action::AutoMod(AutoModAction::BlockMessage), _) => {
            "Message was blocked by Auto Moderation".to_string()
        }
        (Action::AutoMod(AutoModAction::FlagToChannel), _) => {
            "Message was flagged by Auto Moderation".to_string()
        }
        (Action::AutoMod(AutoModAction::UserCommunicationDisabled), _) => {
            "Member was timed out by Auto Moderation".to_string()
        }

        (Action::CreatorMonetization(CreatorMonetizationAction::RequestCreated), _) => {
            "Creator monetization request was created".to_string()
        }
        (Action::CreatorMonetization(CreatorMonetizationAction::TermsAccepted), _) => {
            "Creator monetization terms were accepted".to_string()
        }

        (Action::VoiceChannelStatus(VoiceChannelStatusAction::StatusUpdate), _) => {
            "Voice channel status update".to_string()
        }
        (Action::VoiceChannelStatus(VoiceChannelStatusAction::StatusDelete), _) => {
            "Voice channel status delete".to_string()
        }

        (Action::Unknown(code), _) => format!("Unknown action: {}", code),
        _ => format!("Unknown or unhandled action: {:?}", entry.action),
    }
}

#[async_trait]
impl EventHandler for GuildModerationHandler {
    async fn guild_audit_log_entry_create(
        &self,
        ctx: Context,
        entry: AuditLogEntry,
        _guild_id: GuildId,
    ) {
        if !self.config.bot.enable_logs {
            return;
        }

        let logs_channel_id = match self.config.bot.logs_channel_id {
            Some(channel_id) => channel_id,
            None => return,
        };

        let user = match entry.user_id.to_user(ctx.clone()).await {
            Ok(user) => user,
            Err(_) => {
                eprintln!("Unable to get User from user_id for showing logs");
                return;
            }
        };

        let _ = MessageBuilder::user_message(&ctx, &self.config, user.id, user.name)
            .to_channel(ChannelId::new(logs_channel_id))
            .content(format_audit_log(&entry))
            .send()
            .await;
    }
}
