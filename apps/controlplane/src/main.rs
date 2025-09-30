mod api;
mod models;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    // Initialize application state
    let state = AppState::new();

    // Build the API router with state
    let app = api::routes(state);

    // Bind to port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
