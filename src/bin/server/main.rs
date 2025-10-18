use axum::{
    Router,
    routing::get,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod state;

use ctrlsys::{config::ServerConfig, db};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ctrlsys=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = ServerConfig::load()?;
    tracing::info!("Server configuration loaded");

    // Connect to database
    let pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Run migrations
    db::run_migrations(&pool).await?;
    tracing::info!("Database migrations completed");

    // Create application state
    let state = Arc::new(AppState {
        db: pool,
        config: config.clone(),
    });

    // Build the application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(state);

    // Start the server
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

