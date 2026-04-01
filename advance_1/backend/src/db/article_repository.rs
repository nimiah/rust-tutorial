use crate::{
    db::DbTransaction,
    models::article::RequestArticle,
};
use sqlx::Row;

pub struct ArticleRepository {
    tx: DbTransaction,
}

impl ArticleRepository {
    pub fn new(tx: DbTransaction) -> Self {
        ArticleRepository { tx }
    }

    pub async fn create(
        &self,
        owner_id: i32,
        article: RequestArticle,
    ) -> Result<i32, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let row = sqlx::query(
            r#"
            INSERT INTO articles (user_id, title, body, visibility, views, likes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(owner_id)
        .bind(&article.title)
        .bind(&article.body)
        .bind(&article.visibility)
        .bind(0_i64)
        .bind(0_i64)
        .fetch_one(&mut *db.as_mut())
        .await?;

        let id: i32 = row.try_get("id")?;
        Ok(id)
    }
}