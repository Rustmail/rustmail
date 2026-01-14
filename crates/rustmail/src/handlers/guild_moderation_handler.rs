use crate::handlers::audit_log::send_audit_log;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::features::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::audit_log::Action;
use serenity::all::{ButtonStyle, ChannelAction};
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

async fn manage_creating_ticket_via_opening_thread(
    ctx: &Context,
    config: &Config,
    guild_id: GuildId,
    channel_id: ChannelId,
) {
    let channel_id_inner = {
        let Some(guild) = ctx.cache.guild(guild_id) else {
            return;
        };
        let Some(channel) = guild.channels.get(&channel_id) else {
            return;
        };

        if config.bot.is_dual_mode() && config.bot.is_community_guild(channel.guild_id.get()) {
            return;
        }
        if !config.thread.create_ticket_by_create_channel {
            return;
        }

        let expected_category = config.thread.inbox_category_id;
        if let Some(parent_id) = channel.parent_id {
            if parent_id.get() != expected_category {
                return;
            }
        } else {
            return;
        }

        channel.id
    };

    if let Some(pool) = &config.db_pool {
        if get_thread_by_channel_id(&channel_id_inner.to_string(), pool)
            .await
            .is_some()
        {
            return;
        }
    }

    let yes = get_translated_message(&config, "general.yes", None, None, None, None).await;
    let no = get_translated_message(&config, "general.no", None, None, None, None).await;
    let res_button = make_buttons(&[
        (
            yes.as_ref(),
            "ticket:wants_to_create",
            ButtonStyle::Success,
            false,
        ),
        (
            no.as_ref(),
            "ticket:dont_create",
            ButtonStyle::Danger,
            false,
        ),
    ]);

    let _ = MessageBuilder::system_message(&ctx, &config)
        .translated_content("thread.ask_create_ticket", None, None, None)
        .await
        .components(res_button)
        .to_channel(channel_id_inner)
        .send(true)
        .await;
}

#[async_trait]
impl EventHandler for GuildModerationHandler {
    async fn guild_audit_log_entry_create(
        &self,
        ctx: Context,
        entry: AuditLogEntry,
        guild_id: GuildId,
    ) {
        if let Action::Channel(ChannelAction::Create) = entry.action {
            if entry.user_id != ctx.cache.current_user().id {
                let channel_id = match entry.target_id {
                    Some(id) => ChannelId::new(id.get()),
                    None => return,
                };
                manage_creating_ticket_via_opening_thread(&ctx, &self.config, guild_id, channel_id)
                    .await;
            }
        }

        if !self.config.bot.enable_discord_logs {
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

        send_audit_log(&ctx, &self.config, &entry, &user, guild_id, logs_channel_id).await;
    }
}
