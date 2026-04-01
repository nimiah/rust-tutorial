use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use crate::{
    db::DbTransaction,
    services::{tokenizer::Tokenizer, user_service::UserService},
};

pub async fn authentication(mut req: Request, next: Next) -> Response {
    println!("Authentication middleware layer reached");

    // Cho phép swagger đi qua
    let path = req.uri().path();
    if path.starts_with("/swagger-ui") || path.starts_with("/api-docs") || path == "/openapi.json" {
        return next.run(req).await;
    }

    // 1) Lấy Authorization header
    let auth_header = match req.headers().get(AUTHORIZATION) {
        //Some(v) => v,
        Some(v) => {
            println!("👉 RAW HEADER: {:?}", v);
    v
},
        None => {
            return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response();
        }
    };

    // 2) Chuyển header sang string
    let auth_str = match auth_header.to_str() {
        //Ok(v) => v,
        Ok(v) => {
             println!("👉 AUTH STRING: {}", v);
    v
},
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, "Invalid Authorization header").into_response();
        }
    };

    // 3) Tách Bearer token
    let token = match auth_str.strip_prefix("Bearer ") {
        Some(t) => {
            println!("🔥 TOKEN NHẬN ĐƯỢC: {}", t);
            t
        }
            
        None => {
            return (StatusCode::UNAUTHORIZED, "Invalid Bearer token format").into_response();
        }
    };

    let tokenizer = Tokenizer::new();

    // 4) Verify token
    let claims = match tokenizer.verify(token) {
        Ok(c) => c,
        Err(e) => {
            println!("JWT verification failed: {}", e);
            return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
    };

    // 5) Check token hết hạn
    if claims.exp <= Utc::now().timestamp() {
        return (StatusCode::UNAUTHORIZED, "Token expired").into_response();
    }

    // 6) Lấy DbTransaction từ request extensions
    let tx = match req.extensions().get::<DbTransaction>() {
        Some(tx) => tx.clone(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database transaction not found",
            )
                .into_response();
        }
    };

    // 7) Lấy user từ database theo uid trong token
    let user = match UserService::new(tx).get_user(claims.uid).await {
        Some(user) => user,
        None => {
            return (StatusCode::UNAUTHORIZED, "User not found").into_response();
        }
    };

    // 8) NHÉT user vào request để handler dùng được Extension<User>
    req.extensions_mut().insert(user);

    // 9) Cho request đi tiếp
    next.run(req).await
}