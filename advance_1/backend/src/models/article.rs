use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Type, prelude::FromRow};
use std::str::FromStr;
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

#[derive(Debug, Deserialize, Serialize, Type, ToSchema, PartialEq, Clone)]
#[sqlx(type_name = "article_visibility", rename_all = "lowercase")]
pub enum ArticleVisibility {
    Public,
    Unlisted,
}

impl FromStr for ArticleVisibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "public" => Ok(ArticleVisibility::Public),
            "unlisted" => Ok(ArticleVisibility::Unlisted),
            _ => Err(()),
        }
    }
}
