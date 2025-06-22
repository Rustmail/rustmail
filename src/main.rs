use config::load_config;
use handlers::{message_handler::MessageHandler, ready_handler::ReadyHandler, member_handler::MemberHandler, reaction_handler::ReactionHandler};
use serenity::{
    all::{ClientBuilder, GatewayIntents},
    prelude::TypeMapKey,
};
use crate::handlers::typing_proxy_handler::TypingProxyHandler;
use std::process;

mod commands;
mod config;
mod db;
mod errors;
mod events;
mod handlers;
mod i18n;
mod modules;
mod utils;

pub struct Database;

impl TypeMapKey for Database {
    type Value = sqlx::SqlitePool;
}

#[tokio::main]
async fn main() {
    let pool = db::operations::init_database().await.expect("An error occured!");
    println!("Database connected!");

    let mut config = load_config("config.toml");
    config.db_pool = Some(pool.clone());
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_TYPING
        | GatewayIntents::DIRECT_MESSAGE_TYPING
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS;
    let mut client: serenity::Client = ClientBuilder::new(config.bot.token.clone(), intents)
        .event_handler(ReadyHandler::new(&config))
        .event_handler(MessageHandler::new(&config))
        .event_handler(TypingProxyHandler::new(&config))
        .event_handler(MemberHandler::new(&config))
        .event_handler(ReactionHandler::new(&config))
        .await
        .expect("Failed to create client.");

    if let Err(e) = config.validate_servers(&client.http).await {
        eprintln!("Erreur de validation de configuration: {}", e);
        eprintln!("Vérifiez que les IDs de serveur sont corrects et que le bot a accès aux serveurs.");
        process::exit(1);
    }

    println!("Configuration validée avec succès!");
    if config.bot.is_dual_mode() {
        println!("Mode: Double serveur (Communautaire: {}, Staff: {})", 
                 config.bot.get_community_guild_id(), 
                 config.bot.get_staff_guild_id());
    } else {
        println!("Mode: Serveur unique (ID: {})", config.bot.get_community_guild_id());
    }

    if let Err(e) = client.start().await {
        println!("Failed to initialize client: {e}");
    }
}
