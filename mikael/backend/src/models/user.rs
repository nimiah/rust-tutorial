use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub created_at: DateTime<Utc>,
}

#[derive(ToSchema, Validate, Deserialize)]
pub struct RequestUser {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, utoipa::ToSchema, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}