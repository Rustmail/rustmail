use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::utils::*;
use serde::Deserialize;
use serenity::all::{ChannelId, Context};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time::interval;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPOSITORY_URL: &str = "https://github.com/Rustmail/rustmail";

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/Rustmail/rustmail/releases/latest";

const LATEST_KNOWN_VERSION_KEY: &str = "latest_known_version";
const LATEST_RELEASE_URL_KEY: &str = "latest_release_url";
const LAST_UPDATE_CHECK_KEY: &str = "last_update_check";
const ANNOUNCED_UPDATE_VERSION_KEY: &str = "announced_update_version";

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
}

fn parse_version(version: &str) -> Option<(u64, u64, u64)> {
    let version = version.trim();
    let version = version.strip_prefix('v').unwrap_or(version);
    let version = version.split(['-', '+']).next()?;

    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some((major, minor, patch))
}

pub fn is_newer(latest: &str, current: &str) -> bool {
    match (parse_version(latest), parse_version(current)) {
        (Some(latest), Some(current)) => latest > current,
        _ => false,
    }
}

async fn fetch_latest_release() -> Option<GithubRelease> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| eprintln!("Update check: failed to build HTTP client: {}", e))
        .ok()?;

    let response = client
        .get(LATEST_RELEASE_URL)
        .header("User-Agent", format!("rustmail/{}", CURRENT_VERSION))
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| eprintln!("Update check: request to GitHub failed: {}", e))
        .ok()?;

    if !response.status().is_success() {
        eprintln!(
            "Update check: GitHub returned status {}",
            response.status().as_u16()
        );
        return None;
    }

    response
        .json::<GithubRelease>()
        .await
        .map_err(|e| eprintln!("Update check: invalid response from GitHub: {}", e))
        .ok()
}

async fn announce_update(ctx: &Context, config: &Config, latest: &str, release_url: &str) {
    let channel_id = config
        .updates
        .notify_channel_id
        .or(config.bot.logs_channel_id);

    let Some(channel_id) = channel_id else {
        println!(
            "A new Rustmail version is available: {} (current: {}) - {}",
            latest, CURRENT_VERSION, release_url
        );
        return;
    };

    let mut params = HashMap::new();
    params.insert("current".to_string(), CURRENT_VERSION.to_string());
    params.insert("latest".to_string(), latest.to_string());
    params.insert("url".to_string(), release_url.to_string());

    if let Err(e) = MessageBuilder::system_message(ctx, config)
        .translated_content("update.available", Some(&params), None, None)
        .await
        .to_channel(ChannelId::new(channel_id))
        .send(false)
        .await
    {
        eprintln!("Update check: failed to announce new version: {}", e);
    }
}

async fn check_once(ctx: &Context, config: &Config) {
    let Some(pool) = config.db_pool.as_ref() else {
        eprintln!("Update check: database pool is not set, skipping.");
        return;
    };

    let Some(release) = fetch_latest_release().await else {
        return;
    };

    let _ = set_system_metadata(LATEST_KNOWN_VERSION_KEY, &release.tag_name, pool).await;
    let _ = set_system_metadata(LATEST_RELEASE_URL_KEY, &release.html_url, pool).await;
    let _ = set_system_metadata(
        LAST_UPDATE_CHECK_KEY,
        &chrono::Utc::now().to_rfc3339(),
        pool,
    )
    .await;

    if !is_newer(&release.tag_name, CURRENT_VERSION) {
        return;
    }

    let already_announced = get_system_metadata(ANNOUNCED_UPDATE_VERSION_KEY, pool)
        .await
        .ok()
        .flatten();

    if already_announced.as_deref() == Some(release.tag_name.as_str()) {
        return;
    }

    announce_update(ctx, config, &release.tag_name, &release.html_url).await;

    let _ = set_system_metadata(ANNOUNCED_UPDATE_VERSION_KEY, &release.tag_name, pool).await;
}

pub async fn run_update_checker(ctx: Context, config: Config, mut shutdown: Receiver<bool>) {
    let period = Duration::from_secs(config.updates.check_interval_hours.max(1) * 3600);
    let mut ticker = interval(period);

    loop {
        tokio::select! {
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    break;
                }
            }
            _ = ticker.tick() => {
                check_once(&ctx, &config).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("1.1.0"), Some((1, 1, 0)));
        assert_eq!(parse_version("v1.1.0"), Some((1, 1, 0)));
        assert_eq!(parse_version("v1.2"), Some((1, 2, 0)));
        assert_eq!(parse_version("1.2.3-rc.1"), Some((1, 2, 3)));
        assert_eq!(parse_version("nightly"), None);
        assert_eq!(parse_version("1.2.3.4"), None);
    }

    #[test]
    fn test_is_newer() {
        assert!(is_newer("v1.2.0", "1.1.0"));
        assert!(is_newer("1.10.0", "1.9.0"));
        assert!(is_newer("2.0.0", "1.99.99"));
        assert!(!is_newer("v1.1.0", "1.1.0"));
        assert!(!is_newer("1.0.32", "1.1.0"));
        assert!(!is_newer("not-a-version", "1.1.0"));
    }
}
