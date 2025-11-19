use crate::bot::{init_bot_state, start_bot_if_config_valid};
use crate::prelude::api::*;
use axum::extract::Path;
use axum::response::Response;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::net::SocketAddr;
use tokio::signal;

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
mod types;
mod utils;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

async fn static_handler(path: Option<Path<String>>) -> Response {
    let path = path.map(|p| p.0).unwrap_or_else(|| "".to_string());

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
            axum::response::Response::builder()
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
                axum::response::Response::builder()
                    .header("Content-Type", "text/html")
                    .body(body)
                    .unwrap()
            } else {
                axum::response::Response::builder()
                    .status(404)
                    .body(axum::body::Body::from("404 Not Found"))
                    .unwrap()
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let bot_state = init_bot_state().await;

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
                    .route("/{*path}", axum::routing::get(static_handler));

                let listener = tokio::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", 3002)).await.unwrap();
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
            loop {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        println!("Shutting down");
                        break;
                    }
                }
            }
        }
    }
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    println!("Shutdown signal received");
}
