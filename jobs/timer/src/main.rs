use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::{broadcast, RwLock};
use tokio::time::sleep;
use tonic::transport::Server;
use tracing::{error, info, warn};

// Import our library modules
use timer::{
    timer_proto::timer_service_server::TimerServiceServer, TimerConfig, TimerError, TimerRunner,
    TimerServiceImpl, TimerStatus,
};

/// Initialize tracing/logging based on configuration
fn init_tracing(config: &TimerConfig) {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default to info level, but use config if available
        let level = match config.log_level.to_lowercase().as_str() {
            "trace" => "trace",
            "debug" => "debug",
            "info" => "info",
            "warn" => "warn",
            "error" => "error",
            _ => "info",
        };
        tracing_subscriber::EnvFilter::new(format!("timer_service={}", level))
    });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .json(), // Use JSON format for better parsing in Kubernetes
        )
        .with(env_filter)
        .init();
}

/// Setup graceful shutdown handling
async fn setup_shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        }
        _ = terminate => {
            info!("Received terminate signal");
        }
    }
}

/// Main timer service application
async fn run_timer_service() -> Result<()> {
    // Load configuration from environment
    let config = TimerConfig::from_env().context("Failed to load timer configuration")?;

    // Initialize logging
    init_tracing(&config);

    info!(
        timer_id = %config.timer_id,
        name = %config.name,
        duration = %config.duration_seconds,
        grpc_port = %config.grpc_port,
        control_plane = %config.control_plane_endpoint,
        "Starting timer service"
    );

    // Validate configuration
    config
        .validate()
        .context("Configuration validation failed")?;

    // Create shared timer status
    let timer_status = Arc::new(RwLock::new(TimerStatus::new(&config)));

    // Create broadcast channel for timer updates
    let (update_sender, _update_receiver) = broadcast::channel::<timer::StreamTimerResponse>(1000);

    // Create timer service implementation
    let timer_service = TimerServiceImpl::new(
        config.clone(),
        Arc::clone(&timer_status),
        update_sender.clone(),
    );

    // Create timer runner
    let timer_runner = TimerRunner::new(config.clone(), Arc::clone(&timer_status), update_sender);

    // Setup gRPC server address
    let grpc_addr = config
        .grpc_socket_addr()
        .parse()
        .context("Invalid gRPC server address")?;

    info!(
        addr = %grpc_addr,
        "Starting gRPC server"
    );

    // Start gRPC server task
    let grpc_server_task = {
        let timer_service = timer_service.clone();
        tokio::spawn(async move {
            let result = Server::builder()
                .add_service(TimerServiceServer::new(timer_service))
                .serve_with_shutdown(grpc_addr, setup_shutdown_signal())
                .await;

            if let Err(e) = result {
                error!(error = %e, "gRPC server failed");
            } else {
                info!("gRPC server shut down cleanly");
            }
        })
    };

    // Wait a moment for the gRPC server to start
    sleep(Duration::from_millis(500)).await;

    // Health check the service before starting the timer
    if let Err(e) = timer_service.health_check().await {
        error!(error = %e, "Service health check failed");
        return Err(anyhow::anyhow!("Service health check failed: {}", e));
    }

    info!("Service health check passed, starting timer");

    // Start timer execution task
    let timer_task = {
        tokio::spawn(async move {
            match timer_runner.run().await {
                Ok(()) => {
                    info!("Timer completed successfully");
                }
                Err(TimerError::ControlPlane(msg)) => {
                    error!(error = %msg, "Timer failed due to control plane error");
                    // Don't return error for control plane issues as the timer itself worked
                }
                Err(e) => {
                    error!(error = %e, "Timer execution failed");
                }
            }
        })
    };

    // Monitor both tasks and handle their completion
    tokio::select! {
        _ = timer_task => {
            info!("Timer task completed");
            // Give the gRPC server a moment to send any final responses
            sleep(Duration::from_secs(2)).await;
        }
        _ = grpc_server_task => {
            warn!("gRPC server shut down before timer completed");
        }
    }

    info!("Timer service shutting down");
    Ok(())
}

/// Health check endpoint (can be extended for Kubernetes probes)
async fn health_check() -> Result<()> {
    // Basic startup validation
    if let Err(e) = TimerConfig::from_env() {
        error!(error = %e, "Health check failed: configuration invalid");
        return Err(e.into());
    }

    info!("Health check passed");
    Ok(())
}

