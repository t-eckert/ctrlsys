use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

/// Configuration for the timer service loaded from environment variables
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimerConfig {
    /// Unique identifier for this timer instance
    pub timer_id: String,

    /// Human-readable name for the timer
    pub name: String,

    /// Duration in seconds for how long the timer should run
    pub duration_seconds: u64,

    /// Key-value labels for metadata and filtering
    pub labels: HashMap<String, String>,

    /// Identifier of who/what created this timer
    pub created_by: String,

    /// gRPC endpoint of the control plane service
    pub control_plane_endpoint: String,

    /// Port for the gRPC server to listen on
    pub grpc_port: u16,

    /// Log level for the application
    pub log_level: String,

    /// Update interval in milliseconds for status broadcasts
    pub update_interval_ms: u64,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            timer_id: Uuid::new_v4().to_string(),
            name: "default-timer".to_string(),
            duration_seconds: 300, // 5 minutes default
            labels: HashMap::new(),
            created_by: "system".to_string(),
            control_plane_endpoint: "http://control-plane-service:50053".to_string(),
            grpc_port: 50051,
            log_level: "info".to_string(),
            update_interval_ms: 1000, // 1 second
        }
    }
}

impl TimerConfig {
    /// Load configuration from environment variables
    /// Falls back to defaults for optional values
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Optional: Override timer ID if provided
        if let Ok(timer_id) = env::var("TIMER_ID") {
            if !timer_id.is_empty() {
                config.timer_id = timer_id;
            }
        }

        // Optional: Override timer name
        if let Ok(name) = env::var("TIMER_NAME") {
            if !name.is_empty() {
                config.name = name;
            }
        }

        // Required: Timer duration
        config.duration_seconds = env::var("TIMER_DURATION_SECONDS")
            .context("TIMER_DURATION_SECONDS environment variable is required")?
            .parse::<u64>()
            .context("TIMER_DURATION_SECONDS must be a valid positive number")?;

        // Validate duration is reasonable (between 1 second and 24 hours)
        if config.duration_seconds == 0 || config.duration_seconds > 86400 {
            return Err(anyhow::anyhow!(
                "TIMER_DURATION_SECONDS must be between 1 and 86400 (24 hours)"
            ));
        }

        // Optional: Parse labels from JSON
        if let Ok(labels_str) = env::var("TIMER_LABELS") {
            if !labels_str.is_empty() {
                config.labels = serde_json::from_str(&labels_str)
                    .context("TIMER_LABELS must be valid JSON object")?;
            }
        }

        // Optional: Created by
        if let Ok(created_by) = env::var("TIMER_CREATED_BY") {
            if !created_by.is_empty() {
                config.created_by = created_by;
            }
        }

        // Required: Control plane endpoint
        config.control_plane_endpoint = env::var("CONTROL_PLANE_ENDPOINT")
            .context("CONTROL_PLANE_ENDPOINT environment variable is required")?;

        // Validate control plane endpoint format
        if !config.control_plane_endpoint.starts_with("http://")
            && !config.control_plane_endpoint.starts_with("https://")
        {
            return Err(anyhow::anyhow!(
                "CONTROL_PLANE_ENDPOINT must start with http:// or https://"
            ));
        }

        // Optional: gRPC port
        if let Ok(port_str) = env::var("GRPC_PORT") {
            config.grpc_port = port_str
                .parse::<u16>()
                .context("GRPC_PORT must be a valid port number")?;
        }

        // Optional: Log level
        if let Ok(log_level) = env::var("RUST_LOG") {
            config.log_level = log_level;
        } else if let Ok(log_level) = env::var("LOG_LEVEL") {
            config.log_level = log_level;
        }

        // Optional: Update interval
        if let Ok(interval_str) = env::var("UPDATE_INTERVAL_MS") {
            config.update_interval_ms = interval_str
                .parse::<u64>()
                .context("UPDATE_INTERVAL_MS must be a valid number")?;

            // Validate interval is reasonable (between 100ms and 60s)
            if config.update_interval_ms < 100 || config.update_interval_ms > 60000 {
                return Err(anyhow::anyhow!(
                    "UPDATE_INTERVAL_MS must be between 100 and 60000"
                ));
            }
        }

        Ok(config)
    }

    /// Validate the configuration for consistency
    pub fn validate(&self) -> Result<()> {
        if self.timer_id.is_empty() {
            return Err(anyhow::anyhow!("timer_id cannot be empty"));
        }

        if self.name.is_empty() {
            return Err(anyhow::anyhow!("timer name cannot be empty"));
        }

        if self.duration_seconds == 0 {
            return Err(anyhow::anyhow!("duration_seconds must be greater than 0"));
        }

        if self.control_plane_endpoint.is_empty() {
            return Err(anyhow::anyhow!("control_plane_endpoint cannot be empty"));
        }

        if self.grpc_port == 0 {
            return Err(anyhow::anyhow!("grpc_port must be a valid port number"));
        }

        Ok(())
    }

    /// Get the socket address for the gRPC server
    pub fn grpc_socket_addr(&self) -> String {
        format!("0.0.0.0:{}", self.grpc_port)
    }

    /// Check if debug logging is enabled
    pub fn is_debug(&self) -> bool {
        self.log_level.to_lowercase() == "debug" || self.log_level.to_lowercase() == "trace"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = TimerConfig::default();
        assert!(!config.timer_id.is_empty());
        assert_eq!(config.name, "default-timer");
        assert_eq!(config.duration_seconds, 300);
        assert!(config.labels.is_empty());
        assert_eq!(config.grpc_port, 50051);
    }

    #[test]
    fn test_config_validation() {
        let mut config = TimerConfig::default();
        assert!(config.validate().is_ok());

        // Test invalid duration
        config.duration_seconds = 0;
        assert!(config.validate().is_err());

        // Test empty timer_id
        config.duration_seconds = 300;
        config.timer_id = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_from_env() {
        // Set environment variables
        unsafe {
            env::set_var("TIMER_DURATION_SECONDS", "600");
            env::set_var("TIMER_NAME", "test-timer");
            env::set_var("CONTROL_PLANE_ENDPOINT", "http://localhost:50053");
            env::set_var("TIMER_LABELS", r#"{"env":"test","team":"platform"}"#);
        }

        let config = TimerConfig::from_env().unwrap();

        assert_eq!(config.duration_seconds, 600);
        assert_eq!(config.name, "test-timer");
        assert_eq!(config.control_plane_endpoint, "http://localhost:50053");
        assert_eq!(config.labels.get("env"), Some(&"test".to_string()));
        assert_eq!(config.labels.get("team"), Some(&"platform".to_string()));

        // Clean up
        unsafe {
            env::remove_var("TIMER_DURATION_SECONDS");
            env::remove_var("TIMER_NAME");
            env::remove_var("CONTROL_PLANE_ENDPOINT");
            env::remove_var("TIMER_LABELS");
        }
    }
}
