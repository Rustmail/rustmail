use crate::commands::add_reminder::text_command::add_reminder::add_reminder;
use crate::commands::add_staff::text_command::add_staff::add_staff;
use crate::commands::alert::text_command::alert::alert;
use crate::commands::anonreply::text_command::anonreply::anonreply;
use crate::commands::close::text_command::close::close;
use crate::commands::delete::text_command::delete::delete;
use crate::commands::edit::message_ops::edit_inbox_message;
use crate::commands::edit::text_command::edit::edit;
use crate::commands::force_close::text_command::force_close::force_close;
use crate::commands::help::text_command::help::help;
use crate::commands::id::text_command::id::id;
use crate::commands::logs::text_command::logs::logs;
use crate::commands::move_thread::text_command::move_thread::move_thread;
use crate::commands::new_thread::text_command::new_thread::new_thread;
use crate::commands::recover::text_command::recover::recover;
use crate::commands::remove_reminder::text_command::remove_reminder::remove_reminder;
use crate::commands::remove_staff::text_command::remove_staff::remove_staff;
use crate::commands::reply::text_command::reply::reply;
use crate::config::Config;
use crate::db::messages::get_thread_message_by_dm_message_id;
use crate::db::operations::messages::get_thread_message_by_message_id;
use crate::db::operations::{
    delete_message as db_delete_message, update_message_numbers_after_deletion,
};
use crate::db::operations::{get_thread_channel_by_user_id, thread_exists, update_message_content};
use crate::db::threads::get_thread_by_user_id;
use crate::errors::{ModmailResult, common};
use crate::i18n::get_translated_message;
use crate::types::logs::PaginationStore;
use crate::utils::message::message_builder::MessageBuilder;
use crate::utils::thread::get_thread_lock::get_thread_lock;
use crate::utils::thread::send_to_thread::send_to_thread;
use crate::{modules::threads::create_channel, utils::wrap_command};
use serenity::all::{GuildId, MessageId, UserId};
use serenity::{
    all::{ChannelId, Context, EventHandler, Message, MessageUpdateEvent},
    async_trait,
};
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use tokio::sync::watch::Receiver;

static SUPPRESSED_DELETES: LazyLock<Mutex<HashSet<u64>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

type CommandFunc = Arc<StaticCommandFunc>;
type StaticCommandFunc = dyn Fn(
        Context,
        Message,
        Config,
        Receiver<bool>,
        PaginationStore,
    ) -> Pin<Box<dyn Future<Output = ModmailResult<()>> + Send>>
    + Send
    + Sync
    + 'static;

pub struct GuildMessagesHandler {
    pub config: Config,
    pub commands: HashMap<String, CommandFunc>,
    pub shutdown: Receiver<bool>,
    pub pagination: PaginationStore,
}

impl GuildMessagesHandler {
    pub fn new(config: &Config, shutdown: Receiver<bool>, pagination: PaginationStore) -> Self {
        let mut h = Self {
            config: config.clone(),
            commands: HashMap::new(),
            pagination,
            shutdown,
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
        wrap_command!(h.commands, ["force_close", "fc"], force_close);
        wrap_command!(h.commands, ["add_staff", "as"], add_staff);
        wrap_command!(h.commands, ["remove_staff", "rs"], remove_staff);
        wrap_command!(h.commands, "id", id);
        wrap_command!(h.commands, "help", help);
        wrap_command!(h.commands, ["add_reminder", "add_rap"], add_reminder);
        wrap_command!(h.commands, ["remove_reminder", "rr"], remove_reminder);
        wrap_command!(h.commands, "logs", logs);
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
            let _ = error_handler
                .reply_to_msg_with_error(ctx, msg, &error)
                .await;
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
                let _ = error_handler
                    .reply_to_msg_with_error(ctx, msg, &error)
                    .await;
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
                    let _ = error_handler
                        .reply_to_msg_with_error(&ctx, &msg, &error)
                        .await;
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
                && let Err(error) = command_func(
                    ctx.clone(),
                    msg.clone(),
                    self.config.clone(),
                    self.shutdown.clone(),
                    self.pagination.clone(),
                )
                .await
            {
                if let Some(error_handler) = &self.config.error_handler {
                    let _ = error_handler
                        .reply_to_msg_with_error(&ctx, &msg, &error)
                        .await;
                } else {
                    eprintln!("Command error: {}", error);
                }
            }
            return;
        }
        return;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        _channel_id: ChannelId,
        deleted_message_id: MessageId,
        _guild_id: Option<GuildId>,
    ) {
        {
            let mut suppressed = SUPPRESSED_DELETES.lock().unwrap();
            if suppressed.remove(&deleted_message_id.get()) {
                return;
            }
        }
        let pool = match &self.config.db_pool {
            Some(p) => p,
            None => return,
        };

        let message_entry =
            match get_thread_message_by_message_id(&deleted_message_id.to_string(), pool).await {
                Ok(m) => m,
                Err(_) => return,
            };

        let thread_opt =
            get_thread_by_user_id(UserId::new(message_entry.user_id as u64), pool).await;
        let thread = match thread_opt {
            Some(t) => t,
            None => return,
        };

        if self.config.logs.show_log_on_delete {
            let guild_id = self.config.bot.get_community_guild_id();
            let mut params = HashMap::new();

            params.insert(
                "content".to_string(),
                format!("`{}`", message_entry.content.clone()),
            );
            params.insert(
                "userid".to_string(),
                format!("<@{}>", message_entry.user_id),
            );

            let is_staff_message = message_entry.message_number.is_some();
            let key = if is_staff_message {
                "delete.removed_by_staff"
            } else {
                "delete.removed_by_user"
            };

            let _ = MessageBuilder::system_message(&ctx, &self.config)
                .translated_content(
                    key,
                    Some(&params),
                    Some(UserId::new(message_entry.user_id as u64)),
                    Some(guild_id),
                )
                .await
                .to_channel(ChannelId::new(
                    thread.channel_id.parse::<u64>().unwrap_or(0),
                ))
                .send(false)
                .await;
        }

        if let Some(num) = message_entry.message_number {
            let _ = update_message_numbers_after_deletion(&thread.channel_id, num, pool).await;
        }

        let _ = db_delete_message(&deleted_message_id.to_string(), pool).await;
    }

    async fn message_delete_bulk(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        multiple_deleted_messages_ids: Vec<MessageId>,
        guild_id: Option<GuildId>,
    ) {
        for mid in multiple_deleted_messages_ids {
            self.message_delete(ctx.clone(), channel_id, mid, guild_id)
                .await;
        }
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
                && let Some(content) = event.content
            {
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
                            .send(false)
                            .await;
                    }

                    let _ = update_message_content(&inbox_message_id, &content, pool).await;
                }
            }
        }
    }
}
