use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ================= ENUM =================
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Unlisted,
}

// ================= ARTICLE =================
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Article {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub visibility: Visibility, // ✅ đã đổi từ String → enum
    pub created_at: DateTime<Utc>,
    pub views: i64,
    pub likes: i64,
}

// ================= REQUEST =================
#[derive(Debug, Deserialize, ToSchema)]
pub struct RequestArticle {
    pub title: String,
    pub body: Option<String>,
    pub visibility: Visibility, // ✅ đã đổi từ String → enum
}