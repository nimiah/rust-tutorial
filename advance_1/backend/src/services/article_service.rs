use crate::{
    db::{DbTransaction, article_repository::ArticleRepository},
    models::article::Article,
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

    pub async fn create(&self, owner_id: i32, article: RequestArticle) -> Result<i32, sqlx::Error> {
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
            "#, // 🔧 FIX: owner_id thay vì user_id theo schema DB
                // 🔧 FIX: thêm description
                // 🔧 FIX: bỏ views, likes — DB tự set default 0
        )
        .bind(owner_id)
        .bind(&article.title)
        .bind(&article.body)
        .bind(&article.description) // 🔧 ADD: bind description
        .bind(visibility_str) // 🔧 FIX: bind &str thay vì enum trực tiếp
        .fetch_one(&mut *db.as_mut())
        .await?;

        let id: i32 = row.try_get("id")?;
        Ok(id)
    }
    pub async fn get_all(&self, user_id: i32) -> Result<Vec<String>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let rows = sqlx::query(
            r#"
        SELECT title
        FROM articles
        WHERE owner_id = $1 OR visibility = 'public'
        ORDER BY time_created DESC
        "#,
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

    pub async fn create_article(
        &self,
        user_id: i32,
        payload: RequestArticle,
    ) -> Result<i32, String> {
        // ================= VALIDATION =================

        if payload.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        // 🔧 FIX: xoá validate visibility bằng string
        // Visibility là enum — serde tự reject JSON không hợp lệ khi deserialize
        // Không thể và không cần so sánh enum với &str

        // ================= CALL REPOSITORY =================

        let result = self.article_repo.create(user_id, payload).await;

        match result {
            Ok(id) => Ok(id),
            Err(_) => Err("Failed to create article".to_string()),
        }
    }

    pub async fn get_all(&self, user_id: i32) -> Result<Vec<String>, String> {
        let result = self.article_repo.get_all(user_id).await;

        match result {
            Ok(data) => Ok(data),
            Err(_) => Err("Failed to get articles".to_string()),
        }
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
