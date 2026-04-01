use axum::{
    extract::State,
    Json,
};
use serde_json::json;

use crate::{
    db::DbTransaction,
    models::article::RequestArticle,
    services::article_service::ArticleService,
};

use utoipa::path;

#[utoipa::path(
    post,
    path = "/api/article",
    request_body = RequestArticle,
    responses(
        (status = 200, description = "Create article success")
    ),
    tag = "Articles"
)]
pub async fn create_article(
    State(tx): State<DbTransaction>,
    Json(payload): Json<RequestArticle>,
) -> Json<serde_json::Value> {
    let service = ArticleService::new(tx);

    // TODO: replace mock user_id with authenticated user.id from token
    let user_id = 1;

    match service.create_article(user_id, payload).await {
        Ok(id) => Json(json!({
            "message": "Success",
            "value": id
        })),
        Err(err) => Json(json!({
            "message": "Error",
            "error": err
        })),
    }
}