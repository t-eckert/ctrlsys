use crate::config::TimerConfig;
use crate::error::{validation_error, TimerResult};
use crate::timer::status::TimerStatus;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::{debug, info, warn};

// Generated protobuf types
use crate::timer_proto::{
    timer_service_server::TimerService, CheckTimerRequest, CheckTimerResponse, StreamTimerRequest,
    StreamTimerResponse, TimerMetadata as ProtoTimerMetadata,
};

/// gRPC service implementation for the timer service
#[derive(Clone)]
pub struct TimerServiceImpl {
    /// Timer configuration
    config: TimerConfig,
    /// Shared timer status
    status: Arc<RwLock<TimerStatus>>,
    /// Broadcast sender for timer updates
    update_sender: broadcast::Sender<StreamTimerResponse>,
}

impl TimerServiceImpl {
    /// Create a new timer service implementation
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

    /// Convert internal TimerStatus to protobuf TimerMetadata
    fn convert_metadata(&self, status: &TimerStatus) -> ProtoTimerMetadata {
        ProtoTimerMetadata {
            timer_id: status.metadata.timer_id.clone(),
            name: status.metadata.name.clone(),
            labels: status.metadata.labels.clone(),
            duration_seconds: status.metadata.duration_seconds,
            created_at: status.metadata.created_at,
            created_by: status.metadata.created_by.clone(),
        }
    }

