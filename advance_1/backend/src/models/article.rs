use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct Article {
    pub id: i32,
    pub owner_id: i32,
    pub time_created: DateTime<Utc>,
    pub visibility: String,
    pub title: String,
    pub body: Option<String>,
    pub description: Option<String>,
    pub views: i64,
    pub likes: i64,
}