use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::store::SharedStore;

#[derive(Deserialize)]
pub struct KV {
    key: String,
    value: String,
}

async fn set_kv(
    Extension(store): Extension<Arc<SharedStore>>,
    Json(kv): Json<KV>,
) -> impl IntoResponse {
    store.insert(kv.key, kv.value).await;
    (StatusCode::OK, "Stored")
}

async fn get_kv(
    Extension(store): Extension<Arc<SharedStore>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    match store.get(&key).await {
        Some(val) => (StatusCode::OK, val),
        None => (StatusCode::NOT_FOUND, "Key not found".into()),
    }
}

pub async fn start_leader_server() {
    let store = Arc::new(SharedStore::new());

    let app = Router::new()
        .route("/kv", post(set_kv))
        .route("/kv/:key", get(get_kv))
        .layer(Extension(store));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🟢 Leader running on http://127.0.0.1:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

