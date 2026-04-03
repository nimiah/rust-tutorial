use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ================= ENUM =================

// 🔧 FIX: thêm sqlx::Type vào derive list
//         Visibility cần implement sqlx::Type + Encode + Decode để:
//         (1) Article struct dùng được với #[derive(FromRow)]
//         (2) Repository có thể .bind(&article.visibility) trực tiếp
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "lowercase")] // 🔧 FIX: map enum variant → lowercase string trong PostgreSQL TEXT column
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
    pub description: Option<String>, // 🔧 ADD: thêm field description theo schema DB
    pub visibility: Visibility,      // 🔧 FIX: giữ Visibility enum — compile được nhờ sqlx::Type ở trên
    pub created_at: DateTime<Utc>,
    pub views: i64,
    pub likes: i64,
}

// ================= REQUEST =================

#[derive(Debug, Deserialize, ToSchema)]
pub struct RequestArticle {
    pub title: String,
    pub body: Option<String>,
    pub description: Option<String>, // 🔧 ADD: thêm field description để client truyền lên
    pub visibility: Visibility,
}
