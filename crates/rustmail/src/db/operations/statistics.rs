use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Serialize)]
pub struct StatisticsOverview {
    pub open_tickets: i64,
    pub total_closed: i64,
    pub closed_today: i64,
    pub closed_this_week: i64,
    pub closed_this_month: i64,
    pub avg_response_time_seconds: Option<i64>,
    pub avg_resolution_time_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DailyActivity {
    pub date: String,
    pub created: i64,
    pub closed: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryStats {
    pub name: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct CategoryRow {
    name: String,
    cnt: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct StaffMember {
    pub user_id: String,
    pub username: String,
    pub messages_count: i64,
    pub tickets_closed: i64,
    pub avg_response_time_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct StaffRow {
    user_id: i64,
    username: String,
    messages_count: i64,
    tickets_closed: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TopPerformer {
    pub user_id: String,
    pub username: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct FastestResponderRow {
    user_id: i64,
    username: String,
    avg_time: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct MostMessagesRow {
    user_id: i64,
    username: String,
    cnt: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct MostTicketsRow {
    user_id: String,
    username: String,
    cnt: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TopPerformers {
    pub fastest_responder: Option<TopPerformer>,
    pub most_messages: Option<TopPerformer>,
    pub most_tickets_closed: Option<TopPerformer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Statistics {
    pub overview: StatisticsOverview,
    pub activity: Vec<DailyActivity>,
    pub categories: Vec<CategoryStats>,
    pub staff_leaderboard: Vec<StaffMember>,
    pub top_performers: TopPerformers,
}

pub async fn get_statistics(pool: &SqlitePool, days: i64) -> Result<Statistics, sqlx::Error> {
    let overview = get_overview(pool).await?;
    let activity = get_daily_activity(pool, days).await?;
    let categories = get_category_stats(pool).await?;
    let staff_leaderboard = get_staff_leaderboard(pool, days).await?;
    let top_performers = get_top_performers(pool).await?;

    Ok(Statistics {
        overview,
        activity,
        categories,
        staff_leaderboard,
        top_performers,
    })
}

async fn get_overview(pool: &SqlitePool) -> Result<StatisticsOverview, sqlx::Error> {
    let open_tickets: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE status = 1")
        .fetch_one(pool)
        .await?;

    let total_closed: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE status = 0")
        .fetch_one(pool)
        .await?;

    let closed_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE status = 0 AND closed_at >= strftime('%s', 'now', 'start of day')"
    )
    .fetch_one(pool)
    .await?;

    let closed_this_week: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE status = 0 AND closed_at >= strftime('%s', 'now', '-7 days')"
    )
    .fetch_one(pool)
    .await?;

    let closed_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE status = 0 AND closed_at >= strftime('%s', 'now', 'start of month')"
    )
    .fetch_one(pool)
    .await?;

    let avg_response_time: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT CAST(AVG(first_response_time) AS INTEGER)
        FROM (
            SELECT strftime('%s', MIN(m.created_at)) - strftime('%s', t.created_at) AS first_response_time
            FROM threads t
            JOIN thread_messages m ON m.thread_id = t.id
            WHERE m.message_number IS NOT NULL
            AND t.status = 0
            GROUP BY t.id
            HAVING first_response_time > 0
        )
        "#
    )
    .fetch_optional(pool)
    .await?
    .flatten();

    let avg_resolution_time: Option<i64> = sqlx::query_scalar(
        "SELECT CAST(AVG(closed_at - strftime('%s', created_at)) AS INTEGER) FROM threads WHERE status = 0 AND closed_at IS NOT NULL"
    )
    .fetch_optional(pool)
    .await?
    .flatten();

    Ok(StatisticsOverview {
        open_tickets,
        total_closed,
        closed_today,
        closed_this_week,
        closed_this_month,
        avg_response_time_seconds: avg_response_time,
        avg_resolution_time_seconds: avg_resolution_time,
    })
}

async fn get_daily_activity(
    pool: &SqlitePool,
    days: i64,
) -> Result<Vec<DailyActivity>, sqlx::Error> {
    use chrono::{Duration, Utc};
    use std::collections::HashMap;

    let today = Utc::now().date_naive();
    let start_date = today - Duration::days(days - 1);
    let start_str = start_date.format("%Y-%m-%d").to_string();

    #[derive(sqlx::FromRow)]
    struct CountRow {
        day: String,
        cnt: i64,
    }

    let created_rows: Vec<CountRow> = sqlx::query_as(
        "SELECT date(created_at) as day, COUNT(*) as cnt FROM threads WHERE date(created_at) >= ? GROUP BY day"
    )
    .bind(&start_str)
    .fetch_all(pool)
    .await?;

    let closed_rows: Vec<CountRow> = sqlx::query_as(
        "SELECT date(closed_at, 'unixepoch') as day, COUNT(*) as cnt FROM threads WHERE status = 0 AND closed_at IS NOT NULL AND date(closed_at, 'unixepoch') >= ? GROUP BY day"
    )
    .bind(&start_str)
    .fetch_all(pool)
    .await?;

    let created_map: HashMap<String, i64> =
        created_rows.into_iter().map(|r| (r.day, r.cnt)).collect();
    let closed_map: HashMap<String, i64> =
        closed_rows.into_iter().map(|r| (r.day, r.cnt)).collect();

    let mut results = Vec::new();
    for i in 0..days {
        let date = start_date + Duration::days(i);
        let date_str = date.format("%Y-%m-%d").to_string();
        results.push(DailyActivity {
            date: date_str.clone(),
            created: *created_map.get(&date_str).unwrap_or(&0),
            closed: *closed_map.get(&date_str).unwrap_or(&0),
        });
    }

    Ok(results)
}

async fn get_category_stats(pool: &SqlitePool) -> Result<Vec<CategoryStats>, sqlx::Error> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE status = 0")
        .fetch_one(pool)
        .await?;

    if total == 0 {
        return Ok(vec![]);
    }

    let rows: Vec<CategoryRow> = sqlx::query_as(
        r#"
        SELECT
            COALESCE(category_name, 'Uncategorized') as name,
            COUNT(*) as cnt
        FROM threads
        WHERE status = 0
        GROUP BY category_name
        ORDER BY cnt DESC
        LIMIT 10
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| CategoryStats {
            name: r.name,
            count: r.cnt,
            percentage: (r.cnt as f64 / total as f64) * 100.0,
        })
        .collect())
}

async fn get_staff_leaderboard(
    pool: &SqlitePool,
    days: i64,
) -> Result<Vec<StaffMember>, sqlx::Error> {
    let rows: Vec<StaffRow> = sqlx::query_as(
        r#"
        SELECT
            m.user_id as user_id,
            m.user_name as username,
            COUNT(*) as messages_count,
            COALESCE(closed.tickets_closed, 0) as tickets_closed
        FROM thread_messages m
        JOIN threads t ON m.thread_id = t.id
        LEFT JOIN (
            SELECT closed_by, COUNT(*) as tickets_closed
            FROM threads
            WHERE status = 0
            AND closed_at >= strftime('%s', 'now', '-' || ? || ' days')
            GROUP BY closed_by
        ) closed ON CAST(m.user_id AS TEXT) = closed.closed_by
        WHERE m.message_number IS NOT NULL
        AND m.created_at >= strftime('%s', 'now', '-' || ? || ' days')
        GROUP BY m.user_id, m.user_name
        ORDER BY messages_count DESC
        LIMIT 20
        "#,
    )
    .bind(days)
    .bind(days)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| StaffMember {
            user_id: r.user_id.to_string(),
            username: r.username,
            messages_count: r.messages_count,
            tickets_closed: r.tickets_closed,
            avg_response_time_seconds: None,
        })
        .collect())
}

async fn get_top_performers(pool: &SqlitePool) -> Result<TopPerformers, sqlx::Error> {
    let fastest: Option<FastestResponderRow> = sqlx::query_as(
        r#"
        SELECT
            m.user_id as user_id,
            m.user_name as username,
            CAST(AVG(response_time) AS INTEGER) as avg_time
        FROM (
            SELECT
                m.user_id,
                m.user_name,
                strftime('%s', MIN(m.created_at)) - strftime('%s', t.created_at) AS response_time
            FROM thread_messages m
            JOIN threads t ON m.thread_id = t.id
            WHERE m.message_number IS NOT NULL
            GROUP BY m.thread_id, m.user_id, m.user_name
            HAVING response_time > 0
        ) m
        GROUP BY m.user_id, m.user_name
        HAVING COUNT(*) >= 5
        ORDER BY avg_time ASC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    let most_messages: Option<MostMessagesRow> = sqlx::query_as(
        r#"
        SELECT
            user_id as user_id,
            user_name as username,
            COUNT(*) as cnt
        FROM thread_messages
        WHERE message_number IS NOT NULL
        GROUP BY user_id, user_name
        ORDER BY cnt DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    let most_tickets: Option<MostTicketsRow> = sqlx::query_as(
        r#"
        SELECT
            t.closed_by as user_id,
            COALESCE(m.user_name, t.closed_by) as username,
            COUNT(*) as cnt
        FROM threads t
        LEFT JOIN (
            SELECT DISTINCT CAST(user_id AS TEXT) as user_id_str, user_name
            FROM thread_messages
            WHERE message_number IS NOT NULL
        ) m ON m.user_id_str = t.closed_by
        WHERE t.status = 0 AND t.closed_by IS NOT NULL
        GROUP BY t.closed_by
        ORDER BY cnt DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(TopPerformers {
        fastest_responder: fastest.map(|r| TopPerformer {
            user_id: r.user_id.to_string(),
            username: r.username,
            value: r.avg_time,
        }),
        most_messages: most_messages.map(|r| TopPerformer {
            user_id: r.user_id.to_string(),
            username: r.username,
            value: r.cnt,
        }),
        most_tickets_closed: most_tickets.map(|r| TopPerformer {
            user_id: r.user_id.clone(),
            username: r.username,
            value: r.cnt,
        }),
    })
}
