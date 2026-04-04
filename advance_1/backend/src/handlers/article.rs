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

    let articles = service.get_all(user.id).await;

    Response::from_result(articles)
}