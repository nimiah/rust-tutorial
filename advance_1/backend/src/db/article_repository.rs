// db/article_repository.rs
use crate::{
    db::DbTransaction,
    models::article::{Article, ArticleVisibility},
};

pub struct ArticleRepository {
    tx: DbTransaction,
}

impl ArticleRepository {
    pub fn new(tx: DbTransaction) -> Self {
        ArticleRepository { tx }
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

        let sql =
            "SELECT * FROM articles_demo WHERE visibility = ANY($1) ORDER BY time_created DESC";
        println!(
            " === db - smt: {:?} - allowed_visibilities: {:?}",
            sql, &allowed_visibilities
        );

        let result = sqlx::query_as::<_, Article>(sql)
            .bind(&allowed_visibilities)
            .fetch_all(&mut *db.as_mut())
            .await;

        result.ok()
    }
}
