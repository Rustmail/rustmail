pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::types::*;
use crate::prelude::utils::*;
use chrono::Utc;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, GuildId, UserId};
use std::collections::HashMap;

pub struct ReplyCommand;

impl Command for ReplyCommand {
    type Args = ReplyArgs;

    fn name(&self) -> &'static str {
        "reply"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["r"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.reply", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("reply")
                .description(translate("slash_command.reply_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "message",
                        translate("slash_command.reply_message_argument_description").await,
                    )
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "snippet",
                        translate("slash_command.reply_snippet_argument_description").await,
                    )
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Attachment,
                        "attachment",
                        translate("slash_command.reply_attachment_argument_description").await,
                    )
                    .required(false),
                )
                .add_option(CreateCommandOption::new(
                    CommandOptionType::Boolean,
                    "anonymous",
                    translate("slash_command.reply_anonymous_argument_description").await,
                )),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<ReplyArgs> {
        parse_reply_text(args, false)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<ReplyArgs> {
        parse_reply_slash(options)
    }

    async fn run(
        &self,
        args: ReplyArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        send_staff_reply(args, invocation, config).await
    }
}

pub struct AnonReplyCommand;

impl Command for AnonReplyCommand {
    type Args = ReplyArgs;

    fn name(&self) -> &'static str {
        "anonreply"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["ar"]
    }

    fn surfaces(&self) -> Surfaces {
        Surfaces::TextOnly
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.reply", None, None, None, None).await
    }

    fn parse_text(&self, args: &str) -> ModmailResult<ReplyArgs> {
        parse_reply_text(args, true)
    }

    async fn run(
        &self,
        args: ReplyArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        send_staff_reply(args, invocation, config).await
    }
}

pub async fn send_staff_reply(
    args: ReplyArgs,
    invocation: &Invocation,
    config: &Config,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;
    let ctx = invocation.ctx();
    let thread = fetch_thread(pool, &invocation.channel_id().to_string()).await?;

    let content = match &args.snippet {
        Some(key) => match get_snippet_by_key(key, pool).await? {
            Some(snippet) => Some(snippet.content),
            None => {
                return Err(ModmailError::Command(CommandError::SnippetNotFound(
                    key.clone(),
                )));
            }
        },
        None => args.content,
    };
    let mut attachments = args.attachments;
    if let Some(message) = invocation.message() {
        attachments.extend(message.attachments.iter().cloned());
    }
    let intent = extract_intent(content, &attachments)
        .await
        .ok_or(ModmailError::Message(MessageError::MessageEmpty))?;

    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    if community_guild_id.member(&ctx.http, user_id).await.is_err() {
        return Err(ModmailError::Thread(ThreadError::UserNotInTheServer(
            thread.user_name.clone(),
        )));
    }

    let message_number = allocate_next_message_number(&thread.id, pool)
        .await
        .map_err(|_| validation_failed("Failed to allocate message number"))?;

    let mut ticket_status = get_thread_status(&thread.id, pool)
        .await
        .ok_or_else(|| validation_failed("Failed to get thread status"))?;
    ticket_status.last_message_by = TicketAuthor::Staff;
    ticket_status.last_message_at = Utc::now().timestamp();
    update_thread_status_db(&thread.id, &ticket_status, pool).await?;

    if let Some(message) = invocation.message() {
        let _ = message.delete(&ctx.http).await;
    }

    let author = invocation.user();
    let mut staff_reply = MessageBuilder::begin_staff_reply(
        ctx,
        config,
        thread.id.clone(),
        author.id,
        author.name.clone(),
        message_number,
    )
    .anonymous(args.anonymous)
    .to_thread(invocation.channel_id())
    .to_user(user_id);
    staff_reply = match intent {
        ReplyIntent::Text(text) => staff_reply.content(text),
        ReplyIntent::Attachments(files) => staff_reply.add_attachments(files),
        ReplyIntent::TextAndAttachments(text, files) => {
            staff_reply.content(text).add_attachments(files)
        }
    };

    let (_, dm_message) = staff_reply
        .send_and_record(invocation, pool)
        .await
        .map_err(|_| validation_failed("Failed to send to thread"))?;
    if dm_message.is_none() {
        return Err(ModmailError::Command(CommandError::SendDmFailed));
    }

    mark_thread_engaged(&thread.id, pool).await;

    if config.notifications.show_success_on_reply {
        let mut params = HashMap::new();
        params.insert("number".to_string(), message_number.to_string());
        let _ = invocation
            .reply_translated(config, "success.message_sent", Some(&params))
            .await;
    }

    Ok(())
}
