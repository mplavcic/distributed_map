use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize)]
pub struct KV {
    key: String,
    value: String,
}

const LEADER_URL: &str = "http://127.0.0.1:3000";

async fn proxy_post(Json(kv): Json<KV>) -> impl IntoResponse {
    let client = reqwest::Client::new();
    match client
        .post(format!("{LEADER_URL}/kv"))
        .json(&kv)
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            (status, text)
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            format!("Forwarding error: {}", e),
        ),
    }
}

async fn proxy_get(Path(key): Path<String>) -> impl IntoResponse {
    match reqwest::get(format!("{LEADER_URL}/kv/{key}")).await {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            (status, text)
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            format!("Forwarding error: {}", e),
        ),
    }
}

pub async fn start_proxy(port: u16) {
    let app = Router::new()
        .route("/kv", post(proxy_post))
        .route("/kv/:key", get(proxy_get));

    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    println!("🛰️  Proxy running on http://127.0.0.1:{port}");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

