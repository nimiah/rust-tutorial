use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use validator::Validate;

use crate::db::DbTransaction;
use crate::models::{
    article::{Article, RequestArticle},
    common::{ApiResult, Response},
    user::User,
};
use crate::services::article_service::ArticleService;

#[utoipa::path(
    post,
    path = "/api/articles",
    request_body = RequestArticle,
    responses(
        (status = 200, description = "Created", body = Response<i32>),
    ),
    tag = "Articles",
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_article(
    Extension(tx): Extension<DbTransaction>,
    Extension(login_user): Extension<User>,
    Json(article): Json<RequestArticle>,
) -> ApiResult<i32> {
    // CAP NHAT (bai 3): validate struct request truoc khi xu ly business logic.
    if let Err(e) = article.validate() {
        return Response::err(StatusCode::BAD_REQUEST, e.to_string());
    }

    // CAP NHAT (bai 3): visibility chi duoc nam trong 3 gia tri cua de bai.
    if let Err(e) = article.validate_visibility() {
        return Response::err(StatusCode::BAD_REQUEST, e);
    }

    // CAP NHAT (bai 3): khong cho client tu gui created_by_user,
    // ma lay user dang login lam chu so huu bai viet.
    let ret = ArticleService::new(tx)
        .create_article(article, login_user.id)
        .await;
    Response::from_result(ret)
}

#[utoipa::path(
    get,
    path = "/api/articles/homepage",
    responses(
        (status = 200, description = "Homepage public articles", body = Response<Vec<Article>>),
    ),
    tag = "Articles"
)]
pub async fn get_homepage_articles(
    Extension(tx): Extension<DbTransaction>,
) -> ApiResult<Vec<Article>> {
    // CAP NHAT (bai 3): homepage chi hien thi cac bai public.
    let ret = ArticleService::new(tx).get_public_articles().await;
    Response::from_result(ret)
}

#[utoipa::path(
    get,
    path = "/api/articles/me",
    responses(
        (status = 200, description = "Logged-in user articles", body = Response<Vec<Article>>),
    ),
    tag = "Articles",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_my_articles(
    Extension(tx): Extension<DbTransaction>,
    Extension(login_user): Extension<User>,
) -> ApiResult<Vec<Article>> {
    // CAP NHAT (bai 3): user da login xem duoc tat ca bai viet cua chinh minh,
    // gom public / unlisted / draft.
    let ret = ArticleService::new(tx).get_my_articles(login_user.id).await;
    Response::from_result(ret)
}
