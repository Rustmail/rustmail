use crate::handlers::guild_handler::GuildHandler;
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::handlers::guild_message_reactions_handler::GuildMessageReactionsHandler;
use crate::handlers::guild_moderation_handler::GuildModerationHandler;
use crate::handlers::typing_proxy_handler::TypingProxyHandler;
use config::load_config;
use handlers::{
    guild_members_handler::GuildMembersHandler, guild_messages_handler::GuildMessagesHandler,
    ready_handler::ReadyHandler,
};
use serenity::all::{ClientBuilder, GatewayIntents};
use serenity::cache::Settings as CacheSettings;
use std::process;
use std::time::Duration;

mod commands;
mod config;
mod db;
mod errors;
mod features;
mod handlers;
mod i18n;
mod modules;
mod utils;

pub struct Database;

#[tokio::main]
async fn main() {
    let pool = db::operations::init_database()
        .await
        .expect("An error occured!");
    println!("Database connected!");

    let mut config = load_config("config.toml");
    config.db_pool = Some(pool.clone());

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_TYPING
        | GatewayIntents::DIRECT_MESSAGE_TYPING
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MODERATION;

    let mut cache_settings = CacheSettings::default();
    cache_settings.max_messages = 10_000;
    cache_settings.time_to_live = Duration::from_secs(6 * 60 * 60);

    let mut client: serenity::Client = ClientBuilder::new(config.bot.token.clone(), intents)
        .cache_settings(cache_settings)
        .event_handler(ReadyHandler::new(&config))
        .event_handler(GuildMessagesHandler::new(&config))
        .event_handler(TypingProxyHandler::new(&config))
        .event_handler(GuildMembersHandler::new(&config))
        .event_handler(GuildMessageReactionsHandler::new(&config))
        .event_handler(GuildModerationHandler::new(&config))
        .event_handler(InteractionHandler::new(&config))
        .event_handler(GuildHandler::new(&config))
        .await
        .expect("Failed to create client.");

    if let Err(e) = config.validate_servers(&client.http).await {
        eprintln!("Erreur de validation de configuration: {}", e);
        eprintln!(
            "Vérifiez que les IDs de serveur sont corrects et que le bot a accès aux serveurs."
        );
        process::exit(1);
    }

    println!("Configuration validée avec succès!");
    if config.bot.is_dual_mode() {
        println!(
            "Mode: Double serveur (Communautaire: {}, Staff: {})",
            config.bot.get_community_guild_id(),
            config.bot.get_staff_guild_id()
        );
    } else {
        println!(
            "Mode: Serveur unique (ID: {})",
            config.bot.get_community_guild_id()
        );
    }

    if let Err(e) = client.start().await {
        println!("Failed to initialize client: {e}");
    }
}
