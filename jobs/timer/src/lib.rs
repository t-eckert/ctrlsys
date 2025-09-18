//! Timer Service Library
//!
//! This library provides a Kubernetes-compatible timer microservice
//! with gRPC interfaces for monitoring and control.

pub mod config;
pub mod error;
pub mod timer;

// Re-export commonly used types
pub use config::TimerConfig;
pub use error::{TimerError, TimerResult};
pub use timer::{TimerRunner, TimerServiceImpl, TimerState, TimerStatus};

// Include generated protobuf code
pub mod timer_proto {
    tonic::include_proto!("timer");
}

// Re-export protobuf types for convenience
pub use timer_proto::*;
