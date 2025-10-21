use crate::commands::logs::common::render_logs_page;
use crate::config::Config;
use crate::features::make_buttons;
use crate::i18n::get_translated_message;
use crate::types::logs::PaginationStore;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{ButtonStyle, ComponentInteraction, Context};
use serenity::builder::CreateInteractionResponse;

pub const LOGS_PAGE_SIZE: usize = 10;

fn parse_command_interaction(custom_id: &str) -> Option<String> {
    custom_id.strip_prefix("command:").map(|s| s.to_string())
}

async fn handle_logs_action(
    session_id: &str,
    page: usize,
    ctx: &Context,
    config: &Config,
    pagination: PaginationStore,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut store = pagination.lock().await;

    let next_button =
        get_translated_message(&config, "logs_command.next", None, None, None, None).await;
    let prev_button =
        get_translated_message(&config, "logs_command.prev", None, None, None, None).await;

    if let Some(ctx_data) = store.get_mut(session_id) {
        if page == 0 && ctx_data.current_page > 0 {
            ctx_data.current_page -= 1;
        } else if page == 1 && ctx_data.current_page < LOGS_PAGE_SIZE {
            ctx_data.current_page += 1;
        } else {
            return Ok(());
        }

        let new_content = render_logs_page(&ctx_data.logs, ctx_data.current_page, LOGS_PAGE_SIZE);

        if new_content.is_empty() {
            return Ok(());
        }

        let components = make_buttons(&[
            (
                &prev_button.to_string(),
                &format!("command:logs_prev:{}", session_id),
                ButtonStyle::Primary,
                ctx_data.current_page == 0,
            ),
            (
                &next_button.to_string(),
                &format!("command:logs_next:{}", session_id),
                ButtonStyle::Primary,
                (ctx_data.current_page + 1) * 10 >= ctx_data.logs.len(),
            ),
        ]);

        let response = MessageBuilder::system_message(&ctx, &config)
            .content(new_content)
            .components(components)
            .to_channel(ctx_data.channel_id)
            .build_edit_message()
            .await;

        ctx_data
            .channel_id
            .edit_message(&ctx.http, ctx_data.message_id, response)
            .await?;
    }

    Ok(())
}

pub async fn handle_command_component_interaction(
    ctx: &Context,
    config: &Config,
    interaction: &mut ComponentInteraction,
    pagination: PaginationStore,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let parts = match parse_command_interaction(&interaction.data.custom_id) {
        Some(parts) => parts,
        None => return Ok(()),
    };

    if parts.starts_with("logs_next:") {
        let session_id = parts.strip_prefix("logs_next:").unwrap();

        interaction
            .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
            .await?;

        handle_logs_action(session_id, 1, ctx, config, pagination.clone()).await?;
    }

    if parts.starts_with("logs_prev:") {
        let session_id = parts.strip_prefix("logs_prev:").unwrap();

        interaction
            .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
            .await?;

        handle_logs_action(session_id, 0, ctx, config, pagination.clone()).await?;
    }

    Ok(())
}
