use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::controllers::timer::{AppError, AppState};
use crate::services::geocoding::GeocodingService;

#[derive(Debug, Deserialize)]
pub struct GeocodingQuery {
    q: String,
}

/// Lookup city location data (lat, lon, timezone) from city name
pub async fn lookup_city(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GeocodingQuery>,
) -> Result<impl IntoResponse, AppError> {
    let api_key = state
        .config
        .weather_api_key
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Weather API key not configured")))?;

    let result = GeocodingService::lookup_city(&query.q, api_key).await?;
    Ok(Json(result))
}
