use crate::config::TimerConfig;
use crate::error::{control_plane_error, TimerError, TimerResult};
use crate::timer::status::{TimerState, TimerStatus};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time::interval;
use tracing::{debug, error, info, warn};

// Generated protobuf types
use crate::timer_proto::{
    control_plane_service_client::ControlPlaneServiceClient, ReportTimerCompleteRequest,
    StreamTimerResponse, TimerMetadata,
};

/// Timer runner manages the lifecycle and execution of a timer
pub struct TimerRunner {
    /// Timer configuration
    config: TimerConfig,
    /// Shared timer status
    status: Arc<RwLock<TimerStatus>>,
    /// Broadcast channel for status updates
    update_sender: broadcast::Sender<StreamTimerResponse>,
}

impl TimerRunner {
    /// Create a new timer runner
    pub fn new(
        config: TimerConfig,
        status: Arc<RwLock<TimerStatus>>,
        update_sender: broadcast::Sender<StreamTimerResponse>,
    ) -> Self {
        Self {
            config,
            status,
            update_sender,
        }
    }

    /// Run the timer until completion or failure
    pub async fn run(&self) -> TimerResult<()> {
        info!(
            timer_id = %self.config.timer_id,
            name = %self.config.name,
            duration = %self.config.duration_seconds,
            "Starting timer execution"
        );

        // Setup update interval
        let mut update_interval = interval(Duration::from_millis(self.config.update_interval_ms));
        let mut tick_count = 0u64;

        loop {
            // Wait for next tick
            update_interval.tick().await;
            tick_count += 1;

            // Update timer state
            let (current_state, should_exit) = {
                let mut status_guard = self.status.write().await;
                status_guard.update_state();

                let elapsed_secs = status_guard.elapsed_seconds();
                let remaining_secs = status_guard.remaining_seconds();
                let current_state = status_guard.state;

                // Log progress periodically
                if tick_count % 10 == 0 || current_state == TimerState::Starting {
                    debug!(
                        timer_id = %self.config.timer_id,
                        state = %current_state,
                        elapsed = elapsed_secs,
                        remaining = remaining_secs,
                        "Timer progress update"
                    );
                }

                // Create update message
                let update = StreamTimerResponse {
                    timer_id: self.config.timer_id.clone(),
                    state: current_state.to_proto_value(),
                    elapsed_seconds: elapsed_secs,
                    remaining_seconds: remaining_secs,
                    timestamp: chrono::Utc::now().timestamp(),
                };

                // Send update to subscribers
                if let Err(e) = self.update_sender.send(update) {
                    warn!(
                        timer_id = %self.config.timer_id,
                        error = %e,
                        "Failed to broadcast timer update"
                    );
                }

                let should_exit = current_state.is_terminal();
                (current_state, should_exit)
            };

            // Exit if timer is complete or failed
            if should_exit {
                match current_state {
                    TimerState::Completed => {
                        info!(
                            timer_id = %self.config.timer_id,
                            "Timer completed successfully"
                        );

                        // Report completion to control plane
                        if let Err(e) = self.report_completion().await {
                            error!(
                                timer_id = %self.config.timer_id,
                                error = %e,
                                "Failed to report timer completion"
                            );
                            // Mark as failed if we can't report completion
                            self.mark_failed(&format!("Failed to report completion: {}", e))
                                .await;
                            return Err(e);
                        }

                        return Ok(());
                    }
                    TimerState::Failed => {
                        let error_msg = {
                            let status = self.status.read().await;
                            status
                                .error_message
                                .clone()
                                .unwrap_or_else(|| "Unknown error".to_string())
                        };

                        error!(
                            timer_id = %self.config.timer_id,
                            error = %error_msg,
                            "Timer failed"
                        );

                        return Err(TimerError::Timer(error_msg));
                    }
                    _ => unreachable!("Should only exit on terminal states"),
                }
            }

            // Safety check: if we've been running much longer than expected, something is wrong
            let status = self.status.read().await;
            if status.elapsed() > status.duration + Duration::from_secs(30) {
                drop(status);
                let error_msg = "Timer exceeded maximum duration by 30 seconds";
                warn!(
                    timer_id = %self.config.timer_id,
                    error = error_msg,
                    "Timer exceeded expected duration"
                );
                self.mark_failed(error_msg).await;
                return Err(TimerError::Timer(error_msg.to_string()));
            }
        }
    }

    /// Mark the timer as failed with an error message
    pub async fn mark_failed(&self, error: &str) {
        let mut status = self.status.write().await;
        status.mark_failed(error);

        error!(
            timer_id = %self.config.timer_id,
            error = error,
            "Timer marked as failed"
        );

        // Send final update
        let update = StreamTimerResponse {
            timer_id: self.config.timer_id.clone(),
            state: TimerState::Failed.to_proto_value(),
            elapsed_seconds: status.elapsed_seconds(),
            remaining_seconds: 0,
            timestamp: chrono::Utc::now().timestamp(),
        };

        if let Err(e) = self.update_sender.send(update) {
            warn!(
                timer_id = %self.config.timer_id,
                error = %e,
                "Failed to send failure update"
            );
        }
    }

