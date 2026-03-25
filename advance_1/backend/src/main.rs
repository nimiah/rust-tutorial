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
        article::{create_article, get_homepage_articles, get_my_articles},
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
        crate::handlers::article::create_article,
        crate::handlers::article::get_homepage_articles,
        crate::handlers::article::get_my_articles,
        crate::handlers::auth::login
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Articles", description = "Article management endpoints")
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

#[tokio::main]
async fn main() {
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
        // CAP NHAT (bai 3): route cho he thong article.
        // - POST /api/articles: tao bai viet
        // - GET /api/articles/homepage: lay bai public cho homepage
        // - GET /api/articles/me: lay tat ca bai cua user dang login
        .route("/api/articles", post(create_article))
        .route("/api/articles/homepage", get(get_homepage_articles))
        .route("/api/articles/me", get(get_my_articles))
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

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("/api/user started at http://localhost:3000");

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
