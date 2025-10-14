use crate::config::Config;
use crate::db::get_user_id_from_channel_id;
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{ChannelId, CommandInteraction, Context, EditChannel, GuildId, Message};
use std::collections::HashMap;

pub async fn is_in_thread(msg: &Message, pool: &sqlx::SqlitePool) -> bool {
    let channel_id = msg.channel_id.to_string();
    get_user_id_from_channel_id(&channel_id, pool)
        .await
        .is_some()
}

pub async fn extract_category_name(msg: &Message, config: &Config) -> String {
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

pub async fn fetch_server_categories(ctx: &Context, config: &Config) -> Vec<(ChannelId, String)> {
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

pub async fn move_channel_to_category_by_msg(
    ctx: &Context,
    msg: &Message,
    category_id: ChannelId,
) -> Result<serenity::model::channel::GuildChannel, serenity::Error> {
    msg.channel_id
        .edit(&ctx.http, EditChannel::new().category(category_id))
        .await
}

pub async fn move_channel_to_category_by_command_option(
    ctx: &Context,
    command: &CommandInteraction,
    category_id: ChannelId,
) -> Result<serenity::model::channel::GuildChannel, serenity::Error> {
    command
        .channel_id
        .edit(&ctx.http, EditChannel::new().category(category_id))
        .await
}

pub async fn send_error_message(
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

pub async fn send_success_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    category_name: &str,
) {
    let mut params = HashMap::new();
    params.insert("category".to_string(), category_name.to_string());
    params.insert("staff".to_string(), msg.author.name.clone());

    let confirmation_msg = get_translated_message(
        config,
        "move_thread.success",
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

pub fn find_best_match_category(
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

pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
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
