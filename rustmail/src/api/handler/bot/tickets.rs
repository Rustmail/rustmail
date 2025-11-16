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
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub status: Option<i64>,
    pub category_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedThreadsResponse {
    pub threads: Vec<CompleteThread>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
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

    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(50).min(200).max(1);
    let offset = (page - 1) * page_size;

    let status_filter = params.status.unwrap_or(0);

    let mut where_conditions = vec![format!("status = {}", status_filter)];

    if let Some(ref cat_id) = params.category_id {
        where_conditions.push(format!("category_id = '{}'", cat_id.replace("'", "''")));
    }

    let where_clause = where_conditions.join(" AND ");

    let sort_column = match params.sort_by.as_deref() {
        Some("user_name") => "user_name",
        Some("closed_at") => "closed_at",
        Some("created_at") => "created_at",
        _ => "created_at",
    };

    let sort_order = match params.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    let count_query = format!("SELECT COUNT(*) as count FROM threads WHERE {}", where_clause);
    let total: i64 = match sqlx::query_scalar(&count_query)
        .fetch_one(&db_pool)
        .await
    {
        Ok(count) => count,
        Err(err) => {
            eprintln!("Erreur SQL count: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to count threads"
                })),
            );
        }
    };

    let total_pages = (total as f64 / page_size as f64).ceil() as i64;

    let query_str = format!(
        r#"
        SELECT
            id,
            user_id,
            user_name,
            channel_id,
            strftime('%s', created_at) as created_at,
            next_message_number as new_message_number,
            status,
            user_left,
            strftime('%s', closed_at) as closed_at,
            closed_by,
            category_id,
            category_name,
            required_permissions
        FROM threads
        WHERE {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        where_clause, sort_column, sort_order, page_size, offset
    );

    let threads_query = match sqlx::query_as::<_, (
        String,
        i64,
        String,
        String,
        Option<String>,
        Option<i64>,
        i64,
        bool,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )>(&query_str)
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

    let thread_ids: Vec<String> = threads_query.iter().map(|t| t.0.clone()).collect();

    let placeholders = thread_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let messages_query_str = format!(
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
            created_at,
            content
        FROM thread_messages
        WHERE thread_id IN ({})
        ORDER BY thread_id, created_at ASC
        "#,
        placeholders
    );

    let mut messages_query = sqlx::query_as::<_, (
        i64,
        String,
        i64,
        String,
        bool,
        Option<String>,
        Option<String>,
        Option<i64>,
        String,
        String,
    )>(&messages_query_str);

    for thread_id in &thread_ids {
        messages_query = messages_query.bind(thread_id);
    }

    let all_messages = messages_query.fetch_all(&db_pool).await.unwrap_or_else(|err| {
        eprintln!("Erreur SQL messages batch: {:?}", err);
        Vec::new()
    });

    let mut messages_by_thread: std::collections::HashMap<String, Vec<ThreadMessage>> =
        std::collections::HashMap::new();

    for msg in all_messages {
        messages_by_thread.entry(msg.1.clone()).or_insert_with(Vec::new).push(ThreadMessage {
            id: msg.0,
            thread_id: msg.1.clone(),
            user_id: msg.2,
            user_name: msg.3,
            is_anonymous: msg.4,
            dm_message_id: msg.5,
            inbox_message_id: msg.6,
            message_number: msg.7,
            created_at: msg.8,
            content: msg.9,
        });
    }

    for thread in threads_query {
        let messages = messages_by_thread.get(&thread.0).cloned().unwrap_or_default();

        threads.push(CompleteThread {
            id: thread.0.clone(),
            user_id: thread.1,
            user_name: thread.2,
            channel_id: thread.3,
            created_at: thread.4
                .and_then(|ts: String| ts.parse::<i64>().ok())
                .unwrap_or_default(),
            new_message_number: thread.5.unwrap_or_default(),
            status: thread.6,
            user_left: thread.7,
            closed_at: thread.8.and_then(|ts: String| ts.parse::<i64>().ok()),
            closed_by: thread.9,
            category_id: thread.10,
            category_name: thread.11,
            required_permissions: thread.12,
            messages,
        });
    }

    let response = PaginatedThreadsResponse {
        threads,
        total,
        page,
        page_size,
        total_pages,
    };

    (StatusCode::OK, Json(serde_json::json!(response)))
}
