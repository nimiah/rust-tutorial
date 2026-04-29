use axum::{
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
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

    // Skip JWT verification for Swagger/OpenAPI documentation paths
    let path = req.uri().path();
    if path.starts_with("/swagger-ui") || path.starts_with("/api-docs") || path == "/openapi.json" {
        println!("Skipping JWT verification for Swagger path: {}", path);
        return next.run(req).await;
    }

    // Try to extract Authorization header
    let auth_header = req.headers().get(AUTHORIZATION);
    println!("Bearer Authentication {:#?}", auth_header);

    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            // Extract Bearer token
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let tokenizer = Tokenizer::new();

                // Verify token
                match tokenizer.verify(token) {
                    Ok(claims) => {
                        // Check if token is expired
                        if claims.exp <= Utc::now().timestamp() {
                            println!("Token expired");
                            return (StatusCode::UNAUTHORIZED, "Token expired".to_string())
                                .into_response();
                        }

                        if let Some(tx) = req.extensions().get::<DbTransaction>() {
                            if let Some(user) =
                                UserService::new(tx.clone()).get_user(claims.uid).await
                            {
                                // get user identifier
                                req.extensions_mut().insert(user);
                            }
                        }

                        println!("JWT token verified successfully");
                    }
                    Err(e) => {
                        println!("JWT verification failed: {}", e);
                        // Don't return error - let the handler decide if auth is required
                        // Routes with AuthUser will fail, routes with PublicRoute will succeed
                    }
                }
            }
        }
    }

    // Continue to next handler
    next.run(req).await
}
