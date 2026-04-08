// main.rs
mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod services;

use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post, put},
};
use utoipa::OpenApi;

use crate::{
    handlers::{
        article::get_articles,
        auth::login,
        user::{create_user, delete_user, edit_user, get_all_users, get_user_detail},
    },
    middleware::SecurityAddon,
};

use config::AppConfig;

// ─── OpenAPI Documentation ───────────────────────────────────────────────────

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user::create_user,
        crate::handlers::user::edit_user,
        crate::handlers::user::delete_user,
        crate::handlers::user::get_user_detail,
        crate::handlers::user::get_all_users,
        crate::handlers::article::get_articles,
        crate::handlers::auth::login
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Articles", description = "Article management endpoints"),
    ),
    info(
        title       = "Demo API",
        version     = "0.1.0",
        description = "REST API for ser management",
        contact(
            name  = "API Support",
            email = "support@example.com"
        )
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

// ─── Router ──────────────────────────────────────────────────────────────────

fn app_router(pool: sqlx::PgPool) -> Router {
    let user_routes = Router::new()
        // user routers
        .route("/api/user", post(create_user))
        .route("/api/users", get(get_all_users))
        .route("/api/users/{id}", put(edit_user))
        .route("/api/users/{id}", get(get_user_detail))
        .route("/api/users/{id}", delete(delete_user));

    let article_routes = Router::new().route("/api/articles", get(get_articles));

    let auth_routes = Router::new()
        // auth routers
        .route("/api/auth/login", post(login));

    Router::new()
        .merge(user_routes)
        .merge(article_routes)
        .merge(auth_routes)
        // middleware
        .route_layer(from_fn(middleware::authentication))
        .route_layer(from_fn_with_state(pool, middleware::start_transaction))
        // swagger - openapi
        .merge(middleware::swagger_ui(ApiDoc::openapi()))
}

// ─── Entry point ─────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    // 1. Nạp biến môi trường (Ưu tiên dev.env)
    AppConfig::init();

    // 2. Create connection pool
    let pool = db::Db::new()
        .connect()
        .await
        .expect("Failed to connect to database");

    // 3. Run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    println!("/api/user started at http://localhost:3000/api/users");
    println!("Swagger UI started at http://localhost:3000/swagger-ui/");

    // 4. Listen
    axum::serve(listener, app_router(pool))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Cannot start http server");
}

// ─── Graceful shutdown ───────────────────────────────────────────────────────

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
