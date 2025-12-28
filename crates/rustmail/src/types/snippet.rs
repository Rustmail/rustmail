use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Snippet {
    pub id: i64,
    pub key: String,
    pub content: String,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
