// handlers/article.rs
use axum::{Extension, extract::Query};
use serde::Deserialize;

use crate::{
    db::DbTransaction,
    models::{
        article::{Article, ArticleVisibility},
        common::{ApiResult, Response},
        user::User,
    },
    services::article_service::ArticleService,
};

#[derive(Deserialize)]
pub struct ArticleParams {
    pub visibility: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/articles",
    // "/api/articles?visibility=public"
    // "/api/articles?visibility=public,unlisted"
    params(
        ("visibility" = Option<String>, Query, description = "Visibility filter: public,unlisted")
    ),
    responses(
        (status = 200, description = "Result", body = Response<Vec<Article>>),
        // (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Articles",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_articles(
    Extension(tx): Extension<DbTransaction>,
    user: Option<Extension<User>>,
    Query(params): Query<ArticleParams>,
) -> ApiResult<Vec<Article>> {
    let user_id = user.map(|u| u.id);

    let requested: Vec<ArticleVisibility> = params
        .visibility
        .map(|s| {
            s.split(',')
                .filter_map(|item| item.trim().parse().ok())
                .collect()
        })
        // FE should not send /api/articles
        // Unless, only return /api/articles?visibility=public
        .unwrap_or_else(|| vec![ArticleVisibility::Public]);

    let ret = ArticleService::new(tx)
        .get_articles(user_id, requested)
        .await;

    match ret {
        Some(articles) => Response::ok(articles),
        None => Response::ok(vec![]),
    }
}
