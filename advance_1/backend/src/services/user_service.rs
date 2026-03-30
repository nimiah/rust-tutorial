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
        let result = self.user_repo.get_by_id(id).await;

        match result {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, String> {
        // CAP NHAT (bai 2): giu nguyen loi tu repository day len handler,
        // khong duoc bien loi thanh danh sach rong.
        self.user_repo.get_all().await.map_err(|e| e.to_string())
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

    pub async fn login(&self, req_login: RequestLogin) -> Result<String, String> {
        let user = self
            .user_repo
            .get_by_name(req_login.name)
            .await
            .map_err(|e| e.to_string())?;
        if user.password != Some(req_login.password) {
            return Err(String::from("Password does not match"));
        }

        // generate token
        let claims = Claims {
            uid: user.id,
            exp: (Utc::now() + Duration::minutes(30)).timestamp(),
            iat: Utc::now().timestamp(),
        };
        Tokenizer::new().generate(claims)
    }
}
