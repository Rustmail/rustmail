use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use std::collections::HashMap;
use std::sync::Arc;

pub struct RecoverCommand;

#[async_trait::async_trait]
impl RegistrableCommand for RecoverCommand {
    fn name(&self) -> &'static str {
        "recover"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.recover", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.recover_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new("recover").description(cmd_desc)]
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
            let _ = config
                .db_pool
                .as_ref()
                .ok_or_else(database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            let mut params = HashMap::new();
            params.insert("user".to_string(), command.user.name.clone());

            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content(
                    "recovery.started",
                    Some(&params),
                    Some(command.user.id),
                    command.guild_id.map(|g| g.get()),
                )
                .await
                .to_channel(command.channel_id)
                .send_interaction_followup(&command, true)
                .await;

            let ctx_clone = ctx.clone();
            let config_clone = config.clone();
            let channel_id = command.channel_id;
            let command_clone = command.clone();

            tokio::spawn(async move {
                let recovery_results = recover_missing_messages(&ctx_clone, &config_clone).await;

                let total_recovered: u32 = recovery_results.iter().map(|r| r.recovered_count).sum();
                let successful_threads = recovery_results.iter().filter(|r| r.success).count();
                let failed_threads = recovery_results.len() - successful_threads;

                let mut params = HashMap::new();
                params.insert("total".to_string(), total_recovered.to_string());
                params.insert("threads".to_string(), successful_threads.to_string());
                params.insert("failed".to_string(), failed_threads.to_string());

                let _ = MessageBuilder::system_message(&ctx_clone, &config_clone)
                    .translated_content("recovery.summary", Some(&params), None, None)
                    .await
                    .to_channel(channel_id)
                    .send_interaction_followup(&command_clone, true)
                    .await;
            });

            Ok(())
        })
    }
}
