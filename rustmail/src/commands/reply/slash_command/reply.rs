use crate::modules::update_thread_status_ui;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::types::*;
use crate::prelude::utils::*;
use chrono::Utc;
use serenity::FutureExt;
use serenity::all::{
    Attachment, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, GuildId, ResolvedOption, UserId,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ReplyCommand;

#[async_trait::async_trait]
impl RegistrableCommand for ReplyCommand {
    fn name(&self) -> &'static str {
        "reply"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.reply", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.reply_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let message_desc = get_translated_message(
                &config,
                "slash_command.reply_message_argument_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let attachments_desc = get_translated_message(
                &config,
                "slash_command.reply_attachment_argument_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let anonymous_desc = get_translated_message(
                &config,
                "slash_command.reply_anonymous_argument_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("reply")
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "message",
                            message_desc,
                        )
                        .required(false),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "snippet",
                            "Use a snippet instead of typing a message",
                        )
                        .required(false),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::Attachment,
                            "attachment",
                            attachments_desc,
                        )
                        .required(false),
                    )
                    .add_option(CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "anonymous",
                        anonymous_desc,
                    )),
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        _handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let db_pool = config
                .db_pool
                .as_ref()
                .ok_or_else(database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            let mut content: Option<String> = None;
            let mut snippet_key: Option<String> = None;
            let mut attachments: Vec<Attachment> = Vec::new();
            let mut anonymous: bool = false;

            for option in &command.data.options {
                match &option.value {
                    CommandDataOptionValue::String(val) if option.name == "message" => {
                        content = Some(val.clone());
                    }
                    CommandDataOptionValue::String(val) if option.name == "snippet" => {
                        snippet_key = Some(val.clone());
                    }
                    CommandDataOptionValue::Attachment(att_id) if option.name == "attachment" => {
                        if let Some(att) = command.data.resolved.attachments.get(att_id) {
                            attachments.push(att.clone());
                        }
                    }
                    CommandDataOptionValue::Boolean(anonym) if option.name == "anonymous" => {
                        anonymous = *anonym;
                    }
                    _ => {}
                }
            }
            
            if let Some(key) = snippet_key {
                match get_snippet_by_key(&key, db_pool).await? {
                    Some(snippet) => {
                        content = Some(snippet.content);
                    }
                    None => {
                        return Err(ModmailError::Command(CommandError::CommandFailed(
                            format!("Snippet '{}' not found", key),
                        )));
                    }
                }
            }

            let intent = extract_intent(content, &attachments).await;

            let Some(intent) = intent else {
                return Err(ModmailError::Message(MessageError::MessageEmpty));
            };

            let thread = fetch_thread(db_pool, &command.channel_id.to_string()).await?;
            let user_id = UserId::new(thread.user_id as u64);
            let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
            let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

            if !user_still_member {
                return Err(ModmailError::Thread(ThreadError::UserNotInTheServer));
            }

            let next_message_number = allocate_next_message_number(&thread.id, db_pool)
                .await
                .map_err(|_| validation_failed("Failed to allocate message number"))?;

            let mut ticket_status = match get_thread_status(&thread.id, db_pool).await {
                Some(status) => status,
                None => {
                    return Err(validation_failed("Failed to get thread status"));
                }
            };

            ticket_status.last_message_by = TicketAuthor::Staff;
            ticket_status.last_message_at = Utc::now().timestamp();
            update_thread_status_db(&thread.id, &ticket_status, db_pool).await?;

            tokio::spawn({
                let ctx = ctx.clone();
                async move {
                    let _ = update_thread_status_ui(&ctx, &ticket_status).await;
                }
            });

            let mut sr = MessageBuilder::begin_staff_reply(
                &ctx,
                &config,
                thread.id.clone(),
                command.user.id,
                command.user.name.clone(),
                next_message_number,
            )
            .to_thread(command.channel_id)
            .anonymous(anonymous)
            .to_user(user_id);

            match intent {
                ReplyIntent::Text(text) => {
                    sr = sr.content(text);
                }
                ReplyIntent::Attachments(files) => {
                    sr = sr.add_attachments(files);
                }
                ReplyIntent::TextAndAttachments(text, files) => {
                    sr = sr.content(text).add_attachments(files);
                }
            }

            let (_, dm_msg_opt) = match sr.send_command_and_record(&command, db_pool).await {
                Ok(tuple) => tuple,
                Err(_) => {
                    return Err(validation_failed("Failed to send to thread"));
                }
            };

            if dm_msg_opt.is_none() {
                return Err(ModmailError::Command(CommandError::SendDmFailed));
            }

            if config.notifications.show_success_on_reply {
                let mut params = HashMap::new();
                params.insert("number".to_string(), next_message_number.to_string());

                let response = MessageBuilder::system_message(&ctx, &config)
                    .translated_content(
                        "success.message_sent",
                        Some(&params),
                        Some(command.user.id),
                        command.guild_id.map(|g| g.get()),
                    )
                    .await
                    .to_channel(command.channel_id)
                    .build_interaction_message_followup()
                    .await;

                let _ = command.create_followup(&ctx.http, response).await;
            }

            Ok(())
        })
    }
}
