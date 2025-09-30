use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> impl IntoResponse {
    "OK"
}