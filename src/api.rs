use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use crate::store::{SharedStore, KV};

pub fn routes(store: Arc<SharedStore>) -> Router {
    let store_post = store.clone();
    let store_get = store.clone();

    Router::new()
        .route(
            "/",
            post(move |Json(kv): Json<KV>| {
                let store = store_post.clone();
                async move {
                    store.insert(kv.key.clone(), kv.value.clone());
                    Json(format!("Inserted key: {}", kv.key))
                }
            }),
        )
        .route(
            "/:key",
            get(move |Path(key): Path<String>| {
                let store = store_get.clone();
                async move {
                    let val = store.get(&key);
                    Json(val)
                }
            }),
        )
}

