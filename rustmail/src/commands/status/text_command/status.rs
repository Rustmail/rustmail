use crate::commands::status::BotStatus;
use crate::config::Config;
use crate::errors::{CommandError, ModmailError, ModmailResult};
use crate::handlers::GuildMessagesHandler;
use crate::i18n::get_translated_message;
use crate::utils::{MessageBuilder, extract_reply_content};
use serenity::all::{ActivityData, Context, Message, Permissions};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::Ordering;

async fn is_user_admin_or_super_admin(ctx: &Context, msg: &Message, config: &Config) -> bool {
    let user_id = msg.author.id.get();

    if config.bot.panel_super_admin_users.contains(&user_id) {
        return true;
    }

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return false,
    };

    let member = match guild_id.member(&ctx.http, msg.author.id).await {
        Ok(m) => m,
        Err(_) => return false,
    };

    if !config.bot.panel_super_admin_roles.is_empty() {
        for role_id in &member.roles {
            if config.bot.panel_super_admin_roles.contains(&role_id.get()) {
                return true;
            }
        }
    }

    let guild = match guild_id.to_partial_guild(&ctx.http).await {
        Ok(g) => g,
        Err(_) => return false,
    };

    if guild.owner_id == msg.author.id {
        return true;
    }

    member.roles.iter().any(|role_id| {
        guild
            .roles
            .get(role_id)
            .map(|role| role.permissions.contains(Permissions::ADMINISTRATOR))
            .unwrap_or(false)
    })
}

pub async fn status_command(
    ctx: Context,
    msg: Message,
    config: &Config,
    handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let content = match extract_reply_content(&msg.content, &config.command.prefix, &["status"]) {
        Some(c) => c,
        None => return Err(ModmailError::Command(CommandError::StatusIsMissing)),
    };

    let bot_status = match BotStatus::from_str(&content) {
        Ok(status) => status,
        Err(_) => return Err(ModmailError::Command(CommandError::InvalidStatusValue)),
    };

    if bot_status == BotStatus::Maintenance
        && !is_user_admin_or_super_admin(&ctx, &msg, config).await
    {
        return Err(ModmailError::Command(
            CommandError::MaintenanceModeNotAllowed,
        ));
    }

    let message_key = match bot_status {
        BotStatus::Online => {
            handler.maintenance_mode.store(false, Ordering::Relaxed);
            ctx.set_activity(Some(ActivityData::playing(&config.bot.status)));
            ctx.online();
            "status.status_online"
        }
        BotStatus::Idle => {
            ctx.idle();
            "status.status_idle"
        }
        BotStatus::Dnd => {
            ctx.dnd();
            "status.status_dnd"
        }
        BotStatus::Invisible => {
            ctx.invisible();
            "status.status_invisible"
        }
        BotStatus::Maintenance => {
            handler.maintenance_mode.store(true, Ordering::Relaxed);
            let maintenance_status = get_translated_message(
                config,
                "status.maintenance_activity",
                None,
                None,
                None,
                None,
            )
            .await;
            ctx.set_activity(Some(ActivityData::playing(&maintenance_status)));
            ctx.dnd();
            "status.status_maintenance"
        }
    };

    let _ = MessageBuilder::system_message(&ctx, config)
        .translated_content(message_key, None, None, None)
        .await
        .to_channel(msg.channel_id)
        .send(true)
        .await;

    Ok(())
}
