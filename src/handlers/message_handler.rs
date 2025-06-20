use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use serenity::{
    all::{ChannelId, Context, EventHandler, Message},
    async_trait,
};

use crate::commands::{
    close::close,
    edit::edit_command::edit,
    help::help,
    reply::reply,
    test_errors::{test_all_errors, test_errors, test_language},
};
use crate::db::operations::{get_thread_channel_by_user_id, thread_exists};
use crate::errors::{ModmailResult, common};
use crate::utils::send_to_thread::send_to_thread;
use crate::config::Config;
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

    if thread_exists(msg.author.id, pool).await {
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
        if !msg.guild_id.is_some() {
            if let Err(error) = manage_incoming_message(&ctx, &msg, &self.config).await {
                if let Some(error_handler) = &self.config.error_handler {
                    let _ = error_handler.reply_with_error(&ctx, &msg, &error).await;
                } else {
                    eprintln!("DM handling error: {}", error);
                }
            }
            return;
        }
        let message_content = &msg.clone().content;
        if !message_content.starts_with(&self.config.command.prefix) {
            return;
        }
        let mut command_name = &message_content[1..];

        if let Some(i) = message_content.find(" ") {
            command_name = &message_content[self.config.command.prefix.len()..i];
        }

        if let Some(command_func) = self.commands.get(command_name) {
            if let Err(error) = command_func(ctx.clone(), msg.clone(), self.config.clone()).await {
                if let Some(error_handler) = &self.config.error_handler {
                    let _ = error_handler.reply_with_error(&ctx, &msg, &error).await;
                } else {
                    eprintln!("Command error: {}", error);
                }
            }
        }
    }
}
