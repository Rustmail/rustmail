use crate::config::Config;
use crate::i18n::get_translated_message;
use crate::utils::hex_string_to_int::hex_string_to_int;
use serenity::all::{ChannelId, Colour, Context, CreateAttachment, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, EditMessage, Message, Timestamp, UserId};
use std::collections::HashMap;
use sqlx::SqlitePool;
use crate::db::operations::{insert_staff_message, insert_user_message_with_ids};

#[derive(Debug, Clone)]
pub enum MessageSender {
    User {
        user_id: UserId,
        username: String,
    },
    Staff {
        user_id: UserId,
        username: String,
        role: Option<String>,
        message_number: Option<u64>,
        anonymous: bool,
    },
    System {
        user_id: UserId,
        username: String,
    },
}

#[derive(Debug, Clone)]
pub enum MessageTarget {
    Channel(ChannelId),
    User(UserId),
    Reply(Message),
}

#[derive(Debug, Clone)]
pub struct MessageBuilder<'a> {
    ctx: &'a Context,
    config: &'a Config,
    content: String,
    sender: Option<MessageSender>,
    target: Option<MessageTarget>,
    attachments: Vec<CreateAttachment>,
    force_embed: Option<bool>,
    custom_color: Option<u32>,
    footer_text: Option<String>,
    ephemeral: bool,
    bot_user_id: UserId,
}

