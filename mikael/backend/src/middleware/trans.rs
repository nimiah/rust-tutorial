use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use tokio::sync::Mutex;

pub async fn start_transaction(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Response {
    // Begin transaction
    let tx = pool
        .begin()
        .await
        .expect("Database transaction cannot start");
    println!("Started transaction");

    let tx = Arc::new(Mutex::new(tx));

    // Inject into request extensions
    req.extensions_mut().insert(tx.clone());

    // Call the next handler
    let response = next.run(req).await;

    // Commit or rollback based on response status
    let tx = Arc::try_unwrap(tx)
        .expect("Cannot unwrap transaction")
        .into_inner();

    if response.status().is_success() {
        tx.commit().await.expect("Cannot commit transaction");
        println!("Commmited transaction");
    } else {
        _ = tx.rollback().await.expect("Cannot rolback transaction");
        println!("Rollback transaction (status: {:#?})", response.status());
    }

    response
}
