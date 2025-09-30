pub mod timers;

use axum::{response::IntoResponse, routing::get, Router};
use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/timers", timers::routes())
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    "OK"
}