impl<'a> MessageBuilder<'a> {
    pub fn new(ctx: &'a Context, config: &'a Config) -> Self {
        let bot_user_id = ctx.cache.current_user().id;
        Self {
            ctx,
            config,
            content: String::new(),
            sender: None,
            target: None,
            attachments: Vec::new(),
            force_embed: None,
            custom_color: None,
            footer_text: None,
            ephemeral: false,
            bot_user_id,
        }
    }

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = content.into();
        self
    }

    pub fn append_content<S: AsRef<str>>(mut self, content: S) -> Self {
        if !self.content.is_empty() {
            self.content.push('\n');
        }
        self.content.push_str(content.as_ref());
        self
    }

    pub fn as_user(mut self, user_id: UserId, username: String) -> Self {
        self.sender = Some(MessageSender::User { user_id, username });
        self
    }

    pub fn as_staff(mut self, user_id: UserId, username: String) -> Self {
        self.sender = Some(MessageSender::Staff {
            user_id,
            username,
            role: None,
            message_number: None,
            anonymous: false,
        });
        self
    }

    pub fn as_anonymous_staff(mut self, user_id: UserId) -> Self {
        let bot_name = self.ctx.cache.current_user().name.clone();

        self.sender = Some(MessageSender::Staff {
            user_id,
            username: bot_name,
            role: None,
            message_number: None,
            anonymous: true,
        });
        self
    }

    pub fn as_system(mut self, user_id: UserId, username: String) -> Self {
        self.sender = Some(MessageSender::System { user_id, username });
        self
    }

    pub fn with_role<S: Into<String>>(mut self, role: S) -> Self {
        if let Some(MessageSender::Staff {
            role: r, ..
        }) = &mut self.sender
        {
            *r = Some(role.into());
        }
        self
    }

    pub fn with_message_number(mut self, number: u64) -> Self {
        if let Some(MessageSender::Staff {
            message_number: n,
            ..
        }) = &mut self.sender
        {
            *n = Some(number);
        }
        self
    }

    pub fn to_channel(mut self, channel_id: ChannelId) -> Self {
        self.target = Some(MessageTarget::Channel(channel_id));
        self
    }

    pub fn to_user(mut self, user_id: UserId) -> Self {
        self.target = Some(MessageTarget::User(user_id));
        self
    }

    pub fn reply_to(mut self, message: Message) -> Self {
        self.target = Some(MessageTarget::Reply(message));
        self
    }

    pub fn add_attachment(mut self, attachment: CreateAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn add_attachments(mut self, attachments: Vec<CreateAttachment>) -> Self {
        self.attachments.extend(attachments);
        self
    }

    pub fn force_embed(mut self, force: bool) -> Self {
        self.force_embed = Some(force);
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.custom_color = Some(color);
        self
    }

    pub fn footer<S: Into<String>>(mut self, text: S) -> Self {
        self.footer_text = Some(text.into());
        self
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }

    pub async fn translated_content(
        mut self,
        key: &str,
        params: Option<&HashMap<String, String>>,
        user_id: Option<UserId>,
        guild_id: Option<u64>,
    ) -> Self {
        let content =
            get_translated_message(self.config, key, params, user_id, guild_id, None).await;
        self.content = content;
        self
    }

    async fn should_use_embed(&self) -> bool {
        self.force_embed
            .unwrap_or(self.config.thread.embedded_message)
    }

    async fn get_message_color(&self) -> u32 {
        if let Some(color) = self.custom_color {
            return color;
        }

        match &self.sender {
            Some(MessageSender::User { .. }) => {
                hex_string_to_int(&self.config.thread.user_message_color) as u32
            }
            Some(MessageSender::Staff { .. }) => {
                hex_string_to_int(&self.config.thread.staff_message_color) as u32
            }
            Some(MessageSender::System { .. }) => {
                hex_string_to_int(&self.config.thread.system_message_color) as u32
            }
            None => 0x36393F,
        }
    }

    async fn get_user_avatar_url(&self, user_id: UserId) -> String {
        match user_id.to_user(&self.ctx.http).await {
            Ok(user) => user
                .avatar_url()
                .unwrap_or_else(|| user.default_avatar_url()),
            Err(_) => String::new(),
        }
    }

    async fn build_embed(&self) -> CreateEmbed {
        let mut embed = CreateEmbed::new()
            .color(Colour::new(self.get_message_color().await))
            .timestamp(Timestamp::now());

        if !self.content.trim().is_empty() {
            let formatted_content = if self.config.thread.block_quote {
                format!(">>> {}", self.content)
            } else {
                self.content.clone()
            };
            embed = embed.description(formatted_content);
        }

        if let Some(sender) = &self.sender {
            match sender {
                MessageSender::User { user_id, username } => {
                    let avatar_url = self.get_user_avatar_url(*user_id).await;
                    embed = embed.author(CreateEmbedAuthor::new(username).icon_url(avatar_url));
                }
                MessageSender::Staff {
                    user_id,
                    username,
                    role,
                    anonymous,
                    ..
                } => {
                    let display_name = if *anonymous {
                        self.ctx.cache.current_user().name.clone()
                    } else if let Some(role) = role {
                        format!("{}  ({})", username, role)
                    } else {
                        username.clone()
                    };

                    let avatar_url = if *anonymous {
                        self.get_user_avatar_url(self.bot_user_id).await
                    } else {
                        self.get_user_avatar_url(*user_id).await
                    };

                    embed = embed.author(CreateEmbedAuthor::new(display_name).icon_url(avatar_url));
                }
                MessageSender::System { user_id, username } => {
                    let avatar_url = self.get_user_avatar_url(*user_id).await;
                    embed = embed.author(CreateEmbedAuthor::new(username).icon_url(avatar_url));
                }
            }
        }

        let mut footer_parts = Vec::new();

        if let Some(MessageSender::Staff {
            message_number: Some(num),
            ..
        }) = &self.sender
        {
            let mut params = HashMap::new();
            params.insert("number".to_string(), num.to_string());
            params.insert("prefix".to_string(), self.config.command.prefix.clone());

            if let Ok(footer_text) = tokio::task::spawn_blocking({
                let config = self.config.clone();
                let params = params.clone();
                move || {
                    tokio::runtime::Handle::current().block_on(get_translated_message(
                        &config,
                        "reply_numbering.footer",
                        Some(&params),
                        None,
                        None,
                        None,
                    ))
                }
            })
            .await
            {
                footer_parts.push(footer_text);
            }
        }

        if let Some(custom_footer) = &self.footer_text {
            footer_parts.push(custom_footer.clone());
        }

        if !footer_parts.is_empty() {
            embed = embed.footer(CreateEmbedFooter::new(footer_parts.join(" • ")));
        }

        embed
    }

    fn build_plain_message(&self) -> String {
        if self.content.trim().is_empty() {
            return String::new();
        }

        let base_message = match &self.sender {
            Some(MessageSender::User { username, .. }) => {
                format!("**{}**: {}", username, self.content)
            }
            Some(MessageSender::Staff {
                username,
                role,
                message_number,
                anonymous,
                ..
            }) => {
                let display_name = if *anonymous {
                    self.ctx.cache.current_user().name.clone()
                } else if let Some(role) = role {
                    format!("{}  ({})", username, role)
                } else {
                    username.clone()
                };

                let mut msg = format!("**{}**: {}", display_name, self.content);

                if let Some(num) = message_number {
                    let mut params = HashMap::new();
                    params.insert("number".to_string(), num.to_string());
                    params.insert("prefix".to_string(), self.config.command.prefix.clone());

                    let config_clone = self.config.clone();
                    let params_clone = params.clone();
                    let footer = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(get_translated_message(
                            &config_clone,
                            "reply_numbering.text_footer",
                            Some(&params_clone),
                            None,
                            None,
                            None,
                        ))
                    });
                    msg.push_str(&format!("\n\n{}", footer));
                }

                msg
            }
            Some(MessageSender::System { username, .. }) => {
                format!("**{}**: {}", username, self.content)
            }
            None => self.content.clone(),
        };

        if let Some(custom_footer) = &self.footer_text {
            format!("{}\n\n*{}*", base_message, custom_footer)
        } else {
            base_message
        }
    }

    pub async fn send(self) -> Result<Message, serenity::Error> {
        let target = self.target.clone()
            .ok_or_else(|| serenity::Error::Other("No target specified for message"))?;

        let message = self.build_create_message().await;

        match target {
            MessageTarget::Channel(channel_id) => {

                let message = match channel_id.send_message(&self.ctx.http, message).await {
                    Ok(message) => message,
                    Err(err) => return Err(err)
                };

                Ok(message)
            }
            MessageTarget::User(user_id) => {
                let dm_channel = user_id.create_dm_channel(&self.ctx.http).await?;

                let message = match dm_channel.send_message(&self.ctx.http, message).await {
                    Ok(message) => message,
                    Err(err) => return Err(err)
                };

                Ok(message)
            }
            MessageTarget::Reply(original_message) => {
                let message = match original_message
                    .channel_id
                    .send_message(&self.ctx.http, message)
                    .await
                {
                    Ok(message) => message,
                    Err(err) => return Err(err)
                };

                Ok(message)
            }
        }
    }

    pub async fn build_create_message(&self) -> CreateMessage {
        let mut message = CreateMessage::new();

        if self.should_use_embed().await {
            message = message.embed(self.build_embed().await);
        } else {
            let content = self.build_plain_message();
            if !content.is_empty() {
                message = message.content(content);
            }
        }

        for attachment in &self.attachments {
            message = message.add_file(attachment.clone());
        }

        message
    }

    pub async fn build_edit_message(&self) -> EditMessage {
        let mut message = EditMessage::new();

        if self.should_use_embed().await {
            message = message.embed(self.build_embed().await);
        } else {
            let content = self.build_plain_message();
            if !content.is_empty() {
                message = message.content(content);
            }
        }

        message
    }

    pub async fn build(self) -> CreateMessage {
        self.build_create_message().await
    }

    pub async fn send_and_forget(self) {
        if let Err(e) = self.send().await {
            eprintln!("Failed to send message: {}", e);
        }
    }
}

