use crate::db::article_repository::ArticleRepository;
use crate::db::DbTransaction;
use crate::models::article::Article;

pub struct ArticleService {
    article_repo: ArticleRepository,
}

impl ArticleService {
    pub fn new(tx: DbTransaction) -> Self {
        Self {
            article_repo: ArticleRepository::new(tx),
        }
    }

    pub async fn get_all_articles(
        &self,
        user_id: i32,
    ) -> Result<Vec<Article>, String> {
        self.article_repo
            .get_by_user(user_id)
            .await
            .map_err(|e| e.to_string())
    }
}