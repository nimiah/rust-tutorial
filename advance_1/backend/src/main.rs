mod db;
mod handlers;
mod middleware;
mod models;
mod services;

use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post, put},
    Router,
};
use utoipa::OpenApi;

use crate::{
    handlers::{
        auth::login,
        user::{create_user, delete_user, edit_user, get_all_users, get_user_detail},
    },
    middleware::SecurityAddon,
};

use std::sync::{Arc, Mutex};

struct AppConfig {
    server_port: u16,
}

impl AppConfig {
    fn from_env() -> Self {
        dotenv::dotenv().ok(); // Load dotenv
        Self {
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user::create_user,
        crate::handlers::user::edit_user,
        crate::handlers::user::delete_user,
        crate::handlers::user::get_user_detail,
        crate::handlers::user::get_all_users,
        crate::handlers::auth::login
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints")
    ),
    info(
        title = "Demo API",
        version = "0.1.0",
        description = "REST API for ser management",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

#[derive(Debug)]
enum UserRole {
    Admin(u32), // Tuple variant
    Guest,      // Unit variant
}

#[allow(unused_variables, dead_code)]
fn test() {
    // --- A. String và Vec (Owned) ---
    let name = String::from("Michael");
    let scores = vec![10, 20, 30, 40];

    // --- B. &str và &[T] (Slices) ---
    let name_slice = &name[0..4]; // "Mich"
    let scores_slice = &scores[1..3]; // [20, 30]

    // --- C. Enum (Option và Custom) ---
    let some_val = Some(100);
    let none_val: Option<i32> = None;
    let role_admin = UserRole::Admin(99);
    let role_guest = UserRole::Guest;

    // --- D. Smart Pointers (Arc & Mutex) ---
    // Đây chính là cấu trúc "bom nổ chậm" trong db.rs của bạn
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));

    println!("Đặt breakpoint ở dòng này và bắt đầu quan sát!"); // <--- BREAKPOINT HERE
}

#[tokio::main]
async fn main() {
    test(); // Chạy hàm test để quan sát các kiểu dữ liệu

    // 1. Load config tập trung
    let config = AppConfig::from_env();

    // 2. Truyền config vào DB Pool
    // create connection pool
    let pool = db::Db::new()
        .connect()
        .await
        .expect("Failed to connect to database");

    // build our application with a single route
    let app = Router::new()
        // user routers
        .route("/api/user", post(create_user))
        .route("/api/users/{id}", put(edit_user))
        .route("/api/users/{id}", get(get_user_detail))
        .route("/api/users/{id}", delete(delete_user))
        .route("/api/users", get(get_all_users))
        // auth routers
        .route("/api/auth/login", post(login))
        // middleware
        .route_layer(from_fn(middleware::authentication))
        .route_layer(from_fn_with_state(
            pool.clone(),
            middleware::start_transaction,
        ))
        // swagger - openapi
        .merge(middleware::swagger_ui(ApiDoc::openapi()));

    // 3. Sử dụng Port từ config
    // run our app with hyper, listening globally on port 3000
    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("/api/user started at {}", &addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Cannot start http server");
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Received Ctrl+C signal, starting graceful shutdown...");
        },
        _ = terminate => {
            println!("Received SIGTERM signal, starting graceful shutdown...");
        },
    }
}
