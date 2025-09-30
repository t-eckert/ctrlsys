use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status of a timer
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimerStatus {
    /// Timer is pending creation/scheduling
    Pending,
    /// Timer is actively running
    Running,
    /// Timer has been paused
    Paused,
    /// Timer has completed successfully
    Completed,
    /// Timer has failed
    Failed,
    /// Timer has been cancelled
    Cancelled,
}

/// Timer resource representing a scheduled timer job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timer {
    /// Unique identifier for the timer
    pub id: String,

    /// Human-readable name for the timer
    pub name: String,

    /// Duration in seconds for how long the timer should run
    pub duration_seconds: u64,

    /// Current status of the timer
    pub status: TimerStatus,

    /// Key-value labels for metadata and filtering
    #[serde(default)]
    pub labels: HashMap<String, String>,

    /// Identifier of who/what created this timer
    pub created_by: String,

    /// Timestamp when the timer was created
    pub created_at: DateTime<Utc>,

    /// Timestamp when the timer was last updated
    pub updated_at: DateTime<Utc>,

    /// Timestamp when the timer started (if running)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,

    /// Timestamp when the timer completed (if completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,

    /// Elapsed time in seconds (for running/paused timers)
    #[serde(default)]
    pub elapsed_seconds: u64,

    /// Error message if the timer failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Request body for creating a new timer
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTimerRequest {
    /// Human-readable name for the timer
    pub name: String,

    /// Duration in seconds for how long the timer should run
    pub duration_seconds: u64,

    /// Key-value labels for metadata and filtering
    #[serde(default)]
    pub labels: HashMap<String, String>,

    /// Identifier of who/what is creating this timer
    #[serde(default = "default_created_by")]
    pub created_by: String,
}

fn default_created_by() -> String {
    "api".to_string()
}

/// Request body for updating a timer
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTimerRequest {
    /// New status for the timer (e.g., pause/resume)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TimerStatus>,

    /// Update labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

/// Response for listing timers
#[derive(Debug, Serialize)]
pub struct ListTimersResponse {
    /// List of timers
    pub timers: Vec<Timer>,

    /// Total count of timers
    pub total: usize,
}

/// Stream event for timer updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerEvent {
    /// Type of event
    pub event_type: TimerEventType,

    /// The timer associated with this event
    pub timer: Timer,

    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
}

/// Types of timer events for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerEventType {
    /// Timer was created
    Created,
    /// Timer status changed
    StatusChanged,
    /// Timer progress update
    Progress,
    /// Timer was deleted
    Deleted,
}

impl Timer {
    /// Create a new timer from a create request
    pub fn new(id: String, req: CreateTimerRequest) -> Self {
        let now = Utc::now();
        Self {
            id,
            name: req.name,
            duration_seconds: req.duration_seconds,
            status: TimerStatus::Pending,
            labels: req.labels,
            created_by: req.created_by,
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
            elapsed_seconds: 0,
            error: None,
        }
    }

    /// Update the timer status
    pub fn update_status(&mut self, status: TimerStatus) {
        self.status = status;
        self.updated_at = Utc::now();

        match status {
            TimerStatus::Running if self.started_at.is_none() => {
                self.started_at = Some(Utc::now());
            }
            TimerStatus::Completed | TimerStatus::Failed | TimerStatus::Cancelled => {
                self.completed_at = Some(Utc::now());
            }
            _ => {}
        }
    }

    /// Update timer labels
    pub fn update_labels(&mut self, labels: HashMap<String, String>) {
        self.labels = labels;
        self.updated_at = Utc::now();
    }

    /// Check if the timer is in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            TimerStatus::Completed | TimerStatus::Failed | TimerStatus::Cancelled
        )
    }
}