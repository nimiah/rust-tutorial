use axum::Json;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use validator::Validate;

use crate::db::DbTransaction;
use crate::models::common::{ApiResult, Response};
use crate::models::user::{RequestUser, RequestUserUpdate, User};
use crate::services::user_service::UserService;

#[utoipa::path(
    post,
    path = "/api/user",
    request_body = RequestUser,
    responses(
        (status = 200, description = "Created", body = Response<u32>),
    ),
    tag = "Users",
)]
pub async fn create_user(
    Extension(tx): Extension<DbTransaction>,
    Json(user): Json<RequestUser>,
) -> ApiResult<i32> {
    if let Err(e) = user.validate() {
        return Response::err(StatusCode::BAD_REQUEST, e.to_string());
    }

    let ret = UserService::new(tx).create_user(user).await;
    Response::from_result(ret)
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = usize, Path, description = "User ID")
    ),
    request_body = RequestUserUpdate,
    responses(
        (status = 200, description = "Updated"),
    ),
    tag = "Users",
    security(
        ("Bearer" = [])
    )
)]
pub async fn edit_user(
    Path(id): Path<i32>,
    Extension(tx): Extension<DbTransaction>,
    auth_user: Option<Extension<User>>,
    Json(user): Json<RequestUserUpdate>,
) -> ApiResult<()> {
    if auth_user.is_none() {
        return Response::err(StatusCode::UNAUTHORIZED, "Unauthorized".to_string());
    }

    let ret: Result<(), String> = UserService::new(tx).update_user(id, user).await;
    Response::from_result(ret)
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = usize, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User detail", body = Response<User>),
    ),
    tag = "Users",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_user_detail(
    Path(id): Path<i32>,
    auth_user: Option<Extension<User>>,
    Extension(tx): Extension<DbTransaction>,
) -> ApiResult<User> {
    if auth_user.is_none() {
        return Response::err(StatusCode::UNAUTHORIZED, "Unauthorized".to_string());
    }

    let user = UserService::new(tx).get_user(id).await;
    Response::from_optional(user)
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = usize, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Deleted"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users",
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_user(
    Path(id): Path<i32>,
    auth_user: Option<Extension<User>>,
    Extension(tx): Extension<DbTransaction>,
) -> ApiResult<()> {
    if auth_user.is_none() {
        return Response::err(StatusCode::UNAUTHORIZED, "Unauthorized".to_string());
    }

    let ret = UserService::new(tx).delete_user(id).await;
    Response::from_result(ret)
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "Result", body = Response<Vec<User>>),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_all_users(
    auth_user: Option<Extension<User>>,
    Extension(tx): Extension<DbTransaction>
) -> ApiResult<Vec<User>> {
    if auth_user.is_none() {
        return Response::err(StatusCode::UNAUTHORIZED, "Unauthorized".to_string());
    }

    let ret = UserService::new(tx).get_all_users().await;
    match ret {
        Some(users) => Response::ok(users),
        None => Response::ok(vec![]),
    }
}
