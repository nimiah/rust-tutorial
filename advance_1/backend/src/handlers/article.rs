use axum::{
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
