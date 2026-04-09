use crate::{
    db::DbTransaction,
    models::user::{RequestUser, User},
};
use sqlx::Row;

pub struct UserRepository {
    tx: DbTransaction,
}

impl UserRepository {
    pub fn new(tx: DbTransaction) -> Self {
        UserRepository { tx }
    }

    // ================= CREATE =================
    pub async fn create(&self, user: RequestUser) -> Result<i32, sqlx::Error> {
        let mut db = self.tx.lock().await;

<<<<<<< HEAD
        let row = sqlx::query(
            "INSERT INTO users_demo (name, email) VALUES ($1, $2) RETURNING id"
        )
        .bind(&user.name)
        .bind(&user.email)
        .fetch_one(&mut *db.as_mut())
        .await?;
=======
        // execute sql to insert user to user table
        let row = sqlx::query("INSERT INTO users_demo (name, email) VALUES ($1, $2) RETURNING id")
            .bind(&user.name)
            .bind(&user.email)
            .fetch_one(&mut *db.as_mut())
            .await?;

        let id: i32 = row.try_get("id")?;
>>>>>>> main

        let id: i32 = row.try_get("id")?;
        Ok(id)
    }

    // ================= GET BY ID =================
    pub async fn get_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, age, password FROM users_demo WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&mut *db.as_mut())
        .await?;

        Ok(user)
    }

    // ================= GET BY NAME =================
<<<<<<< HEAD
    pub async fn get_by_name(&self, name: String) -> Result<User, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, age, password FROM users_demo WHERE name = $1"
        )
        .bind(name)
        .fetch_one(&mut *db.as_mut())
        .await?;
=======
    pub async fn get_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let user = sqlx::query_as::<_, User>("SELECT * FROM users_demo WHERE email = $1")
            .bind(email)
            .fetch_one(&mut *db.as_mut())
            .await?;
>>>>>>> main

        Ok(user)
    }

<<<<<<< HEAD
    // ================= GET BY EMAIL (LOGIN) =================
    pub async fn get_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let mut db = self.tx.lock().await;

        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, age, password FROM users_demo WHERE email = $1"
        )
        .bind(email)
        .fetch_one(&mut *db.as_mut())
        .await?;

        Ok(user)
    }

=======
>>>>>>> main
    // ================= UPDATE =================
    pub async fn update(&self, id: i32, updated: RequestUser) -> Result<(), sqlx::Error> {
        let mut db = self.tx.lock().await;

<<<<<<< HEAD
        let ret = sqlx::query(
            "UPDATE users_demo SET name = $1, email = $2 WHERE id = $3"
        )
        .bind(updated.name)
        .bind(updated.email)
        .bind(id)
        .execute(&mut *db.as_mut())
        .await?;
=======
        let ret = sqlx::query("UPDATE users_demo SET name = $1, email = $2 WHERE id = $3")
            .bind(updated.name)
            .bind(updated.email)
            .bind(id)
            .execute(&mut *db.as_mut())
            .await?;
>>>>>>> main

        if ret.rows_affected() == 1 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    // ================= GET ALL =================
    pub async fn get_all(&self) -> Option<Vec<User>> {
        let mut db = self.tx.lock().await;

        let result = sqlx::query_as::<_, User>(
            "SELECT id, name, email, age, password FROM users_demo"
        )
        .fetch_all(&mut *db.as_mut())
        .await;

        match result {
            Ok(users) => Some(users),
            Err(_) => None,
        }
    }

    // ================= DELETE =================
    pub async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut db = self.tx.lock().await;

        sqlx::query("DELETE FROM users_demo WHERE id = $1")
            .bind(id)
            .execute(&mut *db.as_mut())
            .await?;

        Ok(())
    }
}