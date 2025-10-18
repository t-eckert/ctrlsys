use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ManagedDatabase {
    pub id: Uuid,
    pub db_name: String,
    pub created_at: DateTime<Utc>,
    pub owner: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDatabaseRequest {
    pub db_name: String,
    pub owner: Option<String>,
    pub notes: Option<String>,
}
