use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber;

use secure_notes::{database, routes, state};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = database::connect()
        .await
        .expect("Failed to connect to database");

    let state = state::AppState { db };

    let app = Router::new()
        .merge(routes::router())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}