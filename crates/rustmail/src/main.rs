use crate::bot::{init_bot_state, start_bot_if_config_valid};
use crate::config::{resolve_bind_address, resolve_config_path, resolve_port};
use crate::prelude::api::*;
use crate::setup::router::create_setup_router;
use crate::setup::state::new_setup_state;
use axum::extract::Path;
use axum::response::Response;
#[cfg(test)]
#[allow(unused_imports)]
use cargo_husky as _;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::{env, process};
use tokio::signal;
use tower_http::compression::CompressionLayer;

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod api;
mod bot;
mod commands;
mod config;
mod db;
mod errors;
mod features;
mod handlers;
mod i18n;
mod modules;
mod panel_commands;
mod prelude;
mod setup;
mod types;
mod utils;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

async fn static_handler(path: Option<Path<String>>) -> Response {
    let path = path.map(|p| p.0).unwrap_or_default();

    let path = if path.is_empty() || path == "/" {
        "index.html".to_string()
    } else {
        path.trim_start_matches('/').to_string()
    };

    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            let body = match content.data {
                Cow::Borrowed(bytes) => axum::body::Body::from(bytes.to_vec()),
                Cow::Owned(bytes) => axum::body::Body::from(bytes),
            };
            Response::builder()
                .header("Content-Type", mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => {
            if let Some(index) = Assets::get("index.html") {
                let body = match index.data {
                    Cow::Borrowed(bytes) => axum::body::Body::from(bytes.to_vec()),
                    Cow::Owned(bytes) => axum::body::Body::from(bytes),
                };
                Response::builder()
                    .header("Content-Type", "text/html")
                    .body(body)
                    .unwrap()
            } else {
                Response::builder()
                    .status(404)
                    .body(axum::body::Body::from("404 Not Found"))
                    .unwrap()
            }
        }
    }
}

fn print_version() {
    println!("rustmail {}", VERSION);
}

fn print_help() {
    println!("rustmail {} - Discord modmail bot", VERSION);
    println!();
    println!("USAGE:");
    println!("    rustmail [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print this help message");
    println!("    -v, --version    Print version information");
    println!();
    println!("CONFIGURATION:");
    println!("    Rustmail requires a config.toml file in the current directory.");
    println!("    If no configuration is found, a setup wizard will launch on port 3002.");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    RUSTMAIL_CONFIG_PATH      Path to config.toml (default: config.toml)");
    println!("    RUSTMAIL_BOT_TOKEN        Overrides bot.token from config.toml");
    println!("    RUSTMAIL_BOT_CLIENT_ID    Overrides bot.client_id from config.toml");
    println!("    RUSTMAIL_BOT_CLIENT_SECRET Overrides bot.client_secret from config.toml");
    println!("    RUSTMAIL_DATABASE_URL     Database path (default: db/db.sqlite)");
    println!("    RUSTMAIL_BIND_ADDRESS     Bind address (default: 0.0.0.0)");
    println!("    RUSTMAIL_PORT             Port (default: 3002)");
    println!();
    println!("DOCUMENTATION:");
    println!("    https://docs.rustmail.rs");
    println!();
    println!("SOURCE CODE:");
    println!("    https://github.com/Rustmail/rustmail");
}

async fn run_setup_mode() {
    let bind_address = resolve_bind_address("0.0.0.0");
    let port = resolve_port(3002);

    let setup_state = new_setup_state();

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
    let setup_token = {
        let mut state = setup_state.lock().await;
        state.shutdown_tx = Some(shutdown_tx);
        state.token.clone()
    };

    let app = create_setup_router(setup_state)
        .route("/", axum::routing::get(static_handler))
        .route("/{*path}", axum::routing::get(static_handler))
        .layer(CompressionLayer::new());

    let addr = format!("{}:{}", bind_address, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {}", addr));

    println!("No configuration found.");
    println!(
        "Setup wizard available at http://{}:{}/setup?token={}",
        bind_address, port, setup_token
    );
    println!("Open this URL in your browser to configure Rustmail.");
    println!("This link contains a one-time secret: do not share it.");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                println!("Setup complete, transitioning to bot mode...");
            }
            _ = shutdown_signal() => {
                println!("Shutdown signal received during setup");
                process::exit(0);
            }
        }
    })
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "-v" | "--version" => {
                print_version();
                process::exit(0);
            }
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown option: {}", arg);
                eprintln!("Run 'rustmail --help' for usage information.");
                process::exit(1);
            }
        }
    }

    let config_path = resolve_config_path("config.toml");
    let mut bot_state = init_bot_state(&config_path).await;

    let has_config = {
        let state = bot_state.lock().await;
        state.config.is_some()
    };

    if !has_config {
        run_setup_mode().await;
        bot_state = init_bot_state(&config_path).await;
    }

    let _ = start_bot_if_config_valid(bot_state.clone()).await;

    let config = {
        let state = bot_state.lock().await;
        state.config.clone()
    };

    if let Some(config) = config {
        if config.bot.enable_panel {
            let bot_state_clone = bot_state.clone();

            let server_task = tokio::spawn(async move {
                let app = create_api_router(bot_state_clone)
                    .route("/", axum::routing::get(static_handler))
                    .route("/{*path}", axum::routing::get(static_handler))
                    .layer(CompressionLayer::new());

                let bind_address = resolve_bind_address("0.0.0.0");
                let port = resolve_port(config.bot.panel_port);

                let listener =
                    match tokio::net::TcpListener::bind(format!("{}:{}", bind_address, port)).await
                    {
                        Ok(l) => l,
                        Err(e) => {
                            eprintln!(
                                "Failed to bind to {}:{} ({}), falling back to 0.0.0.0:3002",
                                bind_address, port, e
                            );
                            tokio::net::TcpListener::bind("0.0.0.0:3002")
                                .await
                                .expect("Failed to bind to 0.0.0.0:3002")
                        }
                    };
                println!("listening on {}", listener.local_addr().unwrap());

                axum::serve(
                    listener,
                    app.into_make_service_with_connect_info::<SocketAddr>(),
                )
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
            });

            tokio::select! {
                _ = server_task => {},
                _ = tokio::signal::ctrl_c() => { println!("Shutting down"); }
            }
        } else {
            let _ = tokio::signal::ctrl_c().await;
            println!("Shutting down");
        }
    }
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    println!("Shutdown signal received");
}
