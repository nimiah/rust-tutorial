use crate::{
    db::DbTransaction,
    models::article::{RequestArticle, Visibility}, // 🔧 ADD: import Visibility để dùng trong match convert
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

        // 🔧 FIX: convert enum Visibility → &str để SQLx có thể bind vào SQL query
        let visibility_str = match article.visibility {
            Visibility::Public => "public",
            Visibility::Unlisted => "unlisted",
        };

        let row = sqlx::query(
            r#"
            INSERT INTO articles (owner_id, title, body, description, visibility)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
            // 🔧 FIX: owner_id thay vì user_id theo schema DB
            // 🔧 FIX: thêm description
            // 🔧 FIX: bỏ views, likes — DB tự set default 0
        )
        .bind(owner_id)
        .bind(&article.title)
        .bind(&article.body)
        .bind(&article.description)  // 🔧 ADD: bind description
        .bind(visibility_str)         // 🔧 FIX: bind &str thay vì enum trực tiếp
        .fetch_one(&mut *db.as_mut())
        .await?;

        let id: i32 = row.try_get("id")?;
        Ok(id)
    }
}
