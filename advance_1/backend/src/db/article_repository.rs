use crate::{
    db::DbTransaction,
    models::article::{Article, RequestArticle},
};

pub struct ArticleRepository {
    tx: DbTransaction,
}

impl ArticleRepository {
    pub fn new(tx: DbTransaction) -> Self {
        Self { tx }
    }

    pub async fn create(
        &self,
        article: RequestArticle,
        created_by_user: i32,
    ) -> Result<i32, sqlx::Error> {
        let mut db = self.tx.lock().await;

        // CAP NHAT (bai 3): tao bai viet moi, gan nguoi tao bang created_by_user.
        sqlx::query_scalar::<_, i32>(
            "INSERT INTO articles_demo (title, content, visibility, created_by_user) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(article.title)
        .bind(article.content)
        .bind(article.visibility)
        .bind(created_by_user)
        .fetch_one(&mut *db.as_mut())
        .await
    }

    pub async fn get_public_articles(&self) -> Result<Vec<Article>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        // CAP NHAT (bai 3): phuc vu homepage, chi lay bai public.
        sqlx::query_as::<_, Article>(
            "SELECT id, title, content, time_created, visibility, created_by_user
             FROM articles_demo
             WHERE visibility = 'public'
             ORDER BY time_created DESC",
        )
        .fetch_all(&mut *db.as_mut())
        .await
    }

    pub async fn get_articles_by_user(&self, user_id: i32) -> Result<Vec<Article>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        // CAP NHAT (bai 3): neu user da login thi xem duoc toan bo bai cua chinh minh.
        sqlx::query_as::<_, Article>(
            "SELECT id, title, content, time_created, visibility, created_by_user
             FROM articles_demo
             WHERE created_by_user = $1
             ORDER BY time_created DESC",
        )
        .bind(user_id)
        .fetch_all(&mut *db.as_mut())
        .await
    }
}
