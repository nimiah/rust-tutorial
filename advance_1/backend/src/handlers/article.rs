use axum::{
    extract::{State, Extension},
    Json,
};
use serde_json::json;

use crate::{
    db::DbTransaction,
    models::{
        article::RequestArticle,
        user::User,
    },
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
    Extension(user): Extension<User>, // 👈 lấy user từ middleware
    Json(payload): Json<RequestArticle>,
) -> Json<serde_json::Value> {
    let service = ArticleService::new(tx);

    // ✅ lấy user_id từ user đã login
    let user_id = user.id;

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