impl<'a> MessageBuilder<'a> {
    pub fn user_message(
        ctx: &'a Context,
        config: &'a Config,
        user_id: UserId,
        username: String,
    ) -> Self {
        Self::new(ctx, config).as_user(user_id, username)
    }

    pub fn staff_message(
        ctx: &'a Context,
        config: &'a Config,
        user_id: UserId,
        username: String,
    ) -> Self {
        Self::new(ctx, config).as_staff(user_id, username)
    }

    pub fn anonymous_staff_message(ctx: &'a Context, config: &'a Config, user_id: UserId) -> Self {
        Self::new(ctx, config).as_anonymous_staff(user_id)
    }

    pub fn system_message(ctx: &'a Context, config: &'a Config) -> Self {
        let bot_id = ctx.cache.current_user().id;
        let bot_name = ctx.cache.current_user().name.clone();
        Self::new(ctx, config).as_system(bot_id, bot_name)
    }

    pub async fn send_to_channel(
        ctx: &'a Context,
        config: &'a Config,
        channel_id: ChannelId,
        content: String,
    ) -> Result<Message, serenity::Error> {
        Self::system_message(ctx, config)
            .content(content)
            .to_channel(channel_id)
            .send()
            .await
    }

    pub async fn send_to_user(
        ctx: &'a Context,
        config: &'a Config,
        user_id: UserId,
        content: String,
    ) -> Result<Message, serenity::Error> {
        Self::system_message(ctx, config)
            .content(content)
            .to_user(user_id)
            .send()
            .await
    }

