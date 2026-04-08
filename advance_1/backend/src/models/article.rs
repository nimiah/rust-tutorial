use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema)]
pub struct RequestUpdateArticleVisibility {
    pub visibility: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema)]
pub struct ArticleVisibilityResponse {
    pub id: i32,
    pub owner_id: i32,
    pub visibility: String,
}

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

impl From<Article> for ArticleVisibilityResponse {
    fn from(article: Article) -> Self {
        Self {
            id: article.id,
            owner_id: article.owner_id,
            visibility: article.visibility,
        }
    }
}
