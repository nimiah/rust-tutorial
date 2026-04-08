// services/article_service.rs
use crate::{
    db::{DbTransaction, article_repository::ArticleRepository},
    models::article::{Article, ArticleVisibility},
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

    pub async fn get_articles(
        &self,
        user_id: Option<i32>,
        requested_visibilities: Vec<ArticleVisibility>,
    ) -> Option<Vec<Article>> {
        let visibilities = match user_id {
            None => vec![ArticleVisibility::Public],

            Some(_) => requested_visibilities,
        };

        // Future extend filter: Roles/Permissions <-> Visibilities here..

        self.article_repo.get_articles(visibilities).await
    }
}
