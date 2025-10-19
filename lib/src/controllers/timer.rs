use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::ServerConfig;
use crate::models::timer::{CreateTimerRequest, TimerResponse};
use crate::services::timer::{TimerService, to_response};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: ServerConfig,
}

/// Create a new timer
pub async fn create_timer(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTimerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let timer = TimerService::create(&state.db, req).await?;

    // Auto-start the timer
    let timer = TimerService::start(&state.db, timer.id)
        .await?
        .ok_or(AppError::NotFound)?;

    let response = to_response(timer);
    Ok((StatusCode::CREATED, Json(response)))
}

/// Get a timer by ID
pub async fn get_timer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let timer = TimerService::get_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let response = to_response(timer);
    Ok(Json(response))
}

/// List all timers
pub async fn list_timers(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let timers = TimerService::list(&state.db).await?;
    let responses: Vec<TimerResponse> = timers.into_iter().map(to_response).collect();
    Ok(Json(responses))
}

/// Cancel a timer
pub async fn cancel_timer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let timer = TimerService::cancel(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let response = to_response(timer);
    Ok(Json(response))
}

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Internal(anyhow::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Timer not found"),
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, message).into_response()
    }
}
