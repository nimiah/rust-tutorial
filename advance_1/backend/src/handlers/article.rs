use axum::{extract::Extension, Json};
use serde_json::json;
use crate::models::article::Article;
use crate::models::user::User;
use crate::services::article_service::ArticleService;
use crate::db::DbTransaction;
#[utoipa::path(
    get,
    path = "/api/articles",
    responses(
        (status = 200, description = "Get all articles", body = [Article])
    ),
    tag = "Articles"
)]
pub async fn get_all_articles(
    Extension(user): Extension<User>,
    Extension(tx): Extension<DbTransaction>,
) -> Json<serde_json::Value> {
    let service = ArticleService::new(tx);

    match service.get_all_articles(user.id).await {
        Ok(articles) => Json(json!({
            "message": "Success",
            "value": articles
        })),
        Err(e) => Json(json!({
            "message": "Error",
            "error": e
        })),
    }
}