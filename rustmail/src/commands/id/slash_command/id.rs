use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{CommandInteraction, Context, ResolvedOption};
use serenity::builder::CreateCommand;
use std::sync::Arc;

pub struct IdCommand;

#[async_trait::async_trait]
impl RegistrableCommand for IdCommand {
    fn name(&self) -> &'static str {
        "id"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.id", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.id_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new("id").description(cmd_desc)]
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
            let pool = match &config.db_pool {
                Some(pool) => pool,
                None => {
                    return Err(ModmailError::Database(DatabaseError::ConnectionFailed));
                }
            };

            defer_response(&ctx, &command).await?;

            if !is_a_ticket_channel(command.channel_id, pool).await {
                return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
            }

            let thread = match get_thread_by_channel_id(&command.channel_id.to_string(), pool).await
            {
                Some(thread) => thread,
                None => {
                    return Err(ModmailError::Thread(ThreadError::ThreadNotFound));
                }
            };

            let mut params = std::collections::HashMap::new();
            params.insert("user".to_string(), format!("<@{}>", thread.user_id));
            params.insert(
                "id".to_string(),
                format!("||{}||", thread.user_id.to_string()),
            );

            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content("id.show_id", Some(&params), None, None)
                .await
                .to_channel(command.channel_id)
                .send_interaction_followup(&command, true)
                .await;

            Ok(())
        })
    }
}
