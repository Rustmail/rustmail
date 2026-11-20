pub mod args;
pub mod page;

pub use args::*;
pub use page::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use crate::prelude::utils::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, UserId};
use uuid::Uuid;

pub struct LogsCommand {
    pub pagination: PaginationStore,
}

impl Command for LogsCommand {
    type Args = LogsArgs;

    fn name(&self) -> &'static str {
        "logs"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.logs", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("logs")
                .description(translate("slash_commands.logs_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "id",
                        translate("slash_commands.logs_id_argument_description").await,
                    )
                    .required(false),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<LogsArgs> {
        parse_logs_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<LogsArgs> {
        parse_logs_slash(options)
    }

    async fn run(
        &self,
        args: LogsArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;

        let user_id = match args.user {
            Some(user_id) => user_id,
            None => UserId::new(current_ticket(invocation, pool).await?.user_id as u64),
        };
        let logs = get_logs_from_user_id(&user_id.to_string(), pool).await?;

        let page = 0;
        let session_id = Uuid::new_v4().to_string();
        let content = render_logs_page(config, &logs, page, LOGS_PAGE_SIZE).await;
        let buttons = logs_page_buttons(config, &session_id, page, logs.len()).await;

        let reply = invocation
            .reply(
                MessageBuilder::system_message(invocation.ctx(), config)
                    .content(content)
                    .components(buttons),
            )
            .await?;

        self.pagination.lock().await.insert(
            session_id,
            PaginationContext {
                logs,
                current_page: page,
                message_id: reply.id,
                channel_id: reply.channel_id,
            },
        );
        Ok(())
    }
}
