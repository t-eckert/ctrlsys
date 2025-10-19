use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::controllers::timer::{AppError, AppState};
use crate::services::weather::WeatherService;

/// Get weather for a specific location
pub async fn get_weather_for_location(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let api_key = state
        .config
        .weather_api_key
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Weather API key not configured")))?;

    let weather = WeatherService::get_for_location(&state.db, id, api_key).await?;
    Ok(Json(weather))
}

/// Get weather for all locations
pub async fn get_weather_for_all_locations(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let api_key = state
        .config
        .weather_api_key
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Weather API key not configured")))?;

    let weather_list = WeatherService::get_for_all_locations(&state.db, api_key).await?;
    Ok(Json(weather_list))
}
