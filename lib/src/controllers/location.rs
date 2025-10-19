use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::controllers::timer::AppError;
use crate::controllers::timer::AppState;
use crate::models::location::CreateLocationRequest;
use crate::services::location::LocationService;

/// Create a new location
pub async fn create_location(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateLocationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let location = LocationService::create(&state.db, req).await?;
    Ok((StatusCode::CREATED, Json(location)))
}

/// Get a location by ID
pub async fn get_location(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let location = LocationService::get_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(location))
}

/// List all locations
pub async fn list_locations(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let locations = LocationService::list(&state.db).await?;
    Ok(Json(locations))
}

/// Delete a location
pub async fn delete_location(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let location = LocationService::delete(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(location))
}

/// Get current time at a location
pub async fn get_location_time(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let response = LocationService::get_time(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(response))
}

/// Get times for all locations
pub async fn list_location_times(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let responses = LocationService::list_times(&state.db).await?;
    Ok(Json(responses))
}
