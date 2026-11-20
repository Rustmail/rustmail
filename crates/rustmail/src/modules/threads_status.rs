use crate::prelude::errors::*;
use crate::prelude::types::*;
use chrono::Utc;
use serenity::all::{ChannelId, UserId};
use serenity::builder::EditChannel;
use serenity::client::Context;
use std::collections::{HashMap, VecDeque};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Discord allows 2 channel renames per 10 minutes per channel. Anything beyond that is
/// held in serenity's ratelimiter, which also delays every other request on the same
/// channel (fetching it, editing it...). We never send a rename we know will be limited.
const RENAME_WINDOW: Duration = Duration::from_secs(10 * 60 + 15);
const RENAMES_PER_WINDOW: usize = 2;

/// Interval of the periodic status refresh: one rename window, so each refresh finds the
/// slot it used last time free again.
pub const STATUS_REFRESH_INTERVAL: Duration = RENAME_WINDOW;

#[derive(Default)]
struct RenameState {
    recent: VecDeque<Instant>,
    in_flight: bool,
    last_name: Option<String>,
}

static RENAME_STATES: LazyLock<Mutex<HashMap<u64, RenameState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Reserves a rename slot for `channel_id` if fewer than `max_recent` renames happened in the
/// current window and no rename is already pending.
fn try_reserve_rename(channel_id: u64, max_recent: usize) -> bool {
    let mut states = RENAME_STATES.lock().expect("rename state lock poisoned");
    let now = Instant::now();

    if states.len() > 1024 {
        states.retain(|_, s| {
            s.in_flight
                || s.recent
                    .back()
                    .is_some_and(|t| now.duration_since(*t) < RENAME_WINDOW)
        });
    }

    let state = states.entry(channel_id).or_default();
    while state
        .recent
        .front()
        .is_some_and(|t| now.duration_since(*t) >= RENAME_WINDOW)
    {
        state.recent.pop_front();
    }

    if state.in_flight || state.recent.len() >= max_recent {
        return false;
    }

    state.in_flight = true;
    state.recent.push_back(now);
    true
}

/// Ends a pending rename; `applied_name` is the name Discord accepted, if any.
fn release_rename(channel_id: u64, applied_name: Option<String>) {
    let mut states = RENAME_STATES.lock().expect("rename state lock poisoned");
    if let Some(state) = states.get_mut(&channel_id) {
        state.in_flight = false;
        if applied_name.is_some() {
            state.last_name = applied_name;
        }
    }
}

fn is_current_name(channel_id: u64, name: &str) -> bool {
    let states = RENAME_STATES.lock().expect("rename state lock poisoned");
    states
        .get(&channel_id)
        .and_then(|s| s.last_name.as_deref())
        .is_some_and(|n| n == name)
}

fn build_channel_name(ctx: &Context, ticket: &TicketState, owner_name: &str) -> String {
    let color = match ticket.last_message_by {
        TicketAuthor::Staff => "🔵",
        TicketAuthor::User => "🔴",
    };

    let elapsed = Utc::now().timestamp() - ticket.last_message_at;
    let minutes = elapsed / 60;
    let time_str = if minutes < 60 {
        format!("{}m", minutes)
    } else {
        format!("{}h", minutes / 60)
    };

    let mut staff_name_part = String::new();
    if let Some(staff_id) = &ticket.taken_by {
        let staff_id_u64 = staff_id.parse().unwrap_or(0);
        if staff_id_u64 != 0 {
            let sname = ctx
                .cache
                .user(UserId::new(staff_id_u64))
                .map(|u| u.name.clone())
                .unwrap_or_else(|| format!("Staff-{}", staff_id_u64));
            staff_name_part = format!("・{}", sname);
        }
    }

    if let Some(label) = &ticket.label {
        // Fixed parts: color(1 emoji) + "・" + "・" + owner + staff + "・" + time
        // We truncate the label so the total stays <= 100 chars.
        let without_label = format!("{color}・・{owner_name}{staff_name_part}・{time_str}");
        let budget = 100usize.saturating_sub(without_label.chars().count());
        let truncated: String = label.chars().take(budget).collect();
        if truncated.is_empty() {
            format!("{color}・{owner_name}{staff_name_part}・{time_str}")
        } else {
            format!("{color}・{truncated}・{owner_name}{staff_name_part}・{time_str}")
        }
    } else {
        format!("{color}・{owner_name}{staff_name_part}・{time_str}")
    }
}

