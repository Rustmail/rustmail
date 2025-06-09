use config::load_config;
use handlers::{message_handler::MessageHandler, ready_handler::ReadyHandler};
use serenity::{
    all::{ClientBuilder, GatewayIntents},
    prelude::TypeMapKey,
};
use crate::handlers::typing_proxy_handler::TypingProxyHandler;

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
        | GatewayIntents::DIRECT_MESSAGE_TYPING;
    let mut client: serenity::Client = ClientBuilder::new(config.bot.token.clone(), intents)
        .event_handler(ReadyHandler::new(&config))
        .event_handler(MessageHandler::new(&config))
        .event_handler(TypingProxyHandler::new(&config))
        .await
        .expect("Failed to create client.");

    if let Err(e) = client.start().await {
        println!("Failed to initialize client: {e}");
    }
}
