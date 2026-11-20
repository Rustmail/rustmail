use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::utils::*;
use serenity::all::{ChannelId, CommandInteraction, Context, GuildId, Message, User, UserId};
use std::collections::HashMap;

pub struct Invocation {
    ctx: Context,
    source: Source,
}

enum Source {
    Slash {
        command: Box<CommandInteraction>,
        ephemeral: bool,
    },
    Text(Box<Message>),
}

#[allow(dead_code)]
impl Invocation {
    pub fn slash(ctx: Context, command: CommandInteraction, ephemeral: bool) -> Self {
        Self {
            ctx,
            source: Source::Slash {
                command: Box::new(command),
                ephemeral,
            },
        }
    }

    pub fn text(ctx: Context, message: Message) -> Self {
        Self {
            ctx,
            source: Source::Text(Box::new(message)),
        }
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn user(&self) -> &User {
        match &self.source {
            Source::Slash { command, .. } => &command.user,
            Source::Text(message) => &message.author,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user().id
    }

    pub fn channel_id(&self) -> ChannelId {
        match &self.source {
            Source::Slash { command, .. } => command.channel_id,
            Source::Text(message) => message.channel_id,
        }
    }

    pub fn guild_id(&self) -> Option<GuildId> {
        match &self.source {
            Source::Slash { command, .. } => command.guild_id,
            Source::Text(message) => message.guild_id,
        }
    }

    pub fn message(&self) -> Option<&Message> {
        match &self.source {
            Source::Text(message) => Some(message),
            Source::Slash { .. } => None,
        }
    }

    pub fn interaction(&self) -> Option<&CommandInteraction> {
        match &self.source {
            Source::Slash { command, .. } => Some(command),
            Source::Text(_) => None,
        }
    }

    pub async fn reply(&self, builder: MessageBuilder<'_>) -> ModmailResult<Message> {
        self.post(builder, true).await
    }

    pub async fn reply_unrecorded(&self, builder: MessageBuilder<'_>) -> ModmailResult<Message> {
        self.post(builder, false).await
    }

    async fn post(&self, builder: MessageBuilder<'_>, record: bool) -> ModmailResult<Message> {
        match &self.source {
            Source::Slash { command, ephemeral } => {
                builder
                    .ephemeral(*ephemeral)
                    .send_interaction_followup(command, record)
                    .await
            }
            Source::Text(message) => builder.to_channel(message.channel_id).send(record).await,
        }
    }

    pub async fn edit_reply(
        &self,
        reply: &mut Message,
        builder: MessageBuilder<'_>,
    ) -> ModmailResult<()> {
        match &self.source {
            Source::Slash { command, .. } => {
                let edit = builder.build_interaction_message_followup().await;
                *reply = command
                    .edit_followup(&self.ctx.http, reply.id, edit)
                    .await?;
            }
            Source::Text(_) => {
                let edit = builder.build_edit_message().await;
                reply.edit(&self.ctx.http, edit).await?;
            }
        }
        Ok(())
    }

    pub async fn reply_translated(
        &self,
        config: &Config,
        key: &str,
        params: Option<&HashMap<String, String>>,
    ) -> ModmailResult<Message> {
        let builder = MessageBuilder::system_message(&self.ctx, config)
            .translated_content(
                key,
                params,
                Some(self.user_id()),
                self.guild_id().map(|g| g.get()),
            )
            .await;
        self.reply(builder).await
    }

    pub async fn reply_error(&self, config: &Config, error: &ModmailError) {
        let Some(error_handler) = &config.error_handler else {
            eprintln!("Command error: {}", error);
            return;
        };

        let _ = match &self.source {
            Source::Slash { command, .. } => {
                error_handler
                    .reply_to_command_with_error(&self.ctx, command, error)
                    .await
            }
            Source::Text(message) => {
                error_handler
                    .reply_to_msg_with_error(&self.ctx, message, error)
                    .await
            }
        };
    }
}
