use crate::config::TimerConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Timer state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerState {
    /// Timer is initializing
    Starting,
    /// Timer is actively running
    Running,
    /// Timer has completed successfully
    Completed,
    /// Timer failed due to an error
    Failed,
}

impl TimerState {
    /// Convert to protobuf enum value
    pub fn to_proto_value(self) -> i32 {
        match self {
            TimerState::Starting => 1,
            TimerState::Running => 2,
            TimerState::Completed => 3,
            TimerState::Failed => 4,
        }
    }

    /// Check if the timer is in a terminal state
    pub fn is_terminal(self) -> bool {
        matches!(self, TimerState::Completed | TimerState::Failed)
    }

    /// Check if the timer is active (running or starting)
    pub fn is_active(self) -> bool {
        matches!(self, TimerState::Starting | TimerState::Running)
    }
}

impl std::fmt::Display for TimerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerState::Starting => write!(f, "starting"),
            TimerState::Running => write!(f, "running"),
            TimerState::Completed => write!(f, "completed"),
            TimerState::Failed => write!(f, "failed"),
        }
    }
}

/// Timer metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerMetadata {
    /// Unique timer identifier
    pub timer_id: String,
    /// Human-readable timer name
    pub name: String,
    /// Key-value labels for categorization
    pub labels: HashMap<String, String>,
    /// Total duration in seconds
    pub duration_seconds: i64,
    /// Creation timestamp
    pub created_at: i64,
    /// Creator identifier
    pub created_by: String,
}

impl TimerMetadata {
    /// Create metadata from configuration
    pub fn from_config(config: &TimerConfig) -> Self {
        let now = Utc::now();
        Self {
            timer_id: config.timer_id.clone(),
            name: config.name.clone(),
            labels: config.labels.clone(),
            duration_seconds: config.duration_seconds as i64,
            created_at: now.timestamp(),
            created_by: config.created_by.clone(),
        }
    }
}

/// Complete timer status including state and timing information
#[derive(Debug, Clone)]
pub struct TimerStatus {
    /// Timer metadata
    pub metadata: TimerMetadata,
    /// Current timer state
    pub state: TimerState,
    /// When the timer started (for calculating elapsed time)
    pub start_time: Instant,
    /// UTC timestamp when timer started
    pub started_at: DateTime<Utc>,
    /// Total duration the timer should run
    pub duration: Duration,
    /// Optional error message if timer failed
    pub error_message: Option<String>,
}

impl TimerStatus {
    /// Create a new timer status from configuration
    pub fn new(config: &TimerConfig) -> Self {
        let now_utc = Utc::now();
        let now_instant = Instant::now();

        Self {
            metadata: TimerMetadata::from_config(config),
            state: TimerState::Starting,
            start_time: now_instant,
            started_at: now_utc,
            duration: Duration::from_secs(config.duration_seconds),
            error_message: None,
        }
    }

    /// Get elapsed time since timer started
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get remaining time until timer completes
    pub fn remaining(&self) -> Duration {
        self.duration.saturating_sub(self.elapsed())
    }

    /// Check if the timer should be completed based on elapsed time
    pub fn should_complete(&self) -> bool {
        self.elapsed() >= self.duration
    }

    /// Get completion percentage (0.0 to 1.0)
    pub fn completion_percentage(&self) -> f64 {
        let elapsed_secs = self.elapsed().as_secs_f64();
        let total_secs = self.duration.as_secs_f64();

        if total_secs == 0.0 {
            1.0
        } else {
            (elapsed_secs / total_secs).min(1.0)
        }
    }

    /// Update the timer state based on current conditions
    pub fn update_state(&mut self) {
        match self.state {
            TimerState::Starting => {
                // Transition to running after a brief startup period
                if self.elapsed() > Duration::from_millis(100) {
                    self.state = TimerState::Running;
                }
            }
            TimerState::Running => {
                // Check if timer should complete
                if self.should_complete() {
                    self.state = TimerState::Completed;
                }
            }
            TimerState::Completed | TimerState::Failed => {
                // Terminal states don't change
            }
        }
    }

    /// Mark the timer as failed with an error message
    pub fn mark_failed(&mut self, error: &str) {
        self.state = TimerState::Failed;
        self.error_message = Some(error.to_string());
    }

