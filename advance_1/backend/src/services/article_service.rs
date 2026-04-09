use crate::{
<<<<<<< HEAD
    db::{article_repository::ArticleRepository, DbTransaction},
    models::article::RequestArticle,
};

=======
    db::{DbTransaction, article_repository::ArticleRepository},
    models::article::Article,
};

#[derive(Debug)]
pub enum ArticleServiceError {
    NotFound,
    Forbidden,
    InvalidVisibility,
    Database(String),
}

>>>>>>> main
pub struct ArticleService {
    article_repo: ArticleRepository,
}

impl ArticleService {
    pub fn new(tx: DbTransaction) -> Self {
<<<<<<< HEAD
        ArticleService {
=======
        Self {
>>>>>>> main
            article_repo: ArticleRepository::new(tx),
        }
    }

<<<<<<< HEAD
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
=======
    pub async fn update_visibility(
        &self,
        user_id: i32,
        article_id: i32,
        new_visibility: String,
    ) -> Result<Article, ArticleServiceError> {
        let article = self
            .article_repo
            .get_by_id(article_id)
            .await
            .map_err(|e| ArticleServiceError::Database(e.to_string()))?;

        let Some(article) = article else {
            return Err(ArticleServiceError::NotFound);
        };

        if article.owner_id != user_id {
            return Err(ArticleServiceError::Forbidden);
        }

        match new_visibility.as_str() {
            "public" | "unlisted" => {}
            _ => return Err(ArticleServiceError::InvalidVisibility),
        }

        self.article_repo
            .update_visibility(article_id, &new_visibility)
            .await
            .map_err(|e| ArticleServiceError::Database(e.to_string()))
    }
}
>>>>>>> main
