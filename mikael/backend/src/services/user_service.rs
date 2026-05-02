use chrono::{Duration, Utc};

use crate::{
    db::{DbTransaction, user_repository::UserRepository},  models::{
        auth::{Claims, LoggedInUser, RequestLogin},
        user::{CreateUserRequest, RequestUser, User},
    }, services::{pass_hash::PasswordUtil, tokenizer::Tokenizer}
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

    pub async fn create_user(&self, user: CreateUserRequest) -> Result<i32, String> {
        let (password_hash, password_salt) = PasswordUtil::hash_password(&user.password)
            .map_err(|e: Box<dyn std::error::Error>| e.to_string())?;
        self.user_repo.create(user, password_hash, password_salt).await.map_err(|e| e.to_string())
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
    pub async fn login(&self, req_login: RequestLogin) -> Result<LoggedInUser, String> {
        let user: User = self
            .user_repo
            .get_by_email(req_login.email)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;
        
        // Verify password using hash and salt
        let is_valid = PasswordUtil::verify_password(
            &req_login.password,
            &user.password_hash,
            &user.password_salt
        ).map_err(|e: Box<dyn std::error::Error>| e.to_string())?;
        
        if !is_valid {
            return Err(String::from("Password does not match"));
        }

        // 3. tạo token
        let claims = Claims {
            uid: user.id.clone(),
            exp: (Utc::now() + Duration::minutes(30)).timestamp(),
            iat: Utc::now().timestamp(),
        };

        Tokenizer::new()
            .generate(claims)
            .map(|token| LoggedInUser{ email: user.email, name: user.name, token})

    }
}
