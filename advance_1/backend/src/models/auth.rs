use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

//
// ===== REQUEST LOGIN =====
//
#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, Validate)]
pub struct RequestLogin {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 50))]
    pub password: String,
}

//
// ===== JWT CLAIMS =====
//
#[derive(Deserialize, Debug, Clone, Serialize, ToSchema)]
pub struct Claims {
<<<<<<< HEAD
    pub uid: i32,  // user id
    pub exp: i64,  // expire time
    pub iat: i64,  // issued at
}
=======
    pub uid: i32, // user id
    pub exp: i64, // expire time
    pub iat: i64, // issued at
}
>>>>>>> main
