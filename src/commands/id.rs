use crate::config::Config;
use crate::db::get_thread_by_channel_id;
use crate::db::threads::is_a_ticket_channel;
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::common::{database_connection_failed, thread_not_found};
use crate::errors::{ModmailError, ModmailResult};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, Message, ResolvedOption, ResolvedValue};
use crate::i18n::get_translated_message;

pub fn register() -> CreateCommand {
    CreateCommand::new("id").description("Get ID of the user in the thread").add_option(
        CreateCommandOption::new(CommandOptionType::User, "id", "The ID of the user")
            .required(true),
    )
}

pub async fn run(command: CommandInteraction, options: &[ResolvedOption<'_>], config: &Config) -> String {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return None
        }
    };

    if is_a_ticket_channel(command.channel_id, &pool).await {
        let thread = match get_thread_by_channel_id(&command.channel_id.to_string(), pool).await {
            Some(thread) => thread,
            None => return,
        };

        if let Some(ResolvedOption {
                    value: ResolvedValue::User(user, _), ..
                }) = options.first()
        {
            let mut params = std::collections::HashMap::new();
            params.insert("user".to_string(), format!("<@{}>", thread.user_id));
            params.insert(
                "id".to_string(),
                format!("||{}||", thread.user_id.to_string()),
            );

            get_translated_message(&config, "id.show_id", Some(&params), None, None, None).await
        }
    }
}

pub async fn id(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if is_a_ticket_channel(msg.channel_id, &db_pool).await {
        let thread = match get_thread_by_channel_id(&msg.channel_id.to_string(), db_pool).await {
            Some(thread) => thread,
            None => return Err(thread_not_found()),
        };

        let mut params = std::collections::HashMap::new();
        params.insert("user".to_string(), format!("<@{}>", thread.user_id));
        params.insert(
            "id".to_string(),
            format!("||{}||", thread.user_id.to_string()),
        );

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("id.show_id", Some(&params), None, None)
            .await
            .to_channel(msg.channel_id)
            .send()
            .await?;

        Ok(())
    } else {
        Err(ModmailError::Thread(NotAThreadChannel))
    }
}
