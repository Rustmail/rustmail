use crate::commands::status::BotStatus;
use crate::config::Config;
use crate::errors::{CommandError, ModmailError, ModmailResult};
use crate::handlers::GuildMessagesHandler;
use crate::i18n::get_translated_message;
use crate::utils::{MessageBuilder, extract_reply_content};
use serenity::all::{ActivityData, Context, Message};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::Ordering;

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
