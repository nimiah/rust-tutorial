use crate::{
    db::DbTransaction,
    models::article::{Article, ArticleVisibility},
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

    pub async fn get_articles(
        &self,
        allowed_visibilities: Vec<ArticleVisibility>,
    ) -> Option<Vec<Article>> {
        let mut db = self.tx.lock().await;

        // let smt = sqlx::query_as::<_, Article>("SELECT * FROM articles_demo")
        //     .fetch_all(&mut *db.as_mut())
        //     .await;
        // println!(" *** Test smt: {:?}", smt);

        // let sql = "SELECT * FROM articles_demo WHERE visibility = ANY($1) ORDER BY time_created DESC";
        // let sql = "SELECT id, owner_id, title, visibility::TEXT as visibility, views, likes, time_created
        let sql = "SELECT *, visibility::TEXT as visibility
            FROM articles_demo WHERE visibility = ANY($1) ORDER BY time_created DESC";

        println!(
            " === db - smt: {:?} - allowed_visibilities: {:?}",
            sql, &allowed_visibilities
        );

        let result = sqlx::query_as::<_, Article>(sql)
            .bind(&allowed_visibilities)
            .fetch_all(&mut *db.as_mut())
            .await;

        // result.ok()
        match result {
            Ok(data) => Some(data),
            Err(e) => {
                println!("🛑 THỰC SỰ CÓ LỖI DB: {:?}", e);
                None
            }
        }
    }
}
