use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
    pub timezone: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub timezone: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct LocationTimeResponse {
    pub location: Location,
    pub current_time: DateTime<Utc>,
    pub formatted_time: String,
}
