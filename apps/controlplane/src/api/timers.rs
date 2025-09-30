use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response, Json},
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use crate::models::{
    CreateTimerRequest, ListTimersResponse, Timer, TimerStatus, UpdateTimerRequest,
};
use crate::state::AppState;

/// Router for timer-related endpoints
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_timer).get(list_timers))
        .route("/{id}", get(get_timer)
            .put(update_timer)
            .delete(delete_timer))
}

/// Create a new timer
async fn create_timer(
    State(state): State<AppState>,
    Json(req): Json<CreateTimerRequest>,
) -> Result<Response, AppError> {
    // Validate duration
    if req.duration_seconds == 0 || req.duration_seconds > 86400 {
        return Err(AppError::BadRequest(
            "duration_seconds must be between 1 and 86400 (24 hours)".to_string(),
        ));
    }

    // Validate name
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name cannot be empty".to_string()));
    }

    // Generate a new UUID for the timer
    let timer_id = Uuid::new_v4().to_string();

    // Create the timer
    let timer = Timer::new(timer_id, req);

    // Store in state
    state.add_timer(timer.clone()).await;

    // TODO: Schedule the timer job with the job scheduler

    Ok((StatusCode::CREATED, Json(timer)).into_response())
}

/// List all timers
async fn list_timers(State(state): State<AppState>) -> Response {
    let timers = state.get_all_timers().await;
    let total = timers.len();

    let response = ListTimersResponse { timers, total };

    Json(response).into_response()
}

/// Get a specific timer by ID
async fn get_timer(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    match state.get_timer(&id).await {
        Some(timer) => Ok(Json(timer).into_response()),
        None => Err(AppError::NotFound(format!("Timer {} not found", id))),
    }
}

/// Update a timer
async fn update_timer(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTimerRequest>,
) -> Result<Response, AppError> {
    // Get the existing timer
    let mut timer = state
        .get_timer(&id)
        .await
        .ok_or_else(|| AppError::NotFound(format!("Timer {} not found", id)))?;

    // Check if timer is in a terminal state
    if timer.is_terminal() {
        return Err(AppError::Conflict(format!(
            "Timer {} is in terminal state {:?} and cannot be updated",
            id, timer.status
        )));
    }

    // Update status if provided
    if let Some(new_status) = req.status {
        // Validate status transitions
        match (&timer.status, &new_status) {
            (TimerStatus::Running, TimerStatus::Paused) => {
                // OK: pause a running timer
                timer.update_status(new_status);
            }
            (TimerStatus::Paused, TimerStatus::Running) => {
                // OK: resume a paused timer
                timer.update_status(new_status);
            }
            (TimerStatus::Pending, TimerStatus::Running) => {
                // OK: start a pending timer
                timer.update_status(new_status);
            }
            (TimerStatus::Running, TimerStatus::Cancelled) |
            (TimerStatus::Paused, TimerStatus::Cancelled) |
            (TimerStatus::Pending, TimerStatus::Cancelled) => {
                // OK: cancel a non-terminal timer
                timer.update_status(new_status);
            }
            _ => {
                return Err(AppError::BadRequest(format!(
                    "Invalid status transition from {:?} to {:?}",
                    timer.status, new_status
                )));
            }
        }

        // TODO: Communicate status change to job scheduler
    }

    // Update labels if provided
    if let Some(labels) = req.labels {
        timer.update_labels(labels);
    }

    // Save the updated timer
    state.update_timer(timer.clone()).await;

    Ok(Json(timer).into_response())
}

/// Delete a timer
async fn delete_timer(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    // Check if timer exists
    if !state.timer_exists(&id).await {
        return Err(AppError::NotFound(format!("Timer {} not found", id)));
    }

    // Delete the timer
    state.delete_timer(&id).await;

    // TODO: Cancel the job in the job scheduler if it's running

    Ok(StatusCode::NO_CONTENT.into_response())
}

/// Application error type
#[derive(Debug)]
enum AppError {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}