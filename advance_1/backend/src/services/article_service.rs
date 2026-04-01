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

        if payload.visibility != "public" && payload.visibility != "unlisted" {
            return Err("Invalid visibility".to_string());
        }

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
}