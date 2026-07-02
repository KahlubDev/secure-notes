mod database;
mod routes;

use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db = match database::connect().await {
    Ok(db) => db,
    Err(e) => {
        eprintln!("Database error: {e}");
        return;
    }
};
    let app = Router::new().merge(routes::router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}