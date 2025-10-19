use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::controllers::timer::{AppError, AppState};
use crate::models::database::CreateDatabaseRequest;
use crate::services::database::DatabaseService;

/// Create a new database
pub async fn create_database(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateDatabaseRequest>,
) -> Result<impl IntoResponse, AppError> {
    let database = DatabaseService::create(&state.db, req).await?;
    Ok((StatusCode::CREATED, Json(database)))
}

/// List all managed databases
pub async fn list_databases(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let databases = DatabaseService::list(&state.db).await?;
    Ok(Json(databases))
}

/// Get a specific database by name
pub async fn get_database(
    State(state): State<Arc<AppState>>,
    Path(db_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let database = DatabaseService::get_by_name(&state.db, &db_name)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(database))
}

/// Drop a database
pub async fn drop_database(
    State(state): State<Arc<AppState>>,
    Path(db_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let database = DatabaseService::drop(&state.db, &db_name).await?;
    Ok(Json(database))
}

/// Check if a database exists
pub async fn check_database_exists(
    State(state): State<Arc<AppState>>,
    Path(db_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let exists = DatabaseService::exists(&state.db, &db_name).await?;
    Ok(Json(serde_json::json!({ "exists": exists })))
}
