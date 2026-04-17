use crate::prelude::errors::*;
use crate::prelude::types::*;
use chrono::Utc;
use serenity::all::{ChannelId, UserId};
use serenity::builder::EditChannel;
use serenity::client::Context;
use std::time::Duration;
use tokio::time::timeout;

/// Updates the Discord channel name to reflect the ticket state.
///
/// Returns `true` if the edit was applied immediately, `false` if it timed out
/// (likely rate-limited) but is still running in the background.
pub async fn update_thread_status_ui(ctx: &Context, ticket: &TicketState) -> ModmailResult<bool> {
    let channel = ChannelId::new(ticket.channel_id as u64);

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

    let owner_id = ticket.owner_id.parse().unwrap_or(0);
    let owner_name = UserId::new(owner_id).to_user(&ctx.http).await?.name;

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

    let name = if let Some(label) = &ticket.label {
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
    };

    let http = ctx.http.clone();
    let channel_id_log = ticket.channel_id;

    // Spawn the edit so it is never cancelled — even if the caller drops us,
    // the request keeps running until Discord replies.
    let handle = tokio::spawn(async move {
        match channel.edit(&http, EditChannel::new().name(&name)).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to edit channel {}: {:?}", channel_id_log, e);
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
            eprintln!(
                "Edit task panicked for channel {}: {:?}",
                ticket.channel_id, e
            );
            Ok(true)
        }
        Err(_) => {
            eprintln!(
                "Timeout editing channel {} — continuing in background",
                ticket.channel_id
            );
            Ok(false)
        }
    }
}
