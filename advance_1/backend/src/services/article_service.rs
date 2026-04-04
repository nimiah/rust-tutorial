use crate::{
    db::{article_repository::ArticleRepository, DbTransaction},
    models::article::RequestArticle,
};

pub struct ArticleService {
    article_repo: ArticleRepository,
}

impl ArticleService {
    pub fn new(tx: DbTransaction) -> Self {
        ArticleService {
            article_repo: ArticleRepository::new(tx),
        }
    }

    pub async fn create_article(
        &self,
        user_id: i32,
        payload: RequestArticle,
    ) -> Result<i32, String> {
        // ================= VALIDATION =================

        if payload.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        // 🔧 FIX: xoá validate visibility bằng string
        // Visibility là enum — serde tự reject JSON không hợp lệ khi deserialize
        // Không thể và không cần so sánh enum với &str

        // ================= CALL REPOSITORY =================

        let result = self
            .article_repo
            .create(user_id, payload)
            .await;

        match result {
            Ok(id) => Ok(id),
            Err(_) => Err("Failed to create article".to_string()),
        }
    }

    pub async fn get_all(
        &self,
        user_id: i32,
    ) -> Result<Vec<String>, String> {
        let result = self.article_repo.get_all(user_id).await;

         match result {
            Ok(data) => Ok(data),
            Err(_) => Err("Failed to get articles".to_string()),
        }
    }
}