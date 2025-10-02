use crate::config::Config;
use crate::db::get_thread_by_channel_id;
use crate::db::threads::is_a_ticket_channel;
use crate::errors::ThreadError::{NotAThreadChannel, ThreadNotFound};
use crate::errors::{DatabaseError, ModmailError, ModmailResult};
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{CommandInteraction, Context, ResolvedOption};
use serenity::builder::{CreateCommand, CreateInteractionResponse};

pub async fn register(config: &Config) -> CreateCommand {
    let cmd_desc = get_translated_message(
        config,
        "slash_command.id_command_description",
        None,
        None,
        None,
        None,
    )
    .await;

    CreateCommand::new("id").description(cmd_desc)
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
