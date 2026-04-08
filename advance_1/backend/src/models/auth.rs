use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, Validate)]
pub struct RequestLogin {
    #[validate(length(min = 2, max = 50))]
    pub email: String,
    #[validate(length(min = 2, max = 50))]
    pub password: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, Validate)]
pub struct Claims {
    pub uid: i32,
    pub exp: i64,
    pub iat: i64,
}
