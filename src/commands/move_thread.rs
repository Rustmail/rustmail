use crate::config::Config;
use crate::db::operations::get_user_id_from_channel_id;
use crate::errors::{
    CommandError, DatabaseError, DiscordError, ModmailError, ModmailResult, ThreadError, common,
};
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, EditChannel, GuildId, Message, ResolvedOption,
};
use std::collections::HashMap;

pub fn register() -> CreateCommand {
    CreateCommand::new("move")
        .description("Move a thread in an other category")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "category",
                "The category where you want to move the thread",
            )
            .required(true),
        )
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

    if !get_user_id_from_channel_id(&command.channel_id.to_string(), pool)
        .await
        .is_some()
    {
        return Err(ModmailError::Command(CommandError::NotInThread()));
    }

    let category_name = match command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "category")
    {
        Some(opt) => match &opt.value {
            serenity::all::CommandDataOptionValue::String(name) => name.trim().to_string(),
            _ => String::new(),
        },
        None => String::new(),
    };

    if category_name.is_empty() {
        return Err(ModmailError::Thread(ThreadError::CategoryNotFound));
    }

    let categories = fetch_server_categories(ctx, config).await;
    if categories.is_empty() {
        return Err(ModmailError::Discord(DiscordError::FailedToFetchCategories));
    }

    let target_category = find_best_match_category(&category_name, &categories);

    match target_category {
        Some((category_id, category_name)) => {
            if let Err(e) =
                move_channel_to_category_by_command_option(ctx, command, category_id).await
            {
                eprintln!("Failed to move channel: {}", e);
                return Err(ModmailError::Discord(DiscordError::FailedToMoveChannel));
            }

            let mut params = HashMap::new();
            params.insert("category".to_string(), category_name.to_string());
            params.insert("staff".to_string(), command.user.name.clone());

            let response = MessageBuilder::system_message(&ctx, &config)
                .translated_content(
                    "move.success",
                    Some(&params),
                    Some(command.user.id),
                    command.guild_id.map(|g| g.get()),
                )
                .await
                .to_channel(command.channel_id)
                .build_interaction_message()
                .await;

            command
                .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                .await?;

            Ok(())
        }
        None => {
            let mut params = HashMap::new();
            params.insert("category".to_string(), category_name);

            Err(ModmailError::Thread(ThreadError::CategoryNotFound))
        }
    }
}

pub async fn move_thread(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    if !is_in_thread(msg, pool).await {
        send_error_message(ctx, msg, config, "move.not_in_thread", None).await;
        return Ok(());
    }

    let category_name = extract_category_name(msg, config).await;
    if category_name.is_empty() {
        send_error_message(ctx, msg, config, "move.missing_category", None).await;
        return Ok(());
    }

    let categories = fetch_server_categories(ctx, config).await;
    if categories.is_empty() {
        send_error_message(ctx, msg, config, "move.failed_to_fetch_categories", None).await;
        return Ok(());
    }

    let target_category = find_best_match_category(&category_name, &categories);

    match target_category {
        Some((category_id, category_name)) => {
            if let Err(e) = move_channel_to_category_by_msg(ctx, msg, category_id).await {
                eprintln!("Failed to move channel: {}", e);
                send_error_message(ctx, msg, config, "move.failed_to_move", None).await;
                return Ok(());
            }

            send_success_message(ctx, msg, config, &category_name).await;
        }
        None => {
            let mut params = HashMap::new();
            params.insert("category".to_string(), category_name);
            send_error_message(ctx, msg, config, "move.category_not_found", Some(&params)).await;
        }
    }

    Ok(())
}

async fn is_in_thread(msg: &Message, pool: &sqlx::SqlitePool) -> bool {
    let channel_id = msg.channel_id.to_string();
    get_user_id_from_channel_id(&channel_id, pool)
        .await
        .is_some()
}

async fn extract_category_name(msg: &Message, config: &Config) -> String {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_names = ["move", "mv"];

    if command_names
        .iter()
        .any(|&name| content.starts_with(&format!("{}{}", prefix, name)))
    {
        let start = prefix.len() + command_names[0].len();
        content[start..].trim().to_string()
    } else {
        String::new()
    }
}

async fn fetch_server_categories(ctx: &Context, config: &Config) -> Vec<(ChannelId, String)> {
    let staff_guild_id = GuildId::new(config.bot.get_staff_guild_id());

    match staff_guild_id.channels(&ctx.http).await {
        Ok(channels) => {
            let mut cats = Vec::new();
            for (id, channel) in channels {
                if channel.kind == serenity::model::channel::ChannelType::Category {
                    cats.push((id, channel.name.clone()));
                }
            }
            cats
        }
        Err(_) => Vec::new(),
    }
}

async fn move_channel_to_category_by_msg(
    ctx: &Context,
    msg: &Message,
    category_id: ChannelId,
) -> Result<serenity::model::channel::GuildChannel, serenity::Error> {
    msg.channel_id
        .edit(&ctx.http, EditChannel::new().category(category_id))
        .await
}

async fn move_channel_to_category_by_command_option(
    ctx: &Context,
    command: &CommandInteraction,
    category_id: ChannelId,
) -> Result<serenity::model::channel::GuildChannel, serenity::Error> {
    command
        .channel_id
        .edit(&ctx.http, EditChannel::new().category(category_id))
        .await
}

async fn send_error_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    error_key: &str,
    params: Option<&HashMap<String, String>>,
) {
    let error_msg = get_translated_message(
        config,
        error_key,
        params,
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(ctx, config)
        .content(error_msg)
        .to_channel(msg.channel_id)
        .send()
        .await;
}

async fn send_success_message(ctx: &Context, msg: &Message, config: &Config, category_name: &str) {
    let mut params = HashMap::new();
    params.insert("category".to_string(), category_name.to_string());
    params.insert("staff".to_string(), msg.author.name.clone());

    let confirmation_msg = get_translated_message(
        config,
        "move.success",
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(ctx, config)
        .content(confirmation_msg)
        .to_channel(msg.channel_id)
        .send()
        .await;

    let _ = msg.delete(&ctx.http).await;
}

fn find_best_match_category(
    target_name: &str,
    categories: &[(ChannelId, String)],
) -> Option<(ChannelId, String)> {
    let target_lower = target_name.to_lowercase();
    let mut best_match = None;
    let mut best_distance = usize::MAX;

    for (id, name) in categories {
        let name_lower = name.to_lowercase();

        if name_lower == target_lower {
            return Some((*id, name.clone()));
        }

        if name_lower.contains(&target_lower) || target_lower.contains(&name_lower) {
            let distance = levenshtein_distance(&target_lower, &name_lower);
            if distance < best_distance {
                best_distance = distance;
                best_match = Some((*id, name.clone()));
            }
        }
    }

    if let Some((id, name)) = best_match {
        let max_distance = (target_name.len().max(name.len()) as f64 * 0.5) as usize;
        if best_distance <= max_distance {
            return Some((id, name));
        }
    }

    None
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}