    /// Mark the timer as completed
    pub fn mark_completed(&mut self) {
        self.state = TimerState::Completed;
    }

    /// Get elapsed seconds as i64
    pub fn elapsed_seconds(&self) -> i64 {
        self.elapsed().as_secs() as i64
    }

    /// Get remaining seconds as i64
    pub fn remaining_seconds(&self) -> i64 {
        self.remaining().as_secs() as i64
    }

    /// Check if timer is in a terminal state
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Check if timer is active
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Get a summary string of the timer status
    pub fn summary(&self) -> String {
        let elapsed = self.elapsed_seconds();
        let remaining = self.remaining_seconds();
        let percentage = (self.completion_percentage() * 100.0) as u32;

        match self.state {
            TimerState::Starting => {
                format!("Timer '{}' starting...", self.metadata.name)
            }
            TimerState::Running => {
                format!(
                    "Timer '{}' running: {}s elapsed, {}s remaining ({}%)",
                    self.metadata.name, elapsed, remaining, percentage
                )
            }
            TimerState::Completed => {
                format!(
                    "Timer '{}' completed after {}s",
                    self.metadata.name, elapsed
                )
            }
            TimerState::Failed => {
                let error = self.error_message.as_deref().unwrap_or("unknown error");
                format!(
                    "Timer '{}' failed after {}s: {}",
                    self.metadata.name, elapsed, error
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TimerConfig;
    use std::thread;
    use std::time::Duration;

    fn create_test_config() -> TimerConfig {
        let mut config = TimerConfig::default();
        config.duration_seconds = 2; // 2 second timer for testing
        config.name = "test-timer".to_string();
        config
    }

    #[test]
    fn test_timer_state_transitions() {
        assert!(TimerState::Starting.is_active());
        assert!(TimerState::Running.is_active());
        assert!(!TimerState::Completed.is_active());
        assert!(!TimerState::Failed.is_active());

        assert!(!TimerState::Starting.is_terminal());
        assert!(!TimerState::Running.is_terminal());
        assert!(TimerState::Completed.is_terminal());
        assert!(TimerState::Failed.is_terminal());
    }

    #[test]
    fn test_timer_status_creation() {
        let config = create_test_config();
        let status = TimerStatus::new(&config);

        assert_eq!(status.metadata.timer_id, config.timer_id);
        assert_eq!(status.metadata.name, config.name);
        assert_eq!(status.state, TimerState::Starting);
        assert_eq!(status.duration, Duration::from_secs(2));
        assert!(status.error_message.is_none());
    }

    #[test]
    fn test_timer_progression() {
        let config = create_test_config();
        let mut status = TimerStatus::new(&config);

        // Initially starting
        assert_eq!(status.state, TimerState::Starting);

        // Wait a bit and update - should transition to running
        thread::sleep(Duration::from_millis(150));
        status.update_state();
        assert_eq!(status.state, TimerState::Running);

        // Check timing calculations
        assert!(status.elapsed().as_millis() >= 150);
        assert!(status.remaining().as_secs() <= 2);
        assert!(status.completion_percentage() > 0.0);
        assert!(status.completion_percentage() < 1.0);
    }

    #[test]
    fn test_timer_completion() {
        let config = create_test_config();
        let mut status = TimerStatus::new(&config);

        // Manually mark as completed
        status.mark_completed();
        assert_eq!(status.state, TimerState::Completed);
        assert!(status.is_terminal());

        // Test failure
        status.mark_failed("test error");
        assert_eq!(status.state, TimerState::Failed);
        assert_eq!(status.error_message, Some("test error".to_string()));
    }

    #[test]
    fn test_timer_summary() {
        let config = create_test_config();
        let mut status = TimerStatus::new(&config);

        let summary = status.summary();
        assert!(summary.contains("starting"));

        status.state = TimerState::Running;
        let summary = status.summary();
        assert!(summary.contains("running"));
        assert!(summary.contains("elapsed"));
        assert!(summary.contains("remaining"));

        status.mark_completed();
        let summary = status.summary();
        assert!(summary.contains("completed"));

        status.mark_failed("test failure");
        let summary = status.summary();
        assert!(summary.contains("failed"));
        assert!(summary.contains("test failure"));
    }
}