    /// Report timer completion to the control plane
    async fn report_completion(&self) -> TimerResult<()> {
        info!(
            timer_id = %self.config.timer_id,
            control_plane = %self.config.control_plane_endpoint,
            "Reporting timer completion to control plane"
        );

        // Create gRPC client with timeout
        let mut client = match tokio::time::timeout(
            Duration::from_secs(10),
            ControlPlaneServiceClient::connect(self.config.control_plane_endpoint.clone()),
        )
        .await
        {
            Ok(Ok(client)) => client,
            Ok(Err(e)) => {
                return Err(control_plane_error(&format!(
                    "Failed to connect to control plane: {}",
                    e
                )));
            }
            Err(_) => {
                return Err(control_plane_error("Timeout connecting to control plane"));
            }
        };

        // Build completion request
        let request = {
            let status = self.status.read().await;
            ReportTimerCompleteRequest {
                timer_id: self.config.timer_id.clone(),
                metadata: Some(TimerMetadata {
                    timer_id: status.metadata.timer_id.clone(),
                    name: status.metadata.name.clone(),
                    labels: status.metadata.labels.clone(),
                    duration_seconds: status.metadata.duration_seconds,
                    created_at: status.metadata.created_at,
                    created_by: status.metadata.created_by.clone(),
                }),
                total_duration_seconds: status.elapsed_seconds(),
                completed_at: chrono::Utc::now().timestamp(),
            }
        };

        // Send completion report with timeout
        match tokio::time::timeout(
            Duration::from_secs(30),
            client.report_timer_complete(request),
        )
        .await
        {
            Ok(Ok(response)) => {
                let resp = response.into_inner();
                if resp.acknowledged {
                    info!(
                        timer_id = %self.config.timer_id,
                        "Timer completion acknowledged by control plane"
                    );
                    Ok(())
                } else {
                    warn!(
                        timer_id = %self.config.timer_id,
                        "Timer completion not acknowledged by control plane"
                    );
                    Err(control_plane_error(
                        "Control plane did not acknowledge completion",
                    ))
                }
            }
            Ok(Err(e)) => Err(control_plane_error(&format!(
                "gRPC error reporting completion: {}",
                e
            ))),
            Err(_) => Err(control_plane_error(
                "Timeout reporting completion to control plane",
            )),
        }
    }

    /// Get a snapshot of the current timer status
    pub async fn get_status_snapshot(&self) -> TimerStatus {
        self.status.read().await.clone()
    }

    /// Check if the timer is still running
    pub async fn is_running(&self) -> bool {
        let status = self.status.read().await;
        status.is_active()
    }

    /// Get the timer configuration
    pub fn config(&self) -> &TimerConfig {
        &self.config
    }

    /// Force stop the timer (for testing or emergency shutdown)
    pub async fn force_stop(&self, reason: &str) {
        info!(
            timer_id = %self.config.timer_id,
            reason = reason,
            "Force stopping timer"
        );

        self.mark_failed(&format!("Force stopped: {}", reason))
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TimerConfig;
    use std::time::Duration;

    fn create_test_config() -> TimerConfig {
        let mut config = TimerConfig::default();
        config.duration_seconds = 1; // 1 second for quick testing
        config.update_interval_ms = 100; // Fast updates for testing
        config.timer_id = "test-timer-123".to_string();
        config.name = "test-timer".to_string();
        config
    }

    #[tokio::test]
    async fn test_timer_runner_creation() {
        let config = create_test_config();
        let status = Arc::new(RwLock::new(TimerStatus::new(&config)));
        let (sender, _) = broadcast::channel(10);

        let runner = TimerRunner::new(config.clone(), status, sender);
        assert_eq!(runner.config().timer_id, config.timer_id);
        assert!(runner.is_running().await);
    }

    #[tokio::test]
    async fn test_force_stop() {
        let config = create_test_config();
        let status = Arc::new(RwLock::new(TimerStatus::new(&config)));
        let (sender, _) = broadcast::channel(10);

        let runner = TimerRunner::new(config, status.clone(), sender);

        // Initially running
        assert!(runner.is_running().await);

        // Force stop
        runner.force_stop("test shutdown").await;

        // Should no longer be running
        assert!(!runner.is_running().await);

        // Should be in failed state
        let snapshot = runner.get_status_snapshot().await;
        assert_eq!(snapshot.state, TimerState::Failed);
        assert!(snapshot.error_message.is_some());
    }

    #[tokio::test]
    async fn test_timer_state_progression() {
        let config = create_test_config();
        let status = Arc::new(RwLock::new(TimerStatus::new(&config)));
        let (sender, mut receiver) = broadcast::channel(10);

        let runner = TimerRunner::new(config, status.clone(), sender);

        // Start the runner in a background task
        let runner_handle = tokio::spawn(async move {
            // This will fail because we don't have a real control plane
            // but we can still test the timer progression
            let _ = runner.run().await;
        });

        // Wait for some updates
        let mut update_count = 0;
        while update_count < 3 {
            if let Ok(update) = receiver.recv().await {
                update_count += 1;
                assert_eq!(update.timer_id, "test-timer-123");
                assert!(update.elapsed_seconds >= 0);

                // Early updates should show starting or running state
                if update_count <= 2 {
                    assert!(
                        update.state == TimerState::Starting.to_proto_value()
                            || update.state == TimerState::Running.to_proto_value()
                    );
                }
            }
        }

        // Cancel the runner
        runner_handle.abort();
    }
}
