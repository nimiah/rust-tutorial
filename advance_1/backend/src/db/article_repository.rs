
use crate::{
    db::DbTransaction,
    models::article::{RequestArticle, Visibility}, // 🔧 ADD: import Visibility để dùng trong match convert
};
use sqlx::Row;
use sqlx::query;

use crate::{db::DbTransaction, models::article::Article};


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
pub async fn get_all(
    &self,
    user_id: i32,
) -> Result<Vec<String>, sqlx::Error> {
    let mut db = self.tx.lock().await;

    let rows = sqlx::query(
        r#"
        SELECT title
        FROM articles
        WHERE owner_id = $1 OR visibility = 'public'
        ORDER BY time_created DESC
        "#
    )
    .bind(user_id)
    .fetch_all(&mut **db)
    .await?;
    let articles = rows
        .into_iter()
        .map(|row| row.get::<String, _>("title"))
        .collect();

    Ok(articles)
}

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
