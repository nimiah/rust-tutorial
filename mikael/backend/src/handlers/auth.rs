use axum::http::StatusCode;
use axum::{Extension, Json};
use validator::Validate;

use crate::db::DbTransaction;
use crate::models::auth::{LoggedInUser, RequestLogin};
use crate::models::common::{ApiResult, Response};
use crate::services::user_service::UserService;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    responses(
        // (status = 200, description = "Auth token", body = Response<String>),
    ),
    tag = "Authentication"
)]
pub async fn login(
    Extension(tx): Extension<DbTransaction>,
    Json(req_login): Json<RequestLogin>,
) -> ApiResult<LoggedInUser> {
    if let Err(e) = req_login.validate() {
        return Response::err(StatusCode::BAD_REQUEST, e.to_string());
    }
    let result = UserService::new(tx).login(req_login).await;
    Response::from_result(result)    
}
