use crate::modules::update_thread_status_ui;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::sync::Arc;

pub struct RenameCommand;

#[async_trait::async_trait]
impl RegistrableCommand for RenameCommand {
    fn name(&self) -> &'static str {
        "rename"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.rename", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.rename_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            let label_desc = get_translated_message(
                &config,
                "slash_command.rename_label_option",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(self.name())
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::String, "label", label_desc)
                            .required(false),
                    ),
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

            if !is_a_ticket_channel(command.channel_id, &db_pool).await {
                return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
            }

            let thread =
                match get_thread_by_channel_id(&command.channel_id.to_string(), db_pool).await {
                    Some(thread) => thread,
                    None => return Err(thread_not_found()),
                };

            let mut new_label: Option<String> = None;
            for option in &command.data.options {
                if option.name.as_str() == "label" {
                    if let CommandDataOptionValue::String(val) = &option.value {
                        if !val.trim().is_empty() {
                            new_label = Some(val.trim().to_string());
                        }
                    }
                }
            }

            tokio::spawn({
                let db_pool = db_pool.clone();
                async move {
                    let mut ticket_status = match get_thread_status(&thread.id, &db_pool).await {
                        Some(status) => status,
                        None => return,
                    };

                    ticket_status.label = new_label.clone();
                    let _ = update_thread_status_db(&thread.id, &ticket_status, &db_pool).await;

                    let has_label = new_label.is_some();
                    let applied = update_thread_status_ui(&ctx, &ticket_status)
                        .await
                        .unwrap_or(true);

                    let key = match (has_label, applied) {
                        (true, true) => "rename.confirmation",
                        (true, false) => "rename.confirmation_rate_limited",
                        (false, true) => "rename.cleared",
                        (false, false) => "rename.cleared_rate_limited",
                    };

                    let params = if let Some(label) = new_label {
                        let mut p = std::collections::HashMap::new();
                        p.insert("label".to_string(), label);
                        Some(p)
                    } else {
                        None
                    };

                    let _ = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(key, params.as_ref(), None, None)
                        .await
                        .to_channel(command.channel_id)
                        .send_interaction_followup(&command, true)
                        .await;
                }
            });

            Ok(())
        })
    }
}
