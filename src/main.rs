mod api;
mod store;

use axum::{serve, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber;
use store::SharedStore;
use api::routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let store = Arc::new(SharedStore::default());
    let app = Router::new().nest("/kv", routes(store));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