    /// Validate timer ID in request matches our timer
    fn validate_timer_id(&self, timer_id: &str) -> Result<(), Status> {
        if timer_id.is_empty() {
            return Err(Status::invalid_argument("Timer ID cannot be empty"));
        }

        if timer_id != self.config.timer_id {
            return Err(Status::not_found(format!(
                "Timer ID '{}' not found. This service manages timer '{}'",
                timer_id, self.config.timer_id
            )));
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl TimerService for TimerServiceImpl {
    /// Check the current status of the timer
    async fn check_timer(
        &self,
        request: Request<CheckTimerRequest>,
    ) -> Result<Response<CheckTimerResponse>, Status> {
        let req = request.into_inner();

        debug!(
            timer_id = %req.timer_id,
            "Received check timer request"
        );

        // Validate timer ID
        self.validate_timer_id(&req.timer_id)?;

        // Get current status
        let status = self.status.read().await;

        let response = CheckTimerResponse {
            timer_id: status.metadata.timer_id.clone(),
            metadata: Some(self.convert_metadata(&status)),
            state: status.state.to_proto_value(),
            elapsed_seconds: status.elapsed_seconds(),
            remaining_seconds: status.remaining_seconds(),
        };

        info!(
            timer_id = %req.timer_id,
            state = %status.state,
            elapsed = status.elapsed_seconds(),
            remaining = status.remaining_seconds(),
            "Timer status check completed"
        );

        Ok(Response::new(response))
    }

    /// Stream timer updates in real-time
    type StreamTimerStream = ReceiverStream<Result<StreamTimerResponse, Status>>;

    async fn stream_timer(
        &self,
        request: Request<StreamTimerRequest>,
    ) -> Result<Response<Self::StreamTimerStream>, Status> {
        let req = request.into_inner();

        info!(
            timer_id = %req.timer_id,
            "Starting timer stream"
        );

        // Validate timer ID
        self.validate_timer_id(&req.timer_id)?;

        // Subscribe to updates
        let mut receiver = self.update_sender.subscribe();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        // Send current status immediately
        {
            let status = self.status.read().await;
            let initial_update = StreamTimerResponse {
                timer_id: status.metadata.timer_id.clone(),
                state: status.state.to_proto_value(),
                elapsed_seconds: status.elapsed_seconds(),
                remaining_seconds: status.remaining_seconds(),
                timestamp: chrono::Utc::now().timestamp(),
            };

            if tx.send(Ok(initial_update)).await.is_err() {
                warn!("Failed to send initial timer update to stream");
                return Err(Status::internal("Failed to initialize stream"));
            }
        }

        // Spawn task to forward updates to the stream
        let timer_id_filter = req.timer_id.clone();
        tokio::spawn(async move {
            while let Ok(update) = receiver.recv().await {
                // Only forward updates for the requested timer
                if update.timer_id == timer_id_filter {
                    if tx.send(Ok(update)).await.is_err() {
                        debug!("Timer stream client disconnected");
                        break;
                    }
                }
            }
            debug!("Timer stream ended for timer: {}", timer_id_filter);
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

/// Health check implementation for the timer service
impl TimerServiceImpl {
    /// Check if the service is healthy and ready to serve requests
    pub async fn health_check(&self) -> TimerResult<()> {
        // Basic health checks
        if self.config.timer_id.is_empty() {
            return Err(validation_error("Timer ID is empty"));
        }

        if self.config.duration_seconds == 0 {
            return Err(validation_error("Timer duration is zero"));
        }

        // Check if status is accessible
        let _status = self.status.read().await;

        // Check if broadcast channel is functional
        let test_update = StreamTimerResponse {
            timer_id: "health-check".to_string(),
            state: 0,
            elapsed_seconds: 0,
            remaining_seconds: 0,
            timestamp: chrono::Utc::now().timestamp(),
        };

        // Try to send (will fail if no receivers, but that's ok for health check)
        let _ = self.update_sender.send(test_update);

        Ok(())
    }

    /// Get service metrics and statistics
    pub async fn get_metrics(&self) -> TimerServiceMetrics {
        let status = self.status.read().await;
        let subscriber_count = self.update_sender.receiver_count();

        TimerServiceMetrics {
            timer_id: status.metadata.timer_id.clone(),
            state: status.state,
            elapsed_seconds: status.elapsed_seconds(),
            remaining_seconds: status.remaining_seconds(),
            completion_percentage: status.completion_percentage(),
            active_streams: subscriber_count,
            created_at: status.metadata.created_at,
        }
    }
}

/// Service metrics for monitoring and observability
#[derive(Debug, Clone)]
pub struct TimerServiceMetrics {
    pub timer_id: String,
    pub state: crate::timer::status::TimerState,
    pub elapsed_seconds: i64,
    pub remaining_seconds: i64,
    pub completion_percentage: f64,
    pub active_streams: usize,
    pub created_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer::status::TimerState;
    use futures::StreamExt;
    // Remove unused imports

    fn create_test_config() -> TimerConfig {
        let mut config = TimerConfig::default();
        config.timer_id = "test-timer-456".to_string();
        config.name = "test-service-timer".to_string();
        config.duration_seconds = 5;
        config
    }

    #[tokio::test]
    async fn test_service_creation() {
        let config = create_test_config();
        let status = Arc::new(RwLock::new(TimerStatus::new(&config)));
        let (sender, _) = broadcast::channel(10);

        let service = TimerServiceImpl::new(config.clone(), status, sender);

        // Test health check
        assert!(service.health_check().await.is_ok());

        // Test metrics
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.timer_id, config.timer_id);
        assert_eq!(metrics.state, TimerState::Starting);
    }

    #[tokio::test]
    async fn test_check_timer() {
        let config = create_test_config();
        let status = Arc::<RwLock<TimerStatus>>::new(RwLock::new(TimerStatus::new(&config)));
        let (sender, _) = broadcast::channel(10);

        let service = TimerServiceImpl::new(config.clone(), status.clone(), sender);

        // Test valid request
        let request = Request::new(CheckTimerRequest {
            timer_id: config.timer_id.clone(),
        });

        let response = service.check_timer(request).await.unwrap();
        let resp = response.into_inner();

        assert_eq!(resp.timer_id, config.timer_id);
        assert!(resp.metadata.is_some());
        assert_eq!(resp.state, TimerState::Starting.to_proto_value());
        assert_eq!(resp.elapsed_seconds, 0);
        assert!(resp.remaining_seconds <= 5 && resp.remaining_seconds >= 4);

        // Test invalid timer ID
        let invalid_request = Request::new(CheckTimerRequest {
            timer_id: "wrong-timer".to_string(),
        });

        let error = service.check_timer(invalid_request).await.unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);
    }

    #[tokio::test]
    async fn test_stream_timer() {
        let config = create_test_config();
        let status = Arc::new(RwLock::new(TimerStatus::new(&config)));
        let (sender, _) = broadcast::channel(10);

        let service = TimerServiceImpl::new(config.clone(), status.clone(), sender.clone());

        // Start streaming
        let request = Request::new(StreamTimerRequest {
            timer_id: config.timer_id.clone(),
        });

        let response = service.stream_timer(request).await.unwrap();
        let mut stream = response.into_inner();

        // Should get initial update immediately
        let initial_update = stream.next().await.unwrap().unwrap();
        assert_eq!(initial_update.timer_id, config.timer_id);
        assert_eq!(initial_update.state, TimerState::Starting.to_proto_value());

        // Send a manual update
        let test_update = StreamTimerResponse {
            timer_id: config.timer_id.clone(),
            state: TimerState::Running.to_proto_value(),
            elapsed_seconds: 1,
            remaining_seconds: 4,
            timestamp: chrono::Utc::now().timestamp(),
        };

        sender.send(test_update.clone()).unwrap();

        // Should receive the update
        let received_update = stream.next().await.unwrap().unwrap();
        assert_eq!(received_update.timer_id, test_update.timer_id);
        assert_eq!(received_update.state, test_update.state);
        assert_eq!(received_update.elapsed_seconds, test_update.elapsed_seconds);
    }

    #[tokio::test]
    async fn test_timer_id_validation() {
        let config = create_test_config();
        let status = Arc::new(RwLock::new(TimerStatus::new(&config)));
        let (sender, _) = broadcast::channel(10);

        let service = TimerServiceImpl::new(config, status, sender);

        // Test empty timer ID
        assert!(service.validate_timer_id("").is_err());

        // Test wrong timer ID
        let error = service.validate_timer_id("wrong-id").unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);

        // Test correct timer ID
        assert!(service.validate_timer_id("test-timer-456").is_ok());
    }
}
