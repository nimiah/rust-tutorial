use chrono::{Duration, Utc};

use crate::{
    db::{DbTransaction, user_repository::UserRepository},
    models::{
        auth::{Claims, RequestLogin},
        user::{RequestUser, User},
    },
    services::tokenizer::Tokenizer,
};

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(tx: DbTransaction) -> Self {
        UserService {
            user_repo: UserRepository::new(tx),
        }
    }

    pub async fn create_user(&self, user: RequestUser) -> Result<i32, String> {
        self.user_repo.create(user).await.map_err(|e| e.to_string())
    }

    pub async fn get_user(&self, id: i32) -> Option<User> {
        match self.user_repo.get_by_id(id).await {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn get_all_users(&self) -> Option<Vec<User>> {
        self.user_repo.get_all().await
    }

    pub async fn update_user(&self, id: i32, updated: RequestUser) -> Result<(), String> {
        self.user_repo
            .update(id, updated)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), String> {
        self.user_repo.delete(id).await.map_err(|e| e.to_string())
    }

    // ================= LOGIN =================
    pub async fn login(&self, req_login: RequestLogin) -> Result<String, String> {
        // 1. tìm user theo email
        let user = self
            .user_repo
            .get_by_email(req_login.email)
            .await
            .map_err(|e| e.to_string())?;
        if user.password_hash != req_login.password {
            return Err(String::from("Password does not match"));
        }

        // 3. tạo token
        let claims = Claims {
            uid: user.id,
            exp: (Utc::now() + Duration::minutes(30)).timestamp(),
            iat: Utc::now().timestamp(),
        };

        Tokenizer::new().generate(claims)
    }
}
