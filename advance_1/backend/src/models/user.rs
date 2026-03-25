use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, Validate)]
pub struct RequestUser {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    // CAP NHAT (bai 1): them field phone de request tu API co the day xuong database.
    pub phone: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    // CAP NHAT (bai 1): them field phone de du lieu doc tu bang users_demo map ve Rust duoc.
    pub phone: Option<String>,
}
