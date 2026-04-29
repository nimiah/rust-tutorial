use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};

use crate::db::DbTransaction;
use crate::models::{
    article::{
        Article, ArticleVisibilityResponse, RequestCreateArticle, RequestUpdateArticleVisibility,
    },
    common::{ApiResult, Response},
    user::User,
};
use crate::services::article_service::{ArticleService, ArticleServiceError};

#[utoipa::path(
    get,
    path = "/api/articles",
    responses(
        (status = 200, description = "Article list", body = Response<Vec<Article>>),
        (status = 400, description = "Database error"),
    ),
    tag = "Articles"
)]
pub async fn get_all_articles(Extension(tx): Extension<DbTransaction>) -> ApiResult<Vec<Article>> {
    let result = ArticleService::new(tx).get_all_articles().await;

    match result {
        Ok(articles) => Response::ok(articles),
        Err(ArticleServiceError::Database(message)) => {
            Response::err(StatusCode::BAD_REQUEST, message)
        }
        Err(_) => Response::err(
            StatusCode::BAD_REQUEST,
            String::from("Cannot get article list"),
        ),
    }
}

#[utoipa::path(
    get,
    path = "/api/articles/{id}",
    params(
        ("id" = i32, Path, description = "Article ID")
    ),
    responses(
        (status = 200, description = "Article detail", body = Response<Article>),
        (status = 404, description = "Article not found"),
        (status = 400, description = "Database error"),
    ),
    tag = "Articles"
)]
pub async fn get_article_detail(
    Path(id): Path<i32>,
    Extension(tx): Extension<DbTransaction>,
) -> ApiResult<Article> {
    let result = ArticleService::new(tx).get_article(id).await;

    match result {
        Ok(article) => Response::from_optional(article),
        Err(ArticleServiceError::Database(message)) => {
            Response::err(StatusCode::BAD_REQUEST, message)
        }
        Err(_) => Response::err(StatusCode::BAD_REQUEST, String::from("Cannot get article")),
    }
}

#[utoipa::path(
    post,
    path = "/api/articles",
    request_body = RequestCreateArticle,
    responses(
        (status = 200, description = "Created article", body = Response<Article>),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Articles",
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_article(
    Extension(tx): Extension<DbTransaction>,
    login_user: Option<Extension<User>>,
    Json(req): Json<RequestCreateArticle>,
) -> ApiResult<Article> {
    let Some(Extension(login_user)) = login_user else {
        return Response::err(StatusCode::UNAUTHORIZED, String::from("Unauthorized"));
    };

    let result = ArticleService::new(tx)
        .create_article(login_user.id, req)
        .await;

    match result {
        Ok(article) => Response::ok(article),
        Err(ArticleServiceError::InvalidVisibility) => Response::err(
            StatusCode::BAD_REQUEST,
            String::from("visibility must be either public or unlisted"),
        ),
        Err(ArticleServiceError::Database(message)) => {
            Response::err(StatusCode::BAD_REQUEST, message)
        }
        Err(_) => Response::err(
            StatusCode::BAD_REQUEST,
            String::from("Cannot create article"),
        ),
    }
}

#[utoipa::path(
    patch,
    path = "/api/articles/{id}/like",
    params(
        ("id" = i32, Path, description = "Article ID")
    ),
    responses(
        (status = 200, description = "Liked article", body = Response<Article>),
        (status = 404, description = "Article not found"),
        (status = 400, description = "Database error"),
    ),
    tag = "Articles"
)]
pub async fn like_article(
    Path(id): Path<i32>,
    Extension(tx): Extension<DbTransaction>,
) -> ApiResult<Article> {
    let result = ArticleService::new(tx).like_article(id).await;

    match result {
        Ok(article) => Response::ok(article),
        Err(ArticleServiceError::NotFound) => {
            Response::err(StatusCode::NOT_FOUND, String::from("Article not found"))
        }
        Err(ArticleServiceError::Database(message)) => {
            Response::err(StatusCode::BAD_REQUEST, message)
        }
        Err(_) => Response::err(StatusCode::BAD_REQUEST, String::from("Cannot like article")),
    }
}

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
