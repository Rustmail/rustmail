use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::sync::Arc;

pub struct ForceCloseCommand;

#[async_trait::async_trait]
impl RegistrableCommand for ForceCloseCommand {
    fn name(&self) -> &'static str {
        "force_close"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(config, "help.force_close", None, None, None, None).await
        }.boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.force_close_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new("force_close").description(cmd_desc)]
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

            if !is_a_ticket_channel(command.channel_id, db_pool).await {
                match command.channel_id.to_channel(&ctx.http).await {
                    Ok(channel) => {
                        let guild_channel = match channel.guild() {
                            Some(guild_channel) => guild_channel,
                            None => {
                                return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
                            }
                        };

                        if let Some(category_id) = guild_channel.parent_id {
                            if category_id == config.thread.inbox_category_id {
                                delete_channel(&ctx, command.channel_id).await?;
                            } else {
                                return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
                            }
                        }
                    }
                    Err(_) => {
                        return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
                    }
                }
            }

            let thread = get_thread_by_channel_id(&command.channel_id.to_string(), db_pool).await;

            match is_orphaned_thread_channel(command.channel_id, db_pool).await {
                Ok(res) => {
                    if !res {
                        return Err(ModmailError::Thread(ThreadError::UserStillInServer));
                    }

                    if let Some(thread_info) = thread {
                        if config.bot.enable_logs {
                            if let Some(logs_channel_id) = config.bot.logs_channel_id {
                                let base_url = config
                                    .bot
                                    .redirect_url
                                    .trim_end_matches("/api/auth/callback")
                                    .trim_end_matches('/');

                                let panel_url =
                                    format!("{}/panel/tickets/{}", base_url, thread_info.id);

                                let mut params = std::collections::HashMap::new();
                                params
                                    .insert("username".to_string(), thread_info.user_name.clone());
                                params
                                    .insert("user_id".to_string(), thread_info.user_id.to_string());
                                params.insert("panel_url".to_string(), panel_url);

                                let _ = MessageBuilder::system_message(&ctx, &config)
                                    .translated_content(
                                        "logs.ticket_closed",
                                        Some(&params),
                                        Some(command.user.id),
                                        command.guild_id.map(|g| g.get()),
                                    )
                                    .await
                                    .to_channel(serenity::all::ChannelId::new(logs_channel_id))
                                    .send_interaction_followup(&command, true)
                                    .await;
                            }
                        }
                    }

                    delete_channel(&ctx, command.channel_id).await
                }
                Err(..) => Err(ModmailError::Database(DatabaseError::QueryFailed(
                    "Failed to check if thread channel is orphaned".to_string(),
                ))),
            }
        })
    }
}