    pub async fn reply_to_message(
        ctx: &'a Context,
        config: &'a Config,
        message: Message,
        content: String,
    ) -> Result<Message, serenity::Error> {
        Self::system_message(ctx, config)
            .content(content)
            .reply_to(message)
            .send()
            .await
    }
}

pub struct StaffReply<'a> {
    ctx: &'a Context,
    config: &'a Config,
    thread_id: String,
    thread_channel: Option<ChannelId>,
    dm_user_id: Option<UserId>,
    staff_user_id: UserId,
    staff_username: String,
    is_anonymous: bool,
    message_number: u64,
    content: String,
    attachments: Vec<CreateAttachment>,
}

impl<'a> StaffReply<'a> {
    pub fn new(ctx: &'a Context, config: &'a Config, thread_id: String, staff_user_id: UserId, staff_username: String, message_number: u64) -> Self {
        Self {
            ctx,
            config,
            thread_id,
            thread_channel: None,
            dm_user_id: None,
            staff_user_id,
            staff_username,
            is_anonymous: false,
            message_number,
            content: String::new(),
            attachments: Vec::new(),
        }
    }

    pub fn anonymous(mut self, anonymous: bool) -> Self { self.is_anonymous = anonymous; self }
    pub fn to_thread(mut self, channel_id: ChannelId) -> Self { self.thread_channel = Some(channel_id); self }
    pub fn to_user(mut self, user_id: UserId) -> Self { self.dm_user_id = Some(user_id); self }
    pub fn content<S: Into<String>>(mut self, content: S) -> Self { self.content = content.into(); self }
    pub fn add_attachments(mut self, files: Vec<CreateAttachment>) -> Self { self.attachments.extend(files); self }

