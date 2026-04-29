use crate::{
    db::DbTransaction,
    models::article::{Article, RequestCreateArticle},
};

pub struct ArticleRepository {
    tx: DbTransaction,
}

impl ArticleRepository {
    pub fn new(tx: DbTransaction) -> Self {
        Self { tx }
    }

    pub async fn get_by_id(&self, article_id: i32) -> Result<Option<Article>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        for (table_name, time_column) in article_table_candidates() {
            let sql = format!(
                "SELECT id, owner_id, {time_column} AS time_created, visibility, title, body, description, views, likes
                FROM {table_name}
                WHERE id = $1"
            );

            match sqlx::query_as::<_, Article>(&sql)
                .bind(article_id)
                .fetch_optional(&mut *db.as_mut())
                .await
            {
                Ok(article) => return Ok(article),
                Err(error) if is_missing_table_error(&error) => continue,
                Err(error) => return Err(error),
            }
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn get_all(&self) -> Result<Vec<Article>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        for (table_name, time_column) in article_table_candidates() {
            let sql = format!(
                "SELECT id, owner_id, {time_column} AS time_created, visibility, title, body, description, views, likes
                FROM {table_name}
                ORDER BY time_created DESC, id DESC"
            );

            match sqlx::query_as::<_, Article>(&sql)
                .fetch_all(&mut *db.as_mut())
                .await
            {
                Ok(articles) => return Ok(articles),
                Err(error) if is_missing_table_error(&error) => continue,
                Err(error) => return Err(error),
            }
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn create(
        &self,
        owner_id: i32,
        article: RequestCreateArticle,
    ) -> Result<Article, sqlx::Error> {
        let mut db = self.tx.lock().await;
        let visibility = article
            .visibility
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("public");

        for (table_name, time_column) in article_table_candidates() {
            let sql = format!(
                "INSERT INTO {table_name} (owner_id, title, body, description, visibility)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, owner_id, {time_column} AS time_created, visibility, title, body, description, views, likes"
            );

            match sqlx::query_as::<_, Article>(&sql)
                .bind(owner_id)
                .bind(&article.title)
                .bind(&article.body)
                .bind(&article.description)
                .bind(visibility)
                .fetch_one(&mut *db.as_mut())
                .await
            {
                Ok(article) => return Ok(article),
                Err(error) if is_missing_table_error(&error) => continue,
                Err(error) => return Err(error),
            }
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn like(&self, article_id: i32) -> Result<Article, sqlx::Error> {
        let mut db = self.tx.lock().await;

        for (table_name, time_column) in article_table_candidates() {
            let sql = format!(
                "UPDATE {table_name}
                SET likes = likes + 1
                WHERE id = $1
                RETURNING id, owner_id, {time_column} AS time_created, visibility, title, body, description, views, likes"
            );

            match sqlx::query_as::<_, Article>(&sql)
                .bind(article_id)
                .fetch_one(&mut *db.as_mut())
                .await
            {
                Ok(article) => return Ok(article),
                Err(error) if is_missing_table_error(&error) => continue,
                Err(error) => return Err(error),
            }
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn update_visibility(
        &self,
        article_id: i32,
        visibility: &str,
    ) -> Result<Article, sqlx::Error> {
        let mut db = self.tx.lock().await;

        for (table_name, time_column) in article_table_candidates() {
            let sql = format!(
                "UPDATE {table_name}
                SET visibility = $1
                WHERE id = $2
                RETURNING id, owner_id, {time_column} AS time_created, visibility, title, body, description, views, likes"
            );

            match sqlx::query_as::<_, Article>(&sql)
                .bind(visibility)
                .bind(article_id)
                .fetch_one(&mut *db.as_mut())
                .await
            {
                Ok(article) => return Ok(article),
                Err(error) if is_missing_table_error(&error) => continue,
                Err(error) => return Err(error),
            }
        }

        Err(sqlx::Error::RowNotFound)
    }
}

fn article_table_candidates() -> [(&'static str, &'static str); 3] {
    [
        ("articles", "created_at"),
        ("article", "created_at"),
        ("articles_demo", "time_created"),
    ]
}

fn is_missing_table_error(error: &sqlx::Error) -> bool {
    match error {
        sqlx::Error::Database(db_error) => db_error.code().as_deref() == Some("42P01"),
        _ => false,
    }
}
