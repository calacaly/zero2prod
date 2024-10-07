use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

pub fn new() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(Health::default()))
}

#[derive(Debug, Serialize, Clone)]
struct Health {
    code: usize,
    msg: String,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            code: 200,
            msg: "health ok!".to_string(),
        }
    }
}
