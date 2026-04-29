use crate::{
    db::{DbTransaction, article_repository::ArticleRepository},
    models::article::{Article, RequestCreateArticle},
};

#[derive(Debug)]
pub enum ArticleServiceError {
    NotFound,
    Forbidden,
    InvalidVisibility,
    Database(String),
}

pub struct ArticleService {
    article_repo: ArticleRepository,
}

impl ArticleService {
    pub fn new(tx: DbTransaction) -> Self {
        Self {
            article_repo: ArticleRepository::new(tx),
        }
    }

    pub async fn get_article(
        &self,
        article_id: i32,
    ) -> Result<Option<Article>, ArticleServiceError> {
        self.article_repo
            .get_by_id(article_id)
            .await
            .map_err(|e| ArticleServiceError::Database(e.to_string()))
    }

    pub async fn get_all_articles(&self) -> Result<Vec<Article>, ArticleServiceError> {
        self.article_repo
            .get_all()
            .await
            .map_err(|e| ArticleServiceError::Database(e.to_string()))
    }

    pub async fn create_article(
        &self,
        user_id: i32,
        article: RequestCreateArticle,
    ) -> Result<Article, ArticleServiceError> {
        if article.title.trim().is_empty() {
            return Err(ArticleServiceError::Database(String::from(
                "title is required",
            )));
        }

        if let Some(visibility) = &article.visibility {
            match visibility.as_str() {
                "public" | "unlisted" => {}
                _ => return Err(ArticleServiceError::InvalidVisibility),
            }
        }

        self.article_repo
            .create(user_id, article)
            .await
            .map_err(|e| ArticleServiceError::Database(e.to_string()))
    }

    pub async fn like_article(&self, article_id: i32) -> Result<Article, ArticleServiceError> {
        self.article_repo
            .like(article_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ArticleServiceError::NotFound,
                other => ArticleServiceError::Database(other.to_string()),
            })
    }

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
