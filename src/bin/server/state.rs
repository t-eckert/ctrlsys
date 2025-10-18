use ctrlsys::config::ServerConfig;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: ServerConfig,
}
