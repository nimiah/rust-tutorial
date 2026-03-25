use crate::{
    db::{DbTransaction, article_repository::ArticleRepository},
    models::article::{Article, RequestArticle},
};

pub struct ArticleService {
    article_repo: ArticleRepository,
}

impl ArticleService {
    pub fn new(tx: DbTransaction) -> Self {
        Self {
            article_repo: ArticleRepository::new(tx),
        }
    }

    pub async fn create_article(
        &self,
        article: RequestArticle,
        created_by_user: i32,
    ) -> Result<i32, String> {
        // CAP NHAT (bai 3): service tao bai viet, noi handler voi repository.
        self.article_repo
            .create(article, created_by_user)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_public_articles(&self) -> Result<Vec<Article>, String> {
        // CAP NHAT (bai 3): service cho homepage.
        self.article_repo
            .get_public_articles()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_my_articles(&self, user_id: i32) -> Result<Vec<Article>, String> {
        // CAP NHAT (bai 3): service cho man hinh bai viet cua user da login.
        self.article_repo
            .get_articles_by_user(user_id)
            .await
            .map_err(|e| e.to_string())
    }
}
