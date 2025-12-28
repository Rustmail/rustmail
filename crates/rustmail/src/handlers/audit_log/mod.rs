pub mod formatters;

use crate::prelude::config::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use formatters::*;
use serenity::all::audit_log::{Action, Change};
use serenity::all::{
    AuditLogEntry, ChannelId, Colour, Context, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    GuildId, Timestamp, User,
};
use std::collections::HashMap;

pub const COLOR_DANGER: u32 = 0xED4245;
pub const COLOR_SUCCESS: u32 = 0x57F287;
pub const COLOR_WARNING: u32 = 0xFEE75C;
pub const COLOR_INFO: u32 = 0x5865F2;
pub const COLOR_NEUTRAL: u32 = 0x99AAB5;

#[derive(Debug, Clone)]
pub struct AuditLogContext<'a> {
    pub _ctx: &'a Context,
    pub config: &'a Config,
    pub entry: &'a AuditLogEntry,
    pub executor: &'a User,
    pub guild_id: GuildId,
}

impl<'a> AuditLogContext<'a> {
    pub fn new(
        _ctx: &'a Context,
        config: &'a Config,
        entry: &'a AuditLogEntry,
        executor: &'a User,
        guild_id: GuildId,
    ) -> Self {
        Self {
            _ctx,
            config,
            entry,
            executor,
            guild_id,
        }
    }

    pub async fn translate(&self, key: &str, params: Option<&HashMap<String, String>>) -> String {
        get_translated_message(
            self.config,
            key,
            params,
            None,
            Some(self.guild_id.get()),
            None,
        )
        .await
    }

    pub fn target_id(&self) -> Option<u64> {
        self.entry.target_id.map(|id| id.get())
    }

    pub fn reason(&self) -> Option<&str> {
        self.entry.reason.as_deref()
    }

    pub fn changes(&self) -> &[Change] {
        self.entry.changes.as_deref().unwrap_or(&[])
    }
}

#[async_trait::async_trait]
pub trait AuditLogFormatter: Send + Sync {
    fn emoji(&self) -> &'static str;

    fn color(&self) -> u32;

    async fn title(&self, alc: &AuditLogContext<'_>) -> String;

    async fn description(&self, alc: &AuditLogContext<'_>) -> String;

    async fn format_changes(&self, alc: &AuditLogContext<'_>) -> Vec<EmbedField>;

    async fn build_embed(&self, alc: &AuditLogContext<'_>) -> CreateEmbed {
        let title = format!("{} {}", self.emoji(), self.title(alc).await);
        let description = self.description(alc).await;
        let fields = self.format_changes(alc).await;

        let mut embed = CreateEmbed::new()
            .title(title)
            .color(Colour::new(self.color()))
            .timestamp(Timestamp::now())
            .author(
                CreateEmbedAuthor::new(&alc.executor.name).icon_url(
                    alc.executor
                        .avatar_url()
                        .unwrap_or_else(|| alc.executor.default_avatar_url()),
                ),
            );

        if !description.is_empty() {
            embed = embed.description(description);
        }

        for field in fields {
            embed = embed.field(field.name, field.value, field.inline);
        }

        if let Some(reason) = alc.reason() {
            let reason_label = alc.translate("audit_log.reason", None).await;
            embed = embed.field(reason_label, reason, false);
        }

        embed = embed.footer(CreateEmbedFooter::new(format!(
            "ID: {}",
            alc.entry.id.get()
        )));

        embed
    }
}

#[derive(Debug, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

impl EmbedField {
    pub fn new(name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline,
        }
    }
}

pub fn get_formatter(action: &Action) -> Box<dyn AuditLogFormatter> {
    match action {
        Action::GuildUpdate => Box::new(GuildFormatter::Update),

        Action::Channel(action) => Box::new(ChannelFormatter(*action)),
        Action::ChannelOverwrite(action) => Box::new(ChannelOverwriteFormatter(*action)),

        Action::Member(action) => Box::new(MemberFormatter(*action)),

        Action::Role(action) => Box::new(RoleFormatter(*action)),

        Action::Invite(action) => Box::new(InviteFormatter(*action)),

        Action::Webhook(action) => Box::new(WebhookFormatter(*action)),

        Action::Emoji(action) => Box::new(EmojiFormatter(*action)),

        Action::Message(action) => Box::new(MessageFormatter(*action)),

        Action::Integration(action) => Box::new(IntegrationFormatter(*action)),

        Action::StageInstance(action) => Box::new(StageInstanceFormatter(*action)),

        Action::Sticker(action) => Box::new(StickerFormatter(*action)),

        Action::ScheduledEvent(action) => Box::new(ScheduledEventFormatter(*action)),

        Action::Thread(action) => Box::new(ThreadFormatter(*action)),

        Action::AutoMod(action) => Box::new(AutoModFormatter(*action)),

        Action::CreatorMonetization(action) => Box::new(CreatorMonetizationFormatter(*action)),

        Action::VoiceChannelStatus(action) => Box::new(VoiceChannelStatusFormatter(*action)),

        Action::Unknown(code) => Box::new(UnknownFormatter(*code)),
        _ => Box::new(UnknownFormatter(0)),
    }
}

pub async fn send_audit_log(
    ctx: &Context,
    config: &Config,
    entry: &AuditLogEntry,
    executor: &User,
    guild_id: GuildId,
    logs_channel_id: u64,
) {
    let alc = AuditLogContext::new(ctx, config, entry, executor, guild_id);
    let formatter = get_formatter(&entry.action);
    let embed = formatter.build_embed(&alc).await;

    let _ = MessageBuilder::system_message(ctx, config)
        .to_channel(ChannelId::new(logs_channel_id))
        .build_embed_only(embed)
        .send(false)
        .await;
}