/// Updates the Discord channel name to reflect the ticket state, after a staff action.
///
/// Returns `true` if the channel shows the new name, `false` if the rename was deferred
/// (rate limited); the periodic status refresh will apply it later.
pub async fn update_thread_status_ui(ctx: &Context, ticket: &TicketState) -> ModmailResult<bool> {
    apply_thread_status_ui(ctx, ticket, RENAMES_PER_WINDOW).await
}

/// Periodic variant: only uses one rename slot per window so staff actions keep one.
pub async fn refresh_thread_status_ui(ctx: &Context, ticket: &TicketState) -> ModmailResult<bool> {
    apply_thread_status_ui(ctx, ticket, RENAMES_PER_WINDOW - 1).await
}

async fn apply_thread_status_ui(
    ctx: &Context,
    ticket: &TicketState,
    max_recent: usize,
) -> ModmailResult<bool> {
    let channel_id_u64 = ticket.channel_id as u64;
    if channel_id_u64 == 0 {
        return Ok(true);
    }
    let channel = ChannelId::new(channel_id_u64);

    let owner_id: u64 = ticket.owner_id.parse().unwrap_or(0);
    let owner_name = if owner_id == 0 {
        "unknown".to_string()
    } else {
        // Passing `ctx` (not `&ctx.http`) lets serenity answer from its cache first.
        // GET /users/{id} shares a single ratelimit bucket for the whole bot.
        UserId::new(owner_id).to_user(ctx).await?.name
    };

    let name = build_channel_name(ctx, ticket, &owner_name);

    if is_current_name(channel_id_u64, &name) {
        return Ok(true);
    }

    if !try_reserve_rename(channel_id_u64, max_recent) {
        return Ok(false);
    }

    let http = ctx.http.clone();

    // Spawn the edit so it is never cancelled — even if the caller drops us,
    // the request keeps running until Discord replies. The in-flight flag keeps
    // a second edit from queuing behind it.
    let handle = tokio::spawn(async move {
        let result = channel.edit(&http, EditChannel::new().name(&name)).await;
        release_rename(channel_id_u64, result.is_ok().then_some(name));
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to edit channel {}: {:?}", channel_id_u64, e);
                Err(e)
            }
        }
    });

    // Wait up to 2s so callers that care about ordering can rely on it,
    // but if it takes longer the spawn keeps going in the background.
    match timeout(Duration::from_secs(2), handle).await {
        Ok(Ok(Ok(_))) => Ok(true),
        Ok(Ok(Err(e))) => Err(e.into()),
        Ok(Err(e)) => {
            release_rename(channel_id_u64, None);
            eprintln!("Edit task panicked for channel {}: {:?}", channel_id_u64, e);
            Ok(true)
        }
        Err(_) => {
            eprintln!(
                "Timeout editing channel {} — continuing in background",
                channel_id_u64
            );
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_budget_is_enforced_per_channel() {
        let channel = 9_000_000_000_000_001;

        assert!(try_reserve_rename(channel, 2));
        // A rename is still pending: nothing else may queue behind it.
        assert!(!try_reserve_rename(channel, 2));

        release_rename(channel, None);
        assert!(try_reserve_rename(channel, 2));
        release_rename(channel, None);

        // Two renames used in the window.
        assert!(!try_reserve_rename(channel, 2));
        // Other channels are unaffected.
        assert!(try_reserve_rename(channel + 1, 2));
        release_rename(channel + 1, None);
    }

    #[test]
    fn applied_name_is_remembered() {
        let channel = 9_000_000_000_000_201;

        assert!(!is_current_name(channel, "🔴・user・0m"));
        assert!(try_reserve_rename(channel, 2));
        release_rename(channel, Some("🔴・user・0m".to_string()));
        assert!(is_current_name(channel, "🔴・user・0m"));
        assert!(!is_current_name(channel, "🔵・user・0m"));
    }

    #[test]
    fn periodic_refresh_leaves_a_slot_for_staff_actions() {
        let channel = 9_000_000_000_000_101;

        assert!(try_reserve_rename(channel, RENAMES_PER_WINDOW - 1));
        release_rename(channel, None);
        assert!(!try_reserve_rename(channel, RENAMES_PER_WINDOW - 1));
        assert!(try_reserve_rename(channel, RENAMES_PER_WINDOW));
        release_rename(channel, None);
    }
}
