use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ToSchema)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub visibility: String,
    pub owner_id: i32,
}