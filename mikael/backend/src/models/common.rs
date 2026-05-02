use std::fmt::Debug;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type ApiResult<T> = (StatusCode, Response<T>);

#[derive(Deserialize, Debug, Serialize, ToSchema)]
pub struct Response<T: Serialize> {
    pub message: String,
    pub value: Option<T>,
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl<T: Serialize> Response<T> {
    pub fn ok(value: T) -> ApiResult<T> {
        (
            StatusCode::OK,
            Response {
                message: String::from("Success"),
                value: Some(value),
            },
        )
    }

    pub fn err(status_code: StatusCode, message: String) -> ApiResult<T> {
        (
            status_code,
            Response {
                message,
                value: None,
            },
        )
    }

    pub fn from_result<E>(result: Result<T, E>) -> ApiResult<T>
    where
        E: Debug,
    {
        match result {
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Response {
                    message: format!("Error: {:?}", e),
                    value: None,
                },
            ),
            Ok(v) => (
                StatusCode::OK,
                Response {
                    value: Some(v),
                    message: String::from("Success"),
                },
            ),
        }
    }

    pub fn from_optional(option: Option<T>) -> ApiResult<T> {
        match option {
            Some(v) => (
                StatusCode::OK,
                Response {
                    value: Some(v),
                    message: String::from("OK"),
                },
            ),
            None => (
                StatusCode::NOT_FOUND,
                Response {
                    value: None,
                    message: String::from("Not found"),
                },
            ),
        }
    }
}
