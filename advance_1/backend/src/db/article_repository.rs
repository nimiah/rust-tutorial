use crate::{db::DbTransaction, models::article::Article};

pub struct ArticleRepository {
    tx: DbTransaction,
}

impl ArticleRepository {
    pub fn new(tx: DbTransaction) -> Self {
        Self { tx }
    }

    pub async fn get_by_id(&self, article_id: i32) -> Result<Option<Article>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        sqlx::query_as::<_, Article>(
            "SELECT id, owner_id, time_created, visibility, title, body, description, views, likes
            FROM articles
            WHERE id = $1",
        )
        .bind(article_id)
        .fetch_optional(&mut *db.as_mut())
        .await
    }

    pub async fn update_visibility(
        &self,
        article_id: i32,
        visibility: &str,
    ) -> Result<Article, sqlx::Error> {
        let mut db = self.tx.lock().await;

        sqlx::query_as::<_, Article> (
          "UPDATE articles
             SET visibility = $1
             WHERE id = $2
             RETURNING id, owner_id, time_created, visibility, title, body, description, views, likes",
        ).bind(visibility)
        .bind(article_id)
        .fetch_one(&mut *db.as_mut())
        .await
    }
}
