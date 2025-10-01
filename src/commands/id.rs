use crate::config::Config;
use crate::db::get_thread_by_channel_id;
use crate::db::operations::threads::is_a_ticket_channel;
use crate::errors::ThreadError::{NotAThreadChannel, ThreadNotFound};
use crate::errors::common::{database_connection_failed, thread_not_found};
use crate::errors::{DatabaseError, ModmailError, ModmailResult, ThreadError};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse, Message, ResolvedOption,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("id").description("Get ID of the user in the thread")
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    _options: &[ResolvedOption<'_>],
    config: &Config,
) -> ModmailResult<()> {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            return Err(ModmailError::Database(DatabaseError::ConnectionFailed));
        }
    };

    if !is_a_ticket_channel(command.channel_id, pool).await {
        return Err(ModmailError::Thread(NotAThreadChannel));
    }

    let thread = match get_thread_by_channel_id(&command.channel_id.to_string(), pool).await {
        Some(thread) => thread,
        None => {
            return Err(ModmailError::Thread(ThreadNotFound));
        }
    };

    let mut params = std::collections::HashMap::new();
    params.insert("user".to_string(), format!("<@{}>", thread.user_id));
    params.insert(
        "id".to_string(),
        format!("||{}||", thread.user_id.to_string()),
    );

    let response = CreateInteractionResponse::Message(
        MessageBuilder::system_message(&ctx, &config)
            .translated_content("id.show_id", Some(&params), None, None)
            .await
            .to_channel(command.channel_id)
            .build_interaction_message()
            .await,
    );
    let _ = command.create_response(&ctx.http, response).await;

    Ok(())
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
