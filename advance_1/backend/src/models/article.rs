use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, Validate)]
pub struct RequestArticle {
    // CAP NHAT (bai 3): tieu de bai viet khi tao article.
    #[validate(length(min = 2, max = 150))]
    pub title: String,
    // CAP NHAT (bai 3): noi dung bai viet.
    #[validate(length(min = 2))]
    pub content: String,
    // CAP NHAT (bai 3): chi cho phep 3 muc hien thi theo de bai.
    pub visibility: String,
}

impl RequestArticle {
    pub fn validate_visibility(&self) -> Result<(), String> {
        match self.visibility.as_str() {
            "public" | "unlisted" | "draft" => Ok(()),
            _ => Err(String::from(
                "visibility must be one of: public, unlisted, draft",
            )),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    // CAP NHAT (bai 3): thoi diem tao bai viet.
    pub time_created: DateTime<Utc>,
    // CAP NHAT (bai 3): public / unlisted / draft.
    pub visibility: String,
    // CAP NHAT (bai 3): user nao tao bai viet nay.
    pub created_by_user: i32,
}
