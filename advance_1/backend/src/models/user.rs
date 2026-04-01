use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, Validate)]
pub struct RequestUser {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub password_salt: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
