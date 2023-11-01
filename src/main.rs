mod database;
mod handlers;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let db = database::Database::new().await.unwrap();
    let app = Router::new()
        .route("/", get(handlers::random))
        .route("/character", get(handlers::character)).with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
