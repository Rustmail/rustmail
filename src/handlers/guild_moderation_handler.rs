use serenity::{
    all::{
        AuditLogEntry, ChannelId, Context, EventHandler, GuildId, MessageBuilder,
        standard::CommandFn,
    },
    async_trait,
};

use crate::{
    config::Config,
    utils::{
        build_message_from_ticket::build_message_from_ticket, reply_intent::build_reply_message,
    },
};

pub struct GuildModerationHandler {
    pub config: Config,
}

impl GuildModerationHandler {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for GuildModerationHandler {
    async fn guild_audit_log_entry_create(
        &self,
        ctx: Context,
        entry: AuditLogEntry,
        guild_id: GuildId,
    ) {
        let log_channel_id = ChannelId::new(self.config.bot.logs_channel_id);

        let message_builder = build_reply_message(ticket_message, message_builder);
    }
}
