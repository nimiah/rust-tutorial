use crate::models::article::Article;
use crate::db::DbTransaction;

pub struct ArticleRepository {
    tx: DbTransaction,
}

impl ArticleRepository {
    pub fn new(tx: DbTransaction) -> Self {
        Self { tx }
    }

    pub async fn get_by_user(
        &self,
        user_id: i32,
    ) -> Result<Vec<Article>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let articles = sqlx::query_as::<_, Article>(
            "SELECT * FROM articles WHERE owner_id = $1"
        )
        .bind(user_id)
        .fetch_all(&mut **db)
        .await?;

        Ok(articles)
    }
}