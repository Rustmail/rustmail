use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use serenity::{
    all::{ChannelId, Context, EventHandler, Message},
    async_trait,
};

use crate::commands::{
    alert::alert,
    anonreply::anonreply,
    close::close,
    delete::delete,
    edit::edit_command::edit,
    help::help,
    move_thread::move_thread,
    new_thread::new_thread,
    recover::recover,
    reply::reply,
    test_errors::{test_all_errors, test_errors, test_language},
};
use crate::config::Config;
use crate::db::operations::{get_thread_channel_by_user_id, thread_exists};
use crate::errors::{ModmailResult, common};
use crate::utils::send_to_thread::send_to_thread;
use crate::{modules::threads::create_channel, utils::wrap_command};

type CommandFunc = Arc<StaticCommandFunc>;
type StaticCommandFunc = dyn Fn(Context, Message, Config) -> Pin<Box<dyn Future<Output = ModmailResult<()>> + Send>>
    + Send
    + Sync
    + 'static;

pub struct MessageHandler {
    pub config: Config,
    pub commands: HashMap<String, CommandFunc>,
}

impl MessageHandler {
    pub fn new(config: &Config) -> Self {
        let mut h = Self {
            config: config.clone(),
            commands: HashMap::new(),
        };
        wrap_command!(h.commands, "help", help);
        wrap_command!(h.commands, ["reply", "r"], reply);
        wrap_command!(h.commands, ["edit", "e"], edit);
        wrap_command!(h.commands, ["close", "c"], close);
        wrap_command!(h.commands, "recover", recover);
        wrap_command!(h.commands, "alert", alert);
        wrap_command!(h.commands, ["move", "mv"], move_thread);
        wrap_command!(h.commands, ["nt", "new_thread"], new_thread);
        wrap_command!(h.commands, "delete", delete);
        wrap_command!(h.commands, ["anonreply", "ar"], anonreply);
        wrap_command!(h.commands, "test_errors", test_errors);
        wrap_command!(h.commands, "test_language", test_language);
        wrap_command!(h.commands, "test_all_errors", test_all_errors);
        h
    }
}

async fn manage_incoming_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
) -> ModmailResult<()> {
    if msg.content.starts_with(&config.command.prefix) {
        return Ok(());
    }

    if msg.author.bot {
        return Ok(());
    }

    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let error_handler = config
        .error_handler
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    if let Some(guild_id) = msg.guild_id {
        let community_guild_id = config.bot.get_community_guild_id();
        if guild_id.get() != community_guild_id {
            let error_key = if config.bot.is_dual_mode() {
                "server.wrong_guild_dual"
            } else {
                "server.wrong_guild_single"
            };

            let error_msg = crate::i18n::get_translated_message(
                config,
                error_key,
                None,
                Some(msg.author.id),
                Some(guild_id.get()),
                None,
            )
            .await;

            let error = common::validation_failed(&error_msg);
            let _ = error_handler.reply_with_error(ctx, msg, &error).await;
            return Err(error);
        }
    }

    let thread_exists = thread_exists(msg.author.id, pool).await;

    if thread_exists {
        if let Some(channel_id_str) = get_thread_channel_by_user_id(msg.author.id, pool).await {
            let channel_id_num = channel_id_str
                .parse::<u64>()
                .map_err(|_| common::validation_failed("Invalid channel ID format"))?;

            let channel_id = ChannelId::new(channel_id_num);

            if let Err(e) = send_to_thread(ctx, channel_id, msg, config, false).await {
                let error = common::validation_failed(&format!("Failed to forward message: {}", e));
                let _ = error_handler.reply_with_error(ctx, msg, &error).await;
                return Err(error);
            }
        }
    } else {
        create_channel(ctx, msg, config).await;
    }

    Ok(())
}

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.guild_id.is_none() {
            if let Err(error) = manage_incoming_message(&ctx, &msg, &self.config).await {
                if let Some(error_handler) = &self.config.error_handler {
                    let _ = error_handler.reply_with_error(&ctx, &msg, &error).await;
                } else {
                    eprintln!("DM handling error: {}", error);
                }
            }
            return;
        }

        let message_content = &msg.content;
        if message_content.starts_with(&self.config.command.prefix) {
            let mut command_name = &message_content[1..];

            if let Some(i) = message_content.find(" ") {
                command_name = &message_content[self.config.command.prefix.len()..i];
            }

            if let Some(command_func) = self.commands.get(command_name) {
                if let Err(error) =
                    command_func(ctx.clone(), msg.clone(), self.config.clone()).await
                {
                    if let Some(error_handler) = &self.config.error_handler {
                        let _ = error_handler.reply_with_error(&ctx, &msg, &error).await;
                    } else {
                        eprintln!("Command error: {}", error);
                    }
                }
            }
            return;
        }

        return;
    }
}
