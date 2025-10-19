use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod background;
mod state;

use lib::{config::ServerConfig, db};
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
        db: pool.clone(),
        config: config.clone(),
    });

    // Start background tasks
    tokio::spawn(background::timer_expiration_checker(pool.clone()));
    tracing::info!("Background tasks started");

    // Build the application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        // Timer routes (protected)
        .nest("/api/v1/timers", timer_routes())
        // Location routes (protected)
        .nest("/api/v1/locations", location_routes())
        // Weather routes (protected)
        .nest("/api/v1/weather", weather_routes())
        // Geocoding routes (protected)
        .nest("/api/v1/geocoding", geocoding_routes())
        // Database management routes (protected)
        .nest("/api/v1/databases", database_routes())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ))
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

fn timer_routes() -> Router<Arc<AppState>> {
    use lib::controllers::timer;
    use lib::ws::timer::timer_ws_handler;

    Router::new()
        .route("/", post(timer::create_timer))
        .route("/", get(timer::list_timers))
        .route("/{id}", get(timer::get_timer))
        .route("/{id}", delete(timer::cancel_timer))
        .route("/{id}/ws", get(timer_ws_handler))
}

fn location_routes() -> Router<Arc<AppState>> {
    use lib::controllers::location;

    Router::new()
        .route("/", post(location::create_location))
        .route("/", get(location::list_locations))
        .route("/{id}", get(location::get_location))
        .route("/{id}", delete(location::delete_location))
        .route("/{id}/time", get(location::get_location_time))
        .route("/times", get(location::list_location_times))
}

fn weather_routes() -> Router<Arc<AppState>> {
    use lib::controllers::weather;

    Router::new()
        .route("/locations/{id}", get(weather::get_weather_for_location))
        .route("/locations", get(weather::get_weather_for_all_locations))
}

fn geocoding_routes() -> Router<Arc<AppState>> {
    use lib::controllers::geocoding;

    Router::new()
        .route("/lookup", get(geocoding::lookup_city))
}

fn database_routes() -> Router<Arc<AppState>> {
    use lib::controllers::database;

    Router::new()
        .route("/", post(database::create_database))
        .route("/", get(database::list_databases))
        .route("/{name}", get(database::get_database))
        .route("/{name}", delete(database::drop_database))
        .route("/{name}/exists", get(database::check_database_exists))
}