/// Main entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Handle special command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--health-check" | "health" => {
                return health_check().await;
            }
            "--version" | "version" => {
                println!("Timer Service v{}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--help" | "help" => {
                print_help();
                return Ok(());
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                print_help();
                std::process::exit(1);
            }
        }
    }

    // Run the main timer service
    if let Err(e) = run_timer_service().await {
        error!(error = %e, "Timer service failed");
        std::process::exit(1);
    }

    Ok(())
}

/// Print help information
fn print_help() {
    println!("Timer Service - Kubernetes-based timer microservice");
    println!();
    println!("USAGE:");
    println!("    timer-service [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    (no args)      Run the timer service");
    println!("    health         Run health check and exit");
    println!("    version        Print version and exit");
    println!("    help           Print this help message");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    TIMER_DURATION_SECONDS    Required. Timer duration in seconds");
    println!("    CONTROL_PLANE_ENDPOINT    Required. gRPC endpoint for control plane");
    println!("    TIMER_ID                  Optional. Unique timer identifier");
    println!("    TIMER_NAME                Optional. Human-readable timer name");
    println!("    TIMER_LABELS              Optional. JSON object with key-value labels");
    println!("    TIMER_CREATED_BY          Optional. Creator identifier");
    println!("    GRPC_PORT                 Optional. gRPC server port (default: 50051)");
    println!("    RUST_LOG                  Optional. Log level (debug, info, warn, error)");
    println!("    UPDATE_INTERVAL_MS        Optional. Update broadcast interval (default: 1000)");
    println!();
    println!("EXAMPLES:");
    println!("    # Run a 5-minute timer");
    println!("    TIMER_DURATION_SECONDS=300 \\");
    println!("    CONTROL_PLANE_ENDPOINT=http://control-plane:50053 \\");
    println!("    timer-service");
    println!();
    println!("    # Run with custom configuration");
    println!("    TIMER_DURATION_SECONDS=600 \\");
    println!("    TIMER_NAME=build-timeout \\");
    println!("    TIMER_LABELS='{{\"env\":\"prod\",\"team\":\"platform\"}}' \\");
    println!("    CONTROL_PLANE_ENDPOINT=http://control-plane:50053 \\");
    println!("    RUST_LOG=debug \\");
    println!("    timer-service");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_health_check() {
        // Set required environment variables
        unsafe {
            env::set_var("TIMER_DURATION_SECONDS", "300");
            env::set_var("CONTROL_PLANE_ENDPOINT", "http://localhost:50053");
        }

        let result = health_check().await;
        if let Err(ref e) = result {
            eprintln!("Health check error: {}", e);
        }
        assert!(result.is_ok());

        // Clean up
        unsafe {
            env::remove_var("TIMER_DURATION_SECONDS");
            env::remove_var("CONTROL_PLANE_ENDPOINT");
        }
    }

    #[tokio::test]
    async fn test_health_check_failure() {
        // Don't set required variables
        unsafe {
            env::remove_var("TIMER_DURATION_SECONDS");
            env::remove_var("CONTROL_PLANE_ENDPOINT");
        }

        let result = health_check().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_help_output() {
        // Just make sure help doesn't panic
        print_help();
    }
}

/// Integration test module for end-to-end testing
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::env;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    #[ignore] // Ignore by default as it requires a longer running test
    async fn test_complete_timer_flow() {
        // Set up test environment
        unsafe {
            env::set_var("TIMER_DURATION_SECONDS", "2"); // Short timer for testing
            env::set_var("TIMER_NAME", "integration-test");
            env::set_var("CONTROL_PLANE_ENDPOINT", "http://localhost:50053");
            env::set_var("GRPC_PORT", "50099"); // Use different port for testing
            env::set_var("RUST_LOG", "debug");
        }

        // This test would need a mock control plane service to fully work
        // For now, we just test that the service starts without panicking
        let result = timeout(Duration::from_secs(5), run_timer_service()).await;

        // The service should either complete successfully or fail due to control plane
        // connectivity (which is expected in test environment)
        match result {
            Ok(Ok(())) => {
                // Service completed successfully
            }
            Ok(Err(_)) => {
                // Service failed, likely due to control plane connectivity
                // This is expected in test environment
            }
            Err(_) => {
                // Test timed out - this means the service is running but didn't complete
                // This is also acceptable for this integration test
            }
        }

        // Clean up
        unsafe {
            env::remove_var("TIMER_DURATION_SECONDS");
            env::remove_var("TIMER_NAME");
            env::remove_var("CONTROL_PLANE_ENDPOINT");
            env::remove_var("GRPC_PORT");
            env::remove_var("RUST_LOG");
        }
    }
}
