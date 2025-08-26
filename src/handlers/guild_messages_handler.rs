use crate::commands::edit::message_ops::edit_inbox_message;
use crate::commands::force_close::force_close;
use crate::commands::{
    alert::alert,
    anonreply::anonreply,
    close::close,
    delete::delete,
    edit::edit_command::edit,
    move_thread::move_thread,
    new_thread::new_thread,
    recover::recover,
    reply::reply,
    test_errors::{test_all_errors, test_errors, test_language},
};
use crate::config::Config;
use crate::db::messages::get_thread_message_by_dm_message_id;
use crate::db::operations::{
    get_thread_channel_by_user_id, thread_exists,
    update_message_content,
};
use crate::db::threads::get_thread_by_user_id;
use crate::errors::{ModmailResult, common};
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use crate::utils::thread::get_thread_lock::get_thread_lock;
use crate::utils::thread::send_to_thread::send_to_thread;
use crate::{modules::threads::create_channel, utils::wrap_command};
use serenity::all::UserId;
use serenity::{
    all::{ChannelId, Context, EventHandler, Message, MessageUpdateEvent},
    async_trait,
};
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use crate::commands::add_staff::add_staff;
use crate::commands::remove_staff::remove_staff;

type CommandFunc = Arc<StaticCommandFunc>;
type StaticCommandFunc = dyn Fn(Context, Message, Config) -> Pin<Box<dyn Future<Output = ModmailResult<()>> + Send>>
    + Send
    + Sync
    + 'static;

pub struct GuildMessagesHandler {
    pub config: Config,
    pub commands: HashMap<String, CommandFunc>,
}

impl GuildMessagesHandler {
    pub fn new(config: &Config) -> Self {
        let mut h = Self {
            config: config.clone(),
            commands: HashMap::new(),
        };
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
        wrap_command!(h.commands, ["force_close", "fc"], force_close);
        wrap_command!(h.commands, ["add_staff", "as"], add_staff);
        wrap_command!(h.commands, ["remove_staff", "rs"], remove_staff);
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
        .ok_or_else(common::database_connection_failed)?;

    let error_handler = config
        .error_handler
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    if let Some(guild_id) = msg.guild_id {
        let community_guild_id = config.bot.get_community_guild_id();
        if guild_id.get() != community_guild_id {
            let error_key = if config.bot.is_dual_mode() {
                "server.wrong_guild_dual"
            } else {
                "server.wrong_guild_single"
            };

            let error_msg = get_translated_message(
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

    let user_key = msg.author.id.get();
    let user_mutex = get_thread_lock(config, user_key);
    let guard = user_mutex.lock().await;

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

    drop(guard);
    Ok(())
}

#[async_trait]
impl EventHandler for GuildMessagesHandler {
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

            if let Some(command_func) = self.commands.get(command_name)
                && let Err(error) =
                    command_func(ctx.clone(), msg.clone(), self.config.clone()).await
                {
                    if let Some(error_handler) = &self.config.error_handler {
                        let _ = error_handler.reply_with_error(&ctx, &msg, &error).await;
                    } else {
                        eprintln!("Command error: {}", error);
                    }
                }
            return;
        }
        return;
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        match event.author {
            Some(user) => {
                if user.bot {
                    return;
                }
            }
            None => {
                eprintln!("Message update event without author");
                return;
            }
        };

        if let Some(_channel_id) = event
            .channel_id
            .to_channel(&ctx.http)
            .await
            .ok()
            .and_then(|channel| channel.private())
        {
            let pool = match &self.config.db_pool {
                Some(p) => p,
                None => return,
            };

            let message = match get_thread_message_by_dm_message_id(event.id, pool).await {
                Ok(message) => message,
                Err(e) => {
                    eprintln!("Failed to get thread message by DM message ID: {}", e);
                    return;
                }
            };

            if let Some(thread) =
                get_thread_by_user_id(UserId::new(message.user_id as u64), pool).await
                && let Some(content) = event.content {
                    let inbox_builder = MessageBuilder::user_message(
                        &ctx,
                        &self.config,
                        UserId::new(message.user_id as u64),
                        message.user_name,
                    )
                    .content(content.clone());
                    let edit_msg = inbox_builder.build_edit_message().await;

                    let channel_id_parse = match thread.channel_id.parse::<u64>() {
                        Ok(id) => ChannelId::new(id),
                        Err(e) => {
                            eprintln!("Failed to parse channel ID: {}", e);
                            return;
                        }
                    };

                    if let Some(inbox_message_id) = message.inbox_message_id {
                        
                        if let Err(e) =
                            edit_inbox_message(&ctx, channel_id_parse, &inbox_message_id, edit_msg)
                                .await
                        {
                            eprintln!("Failed to edit mirrored staff message: {}", e);
                            return;
                        }
                        
                        if self.config.logs.show_log_on_edit {
                            let old_content: String = if let Some(old) = old_if_available {
                                old.content
                            } else {
                                String::new()
                            };
                            let before = if old_content.is_empty() {
                                "`(inconnu)`".to_string()
                            } else {
                                format!("`{}`", old_content)
                            };
                            let after = format!("`{}`", content.clone());

                            let guild_id = self.config.bot.get_community_guild_id();
                            let message_link = format!(
                                "https://discord.com/channels/{}/{}/{}",
                                guild_id,
                                channel_id_parse.get(),
                                inbox_message_id
                            );

                            let mut params = HashMap::new();
                            params.insert("before".to_string(), before);
                            params.insert("after".to_string(), after);
                            params.insert("link".to_string(), message_link);

                            let _ = MessageBuilder::system_message(&ctx, &self.config)
                                .translated_content(
                                    "edit.modification_from_user",
                                    Some(&params),
                                    Some(UserId::new(message.user_id as u64)),
                                    Some(guild_id),
                                )
                                .await
                                .to_channel(channel_id_parse)
                                .send()
                                .await;
                        }

                        let _ = update_message_content(&inbox_message_id, &content, pool).await;
                    }
                }
        }
    }
}
