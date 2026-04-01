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
use dotenv::dotenv;
use utoipa::OpenApi;

use crate::{
    handlers::{
        article::get_all_articles,
        auth::login,
        user::{create_user, delete_user, edit_user, get_all_users, get_user_detail},
    },
    middleware::SecurityAddon,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user::create_user,
        crate::handlers::user::edit_user,
        crate::handlers::user::delete_user,
        crate::handlers::user::get_user_detail,
        crate::handlers::user::get_all_users,
        crate::handlers::article::get_all_articles,
        crate::handlers::auth::login
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints")
    ),
    info(
        title = "Demo API",
        version = "0.1.0",
        description = "REST API for user management",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv::from_filename("dev.env").ok();

    // Kết nối DB
    let pool = db::Db::new()
        .connect()
        .await
        .expect("Failed to connect to database");

    // ===== PUBLIC ROUTES =====
    let public_routes = Router::new()
        .route("/api/auth/login", post(login));

    // ===== PROTECTED ROUTES =====
    let protected_routes = Router::new()
        .route("/api/user", post(create_user))
        .route("/api/users/{id}", put(edit_user))
        .route("/api/users/{id}", get(get_user_detail))
        .route("/api/users/{id}", delete(delete_user))
        .route("/api/users", get(get_all_users))
        .route("/api/articles", get(get_all_articles))
        // middleware auth (KHÔNG cần state)
        .route_layer(from_fn(middleware::authentication));

    // ===== APP =====
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // middleware transaction (CÓ state)
        .route_layer(from_fn_with_state(
            pool.clone(),
            middleware::start_transaction,
        ))
        // swagger
        .merge(middleware::swagger_ui(ApiDoc::openapi()));

    // chạy server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running at http://localhost:3000");

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
        tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate()
        )
        .expect("Failed to install SIGTERM handler")
        .recv()
        .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Shutdown (Ctrl+C)");
        },
        _ = terminate => {
            println!("Shutdown (SIGTERM)");
        },
    }
}