use crate::{
    db::DbTransaction,
    models::user::{RequestUser, RequestUserUpdate, User},
};
use sqlx::Row;

pub struct UserRepository {
    tx: DbTransaction,
}

impl UserRepository {
    pub fn new(tx: DbTransaction) -> Self {
        UserRepository { tx }
    }

    pub async fn create(&self, user: RequestUser) -> Result<i32, sqlx::Error> {
        let mut db = self.tx.lock().await;

        // execute sql to insert user to user table
        let row = sqlx::query("INSERT INTO users_demo (name, email) VALUES ($1, $2) RETURNING id")
            .bind(&user.name)
            .bind(&user.email)
        .fetch_one(&mut *db.as_mut())
        .await?;
        
        let id: i32 = row.try_get("id")?;

        // return user_id
        Ok(id)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let user = sqlx::query_as::<_, User>("SELECT * FROM users_demo WHERE id = $1")
            .bind(id)
            .fetch_one(&mut *db.as_mut())
            .await?;

        Ok(user)
    }

    pub async fn get_by_name(&self, name: String) -> Result<User, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let user = sqlx::query_as::<_, User>("SELECT * FROM users_demo WHERE name = $1")
            .bind(name)
            .fetch_one(&mut *db.as_mut())
            .await?;

        Ok(user)
    }

    pub async fn update(&self, id: i32, updated: RequestUserUpdate) -> Result<(), sqlx::Error> {
        let mut db = self.tx.lock().await;

        let ret = sqlx::query("UPDATE users_demo SET email = $1, name = $2 WHERE id = $3")
            .bind(updated.email)
            .bind(updated.name)
            .bind(id)
            .execute(&mut *db.as_mut())
            .await?;

        if ret.rows_affected() == 1 {
            return Ok(());
        }
        Err(sqlx::Error::RowNotFound)
    }

    pub async fn get_all(&self) -> Option<Vec<User>> {
        let mut db = self.tx.lock().await;

        let result = sqlx::query_as::<_, User>("SELECT * FROM users_demo")
            .fetch_all(&mut *db.as_mut())
            .await;

        match result {
            Err(_) => None,
            Ok(users) => Some(users),
        }
    }

    pub async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut db = self.tx.lock().await;

        _ = sqlx::query("DELETE FROM users_demo WHERE id = $1")
            .bind(id)
            .execute(&mut *db.as_mut())
            .await?;

        Ok(())
    }
}