    pub async fn send_and_record(self, pool: &SqlitePool) -> Result<(Message, Option<Message>), serenity::Error> {
        let thread_channel = self.thread_channel.ok_or_else(|| serenity::Error::Other("No thread channel for StaffReply"))?;

        let mut top_role_name: Option<String> = None;
        if let Ok(channel) = thread_channel.to_channel(&self.ctx.http).await {
            use serenity::all::Channel;
            let guild_id_opt = match &channel {
                Channel::Guild(guild_channel) => Some(guild_channel.guild_id),
                _ => None,
            };

            if let Some(guild_id) = guild_id_opt {
                if let (Ok(member), Ok(roles_map)) = (
                    guild_id.member(&self.ctx.http, self.staff_user_id).await,
                    guild_id.roles(&self.ctx.http).await,
                ) {
                    top_role_name = member
                        .roles
                        .iter()
                        .filter_map(|rid| roles_map.get(rid))
                        .filter(|r| r.name != "@everyone")
                        .max_by_key(|r| r.position)
                        .map(|r| r.name.clone());
                }
            }
        }

        let mut thread_builder = if self.is_anonymous {
            MessageBuilder::anonymous_staff_message(self.ctx, self.config, self.staff_user_id)
        } else {
            MessageBuilder::staff_message(self.ctx, self.config, self.staff_user_id, self.staff_username.clone())
        };
        if !self.is_anonymous {
            if let Some(role_name) = &top_role_name {
                thread_builder = thread_builder.with_role(role_name.clone());
            }
        }

        thread_builder = thread_builder
            .content(self.content.clone())
            .with_message_number(self.message_number)
            .add_attachments(self.attachments.clone())
            .to_channel(thread_channel);

        let thread_msg = thread_builder.send().await?;

        let mut dm_msg_opt: Option<Message> = None;
        if let Some(dm_user) = self.dm_user_id {
            let mut dm_builder = if self.is_anonymous {
                MessageBuilder::anonymous_staff_message(self.ctx, self.config, self.staff_user_id)
            } else {
                MessageBuilder::staff_message(self.ctx, self.config, self.staff_user_id, self.staff_username.clone())
            };
            if !self.is_anonymous {
                if let Some(role_name) = &top_role_name {
                    dm_builder = dm_builder.with_role(role_name.clone());
                }
            }

            dm_builder = dm_builder
                .content(self.content.clone())
                .add_attachments(self.attachments.clone())
                .color(hex_string_to_int(&self.config.thread.staff_message_color) as u32)
                .to_user(dm_user);

            match dm_builder.send().await {
                Ok(m) => dm_msg_opt = Some(m),
                Err(e) => {
                    eprintln!("Failed to send DM to user: {}", e);
                }
            }
        }

        let dm_id_opt = dm_msg_opt.as_ref().map(|m| m.id.to_string());
        if let Err(e) = insert_staff_message(
            &thread_msg,
            dm_id_opt,
            &self.thread_id,
            self.staff_user_id,
            self.is_anonymous,
            pool,
            self.config,
            self.message_number,
        ).await {
            eprintln!("Error inserting staff message: {}", e);
        }

        Ok((thread_msg, dm_msg_opt))
    }
}

pub struct UserIncoming<'a> {
    ctx: &'a Context,
    config: &'a Config,
    thread_id: String,
    dm_msg: &'a Message,
    thread_channel: Option<ChannelId>,
    content: String,
    attachments: Vec<CreateAttachment>,
    is_anonymous: bool,
}

impl<'a> UserIncoming<'a> {
    pub fn new(ctx: &'a Context, config: &'a Config, thread_id: String, dm_msg: &'a Message) -> Self {
        Self { ctx, config, thread_id, dm_msg, thread_channel: None, content: String::new(), attachments: Vec::new(), is_anonymous: false }
    }
    pub fn to_thread(mut self, channel_id: ChannelId) -> Self { self.thread_channel = Some(channel_id); self }
    pub fn content<S: Into<String>>(mut self, content: S) -> Self { self.content = content.into(); self }
    pub fn add_attachments(mut self, files: Vec<CreateAttachment>) -> Self { self.attachments.extend(files); self }
    pub fn anonymous(mut self, anonymous: bool) -> Self { self.is_anonymous = anonymous; self }

    pub async fn send_and_record(self, pool: &SqlitePool) -> Result<Message, serenity::Error> {
        let thread_channel = self.thread_channel.ok_or_else(|| serenity::Error::Other("No thread channel for UserIncoming"))?;
        let builder = MessageBuilder::user_message(self.ctx, self.config, self.dm_msg.author.id, self.dm_msg.author.name.clone())
            .content(self.content)
            .add_attachments(self.attachments)
            .to_channel(thread_channel);
        let sent = builder.send().await?;
        if let Err(e) = insert_user_message_with_ids(self.dm_msg, &sent, &self.thread_id, self.is_anonymous, pool, self.config).await {
            eprintln!("Error inserting user message: {}", e);
        }
        Ok(sent)
    }
}

impl<'a> MessageBuilder<'a> {
    pub fn begin_staff_reply(ctx: &'a Context, config: &'a Config, thread_id: String, staff_user_id: UserId, staff_username: String, message_number: u64) -> StaffReply<'a> {
        StaffReply::new(ctx, config, thread_id, staff_user_id, staff_username, message_number)
    }

    pub fn begin_user_incoming(ctx: &'a Context, config: &'a Config, thread_id: String, dm_msg: &'a Message) -> UserIncoming<'a> {
        UserIncoming::new(ctx, config, thread_id, dm_msg)
    }
}
