use crate::{
    db::DbTransaction,
    models::user::{RequestUser, User},
};

pub struct UserRepository {
    tx: DbTransaction,
}

impl UserRepository {
    pub fn new(tx: DbTransaction) -> Self {
        UserRepository { tx }
    }

    pub async fn create(&self, user: RequestUser) -> Result<i32, sqlx::Error> {
        let mut db = self.tx.lock().await;

        // CAP NHAT (bai 1): khi tao user moi, ghi them phone vao bang users_demo.
        let row = sqlx::query!(
            "INSERT INTO users_demo (name, email, phone) VALUES ($1, $2, $3) RETURNING id",
            user.name,
            user.email,
            user.phone
        )
        .fetch_one(&mut *db.as_mut())
        .await?;

        // return user_id
        Ok(row.id)
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

    pub async fn update(&self, id: i32, updated: RequestUser) -> Result<(), sqlx::Error> {
        let mut db = self.tx.lock().await;

        // CAP NHAT (bai 1): khi update user, dong bo luon gia tri phone.
        let ret = sqlx::query("UPDATE users_demo SET email = $1, name = $2, phone = $3 WHERE id = $4")
            .bind(updated.email)
            .bind(updated.name)
            .bind(updated.phone)
            .bind(id)
            .execute(&mut *db.as_mut())
            .await?;

        if ret.rows_affected() == 1 {
            return Ok(());
        }
        Err(sqlx::Error::RowNotFound)
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let mut db = self.tx.lock().await;

        // CAP NHAT (bai 2): tra ve Result that su de phan biet ro
        // - Ok(vec![]) => bang khong co du lieu
        // - Err(...) => query/database dang bi loi
        sqlx::query_as::<_, User>("SELECT * FROM users_demo")
            .fetch_all(&mut *db.as_mut())
            .await
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
