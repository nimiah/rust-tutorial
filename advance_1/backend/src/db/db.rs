use std::sync::Arc;

use sqlx::{Error, Pool, Postgres, Transaction, postgres::PgPoolOptions};
use tokio::sync::Mutex;

use crate::config::AppConfig;

pub type DbTransaction = Arc<Mutex<Transaction<'static, Postgres>>>;

pub struct Db {
    url: String,
    max_connection: u32,
}

impl Db {
    pub fn new() -> Self {
        let config = AppConfig::get();

        Db {
            url: config.database_url.clone(),
            max_connection: 3,
        }
    }

    pub async fn connect(&self) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connection)
            .connect(&self.url)
            .await
    }
}
