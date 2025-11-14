use crate::prelude::types::*;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
pub struct ThreadMessage {
    pub id: i64,
    pub thread_id: String,
    pub user_id: i64,
    pub user_name: String,
    pub is_anonymous: bool,
    pub dm_message_id: Option<String>,
    pub inbox_message_id: Option<String>,
    pub message_number: Option<i64>,
    pub created_at: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompleteThread {
    pub id: String,
    pub user_id: i64,
    pub user_name: String,
    pub channel_id: String,
    pub created_at: i64,
    pub new_message_number: i64,
    pub status: i64,
    pub user_left: bool,
    pub closed_at: Option<i64>,
    pub closed_by: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub required_permissions: Option<String>,
    pub messages: Vec<ThreadMessage>,
}

#[derive(Debug, Deserialize)]
pub struct TicketQuery {
    pub id: Option<String>,
}

pub async fn handle_tickets_bot(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Query(params): Query<TicketQuery>,
) -> (StatusCode, Json<serde_json::Value>) {
    let db_pool = {
        let state_lock = bot_state.lock().await;
        match &state_lock.db_pool {
            Some(pool) => pool.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "Database pool not initialized"
                    })),
                );
            }
        }
    };

    if let Some(id) = params.id {
        let thread = match sqlx::query!(
            r#"
            SELECT
                id,
                user_id,
                user_name,
                channel_id,
                strftime('%s', created_at) as "created_at: Option<String>",
                next_message_number as new_message_number,
                status,
                user_left,
                strftime('%s', closed_at) as "closed_at: Option<String>",
                closed_by,
                category_id,
                category_name,
                required_permissions
            FROM threads
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&db_pool)
        .await
        {
            Ok(Some(row)) => row,
            Ok(None) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": "Thread not found"
                    })),
                );
            }
            Err(err) => {
                eprintln!("Erreur SQL thread {}: {:?}", id, err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "Failed to fetch thread"
                    })),
                );
            }
        };

        let messages_query = match sqlx::query!(
            r#"
            SELECT
                id,
                thread_id,
                user_id,
                user_name,
                is_anonymous,
                dm_message_id,
                inbox_message_id,
                message_number,
                created_at as "created_at: String",
                content
            FROM thread_messages
            WHERE thread_id = ?
            ORDER BY created_at ASC
            "#,
            thread.id
        )
        .fetch_all(&db_pool)
        .await
        {
            Ok(rows) => rows,
            Err(err) => {
                eprintln!("Erreur SQL messages pour {}: {:?}", thread.id, err);
                Vec::new()
            }
        };

        let messages: Vec<ThreadMessage> = messages_query
            .into_iter()
            .map(|m| ThreadMessage {
                id: m.id,
                thread_id: m.thread_id,
                user_id: m.user_id,
                user_name: m.user_name,
                is_anonymous: m.is_anonymous,
                dm_message_id: m.dm_message_id,
                inbox_message_id: m.inbox_message_id,
                message_number: m.message_number,
                created_at: m.created_at,
                content: m.content,
            })
            .collect();

        let complete = CompleteThread {
            id: thread.id,
            user_id: thread.user_id,
            user_name: thread.user_name,
            channel_id: thread.channel_id,
            created_at: thread.created_at
                .flatten()
                .and_then(|ts: String| ts.parse::<i64>().ok())
                .unwrap_or_default(),
            new_message_number: thread.new_message_number.unwrap_or_default(),
            status: thread.status,
            user_left: thread.user_left,
            closed_at: thread.closed_at.flatten().and_then(|ts: String| ts.parse::<i64>().ok()),
            closed_by: thread.closed_by,
            category_id: thread.category_id,
            category_name: thread.category_name,
            required_permissions: thread.required_permissions,
            messages,
        };

        return (StatusCode::OK, Json(serde_json::json!(complete)));
    }

    let threads_query = match sqlx::query!(
        r#"
        SELECT
            id,
            user_id,
            user_name,
            channel_id,
            strftime('%s', created_at) as "created_at: Option<String>",
            next_message_number as new_message_number,
            status,
            user_left,
            strftime('%s', closed_at) as "closed_at: Option<String>",
            closed_by,
            category_id,
            category_name,
            required_permissions
        FROM threads
        WHERE status = 0
        ORDER BY closed_at DESC
        "#
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Erreur SQL threads: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch threads"
                })),
            );
        }
    };

    let mut threads: Vec<CompleteThread> = Vec::new();

    for thread in threads_query {
        let messages_query = match sqlx::query!(
            r#"
            SELECT
                id,
                thread_id,
                user_id,
                user_name,
                is_anonymous,
                dm_message_id,
                inbox_message_id,
                message_number,
                created_at as "created_at: String",
                content
            FROM thread_messages
            WHERE thread_id = ?
            ORDER BY created_at ASC
            "#,
            thread.id
        )
        .fetch_all(&db_pool)
        .await
        {
            Ok(rows) => rows,
            Err(err) => {
                eprintln!("Erreur SQL messages pour {}: {:?}", thread.id, err);
                Vec::new()
            }
        };

        let messages: Vec<ThreadMessage> = messages_query
            .into_iter()
            .map(|m| ThreadMessage {
                id: m.id,
                thread_id: m.thread_id,
                user_id: m.user_id,
                user_name: m.user_name,
                is_anonymous: m.is_anonymous,
                dm_message_id: m.dm_message_id,
                inbox_message_id: m.inbox_message_id,
                message_number: m.message_number,
                created_at: m.created_at,
                content: m.content,
            })
            .collect();

        threads.push(CompleteThread {
            id: thread.id,
            user_id: thread.user_id,
            user_name: thread.user_name,
            channel_id: thread.channel_id,
            created_at: thread.created_at
                .flatten()
                .and_then(|ts: String| ts.parse::<i64>().ok())
                .unwrap_or_default(),
            new_message_number: thread.new_message_number.unwrap_or_default(),
            status: thread.status,
            user_left: thread.user_left,
            closed_at: thread.closed_at.flatten().and_then(|ts: String| ts.parse::<i64>().ok()),
            closed_by: thread.closed_by,
            category_id: thread.category_id,
            category_name: thread.category_name,
            required_permissions: thread.required_permissions,
            messages,
        });
    }

    (StatusCode::OK, Json(serde_json::json!(threads)))
}
