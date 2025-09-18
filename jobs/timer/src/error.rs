use std::fmt;
use tonic::{Code, Status};

/// Custom error types for the timer service
#[derive(Debug)]
pub enum TimerError {
    /// Configuration-related errors
    Config(String),

    /// gRPC communication errors
    Grpc(tonic::Status),

    /// Timer execution errors
    Timer(String),

    /// Control plane communication errors
    ControlPlane(String),

    /// Internal service errors
    Internal(String),

    /// Validation errors
    Validation(String),
}

impl fmt::Display for TimerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerError::Config(msg) => write!(f, "Configuration error: {}", msg),
            TimerError::Grpc(status) => write!(f, "gRPC error: {}", status.message()),
            TimerError::Timer(msg) => write!(f, "Timer error: {}", msg),
            TimerError::ControlPlane(msg) => write!(f, "Control plane error: {}", msg),
            TimerError::Internal(msg) => write!(f, "Internal error: {}", msg),
            TimerError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for TimerError {}

impl From<TimerError> for tonic::Status {
    fn from(err: TimerError) -> Self {
        match err {
            TimerError::Config(msg) => {
                Status::invalid_argument(format!("Configuration error: {}", msg))
            }
            TimerError::Grpc(status) => status,
            TimerError::Timer(msg) => Status::internal(format!("Timer error: {}", msg)),
            TimerError::ControlPlane(msg) => {
                Status::unavailable(format!("Control plane error: {}", msg))
            }
            TimerError::Internal(msg) => Status::internal(format!("Internal error: {}", msg)),
            TimerError::Validation(msg) => {
                Status::invalid_argument(format!("Validation error: {}", msg))
            }
        }
    }
}

impl From<tonic::Status> for TimerError {
    fn from(status: tonic::Status) -> Self {
        TimerError::Grpc(status)
    }
}

impl From<anyhow::Error> for TimerError {
    fn from(err: anyhow::Error) -> Self {
        TimerError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for TimerError {
    fn from(err: serde_json::Error) -> Self {
        TimerError::Config(format!("JSON parsing error: {}", err))
    }
}

/// Result type alias for timer operations
pub type TimerResult<T> = Result<T, TimerError>;

/// Helper function to create validation errors
pub fn validation_error(msg: &str) -> TimerError {
    TimerError::Validation(msg.to_string())
}

/// Helper function to create configuration errors
pub fn config_error(msg: &str) -> TimerError {
    TimerError::Config(msg.to_string())
}

/// Helper function to create timer errors
pub fn timer_error(msg: &str) -> TimerError {
    TimerError::Timer(msg.to_string())
}

/// Helper function to create control plane errors
pub fn control_plane_error(msg: &str) -> TimerError {
    TimerError::ControlPlane(msg.to_string())
}

/// Helper function to create internal errors
pub fn internal_error(msg: &str) -> TimerError {
    TimerError::Internal(msg.to_string())
}

/// Convert gRPC status codes to appropriate TimerError types
pub fn grpc_status_to_timer_error(status: tonic::Status) -> TimerError {
    match status.code() {
        Code::InvalidArgument => TimerError::Validation(status.message().to_string()),
        Code::NotFound => TimerError::Timer("Timer not found".to_string()),
        Code::Unavailable => TimerError::ControlPlane(status.message().to_string()),
        Code::DeadlineExceeded => TimerError::ControlPlane("Request timeout".to_string()),
        _ => TimerError::Grpc(status),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let config_err = TimerError::Config("missing field".to_string());
        assert_eq!(config_err.to_string(), "Configuration error: missing field");

        let timer_err = TimerError::Timer("execution failed".to_string());
        assert_eq!(timer_err.to_string(), "Timer error: execution failed");
    }

    #[test]
    fn test_error_conversion_to_status() {
        let config_err = TimerError::Config("invalid config".to_string());
        let status: tonic::Status = config_err.into();
        assert_eq!(status.code(), Code::InvalidArgument);

        let timer_err = TimerError::Timer("timer failed".to_string());
        let status: tonic::Status = timer_err.into();
        assert_eq!(status.code(), Code::Internal);
    }

    #[test]
    fn test_helper_functions() {
        let err = validation_error("invalid input");
        assert!(matches!(err, TimerError::Validation(_)));

        let err = config_error("missing config");
        assert!(matches!(err, TimerError::Config(_)));

        let err = timer_error("timer stopped");
        assert!(matches!(err, TimerError::Timer(_)));
    }
}
