use axum::{

    extract::Extension, // 🔧 FIX: bỏ State — DbTransaction được inject bởi middleware trans.rs
    Json,
};

use crate::{
    db::DbTransaction,
    models::{
        article::RequestArticle,
        common::{ApiResult, Response}, // 🔧 ADD: import ApiResult và Response theo chuẩn project
        user::User,
    },
    services::article_service::ArticleService,
};

#[utoipa::path(
    post,
    path = "/api/articles", // 🔧 FIX: đổi /api/article → /api/articles
    request_body = RequestArticle,
    responses(
        (status = 200, description = "Create article success")
    ),
    tag = "Articles"
)]
pub async fn create_article(
    Extension(tx): Extension<DbTransaction>, // 🔧 FIX: State(tx) → Extension(tx) theo pattern của project
    Extension(user): Extension<User>,
    Json(payload): Json<RequestArticle>,
) -> ApiResult<i32> { // 🔧 FIX: Json<serde_json::Value> → ApiResult<i32> theo chuẩn Response<T>
    let service = ArticleService::new(tx);

    // owner_id lấy từ user đã xác thực qua JWT — không hardcode
    let user_id = user.id;

    let result = service.create_article(user_id, payload).await;

    Response::from_result(result) // 🔧 FIX: dùng Response::from_result thay vì json!() thủ công
}
pub async fn get_all_articles(
    Extension(tx): Extension<DbTransaction>,
    Extension(user): Extension<User>,
) -> ApiResult<Vec<String>> {

    let service = ArticleService::new(tx);

    let  result = service.get_all(user.id).await;

    Response::from_result( result)
}

    Json,
    extract::{Extension, Path},
    http::StatusCode,
};

use crate::db::DbTransaction;
use crate::models::{
    article::{ArticleVisibilityResponse, RequestUpdateArticleVisibility},
    common::{ApiResult, Response},
    user::User,
};
use crate::services::article_service::{ArticleService, ArticleServiceError};

#[utoipa::path(
    patch,
    path = "/api/articles/{id}/visibility",
    params(
        ("id" = i32, Path, description = "Article ID")
    ),
    request_body = RequestUpdateArticleVisibility,
    responses(
        (status = 200, description = "Update visibility", body = Response<ArticleVisibilityResponse>),
        (status = 400, description = "Invalid visibility"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Article not found"),
    ),
    tag = "Articles",
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_article_visibility(
    Path(id): Path<i32>,
    Extension(tx): Extension<DbTransaction>,
    login_user: Option<Extension<User>>,
    Json(req): Json<RequestUpdateArticleVisibility>,
) -> ApiResult<ArticleVisibilityResponse> {
    let Some(Extension(login_user)) = login_user else {
        return Response::err(StatusCode::UNAUTHORIZED, String::from("Unauthorized"));
    };

    let result = ArticleService::new(tx)
        .update_visibility(login_user.id, id, req.visibility)
        .await;

    match result {
        Ok(article) => Response::ok(article.into()),
        Err(ArticleServiceError::NotFound) => {
            Response::err(StatusCode::NOT_FOUND, String::from("Article not found"))
        }
        Err(ArticleServiceError::Forbidden) => Response::err(
            StatusCode::FORBIDDEN,
            String::from("You can only update your own article"),
        ),
        Err(ArticleServiceError::InvalidVisibility) => Response::err(
            StatusCode::BAD_REQUEST,
            String::from("visibility must be either public or unlisted"),
        ),
        Err(ArticleServiceError::Database(message)) => {
            Response::err(StatusCode::BAD_REQUEST, message)
        }
    }
}

