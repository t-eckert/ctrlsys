//! Timer module containing all timer-related functionality
//!
//! This module is organized into submodules:
//! - `status`: Timer state management and metadata
//! - `runner`: Timer execution logic and lifecycle
//! - `service`: gRPC service implementation

pub mod runner;
pub mod service;
pub mod status;

// Re-export commonly used types
pub use runner::TimerRunner;
pub use service::TimerServiceImpl;
pub use status::{TimerState, TimerStatus};
