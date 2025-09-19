package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"go.uber.org/zap"

	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/config"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/jobs"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/k8s"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/server"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/version"
)

func main() {
	// Handle command line arguments
	if len(os.Args) > 1 {
		switch os.Args[1] {
		case "help", "--help":
			printHelp()
			return
		case "health", "--health":
			if err := healthCheck(); err != nil {
				fmt.Printf("Health check failed: %v\n", err)
				os.Exit(1)
			}
			fmt.Println("Health check passed")
			return
		}
	}

	// Load configuration
	cfg, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("Failed to load configuration: %v\n", err)
		os.Exit(1)
	}

	// Validate configuration
	if err := cfg.Validate(); err != nil {
		fmt.Printf("Configuration validation failed: %v\n", err)
		os.Exit(1)
	}

	// Setup logging
	logger, err := cfg.SetupLogging()
	if err != nil {
		fmt.Printf("Failed to setup logging: %v\n", err)
		os.Exit(1)
	}
	defer logger.Sync()

	logger.Info("Starting JobScheduler service",
		zap.Int("grpc_port", cfg.Server.Port),
		zap.String("namespace", cfg.Kubernetes.DefaultNamespace),
		zap.Bool("in_cluster", cfg.Kubernetes.InCluster))

	// Initialize Kubernetes client
	k8sClient, err := k8s.NewClient(
		cfg.Kubernetes.InCluster,
		cfg.Kubernetes.KubeConfigPath,
		logger,
	)
	if err != nil {
		logger.Fatal("Failed to create Kubernetes client", zap.Error(err))
	}

	// Initialize job registry
	registry := jobs.NewRegistry(logger)
	if err := registry.InitializeDefaultHandlers(); err != nil {
		logger.Fatal("Failed to initialize job handlers", zap.Error(err))
	}

	// Initialize job creator
	jobCreator := k8s.NewJobCreator(k8sClient, cfg, registry, logger)

	// Initialize gRPC server
	grpcServer := server.NewServer(cfg, jobCreator, registry, logger)

	// Setup graceful shutdown
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Handle shutdown signals
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		<-sigChan
		logger.Info("Received shutdown signal")
		cancel()
	}()

	// Start the server
	serverErrChan := make(chan error, 1)
	go func() {
		serverErrChan <- grpcServer.Start()
	}()

	// Wait for shutdown signal or server error
	select {
	case <-ctx.Done():
		logger.Info("Shutting down gracefully")
	case err := <-serverErrChan:
		if err != nil {
			logger.Fatal("Server failed to start", zap.Error(err))
		}
	}

	logger.Info("JobScheduler service stopped")
}

// healthCheck performs a basic health check
func healthCheck() error {
	// Load configuration
	cfg, err := config.LoadConfig()
	if err != nil {
		return fmt.Errorf("failed to load configuration: %w", err)
	}

	// Validate configuration
	if err := cfg.Validate(); err != nil {
		return fmt.Errorf("configuration validation failed: %w", err)
	}

	// Create a simple logger for health check
	logger, _ := zap.NewProduction()
	defer logger.Sync()

	// Test Kubernetes connectivity
	k8sClient, err := k8s.NewClient(
		cfg.Kubernetes.InCluster,
		cfg.Kubernetes.KubeConfigPath,
		logger,
	)
	if err != nil {
		return fmt.Errorf("failed to create Kubernetes client: %w", err)
	}

	// Test basic Kubernetes connectivity
	ctx := context.Background()
	_, err = k8sClient.ListJobs(ctx, cfg.Kubernetes.DefaultNamespace, nil)
	if err != nil {
		return fmt.Errorf("failed to connect to Kubernetes: %w", err)
	}

	return nil
}

// printHelp prints usage information
func printHelp() {
	fmt.Printf(`JobScheduler Service - Kubernetes job scheduling microservice

USAGE:
    jobscheduler [COMMAND]

COMMANDS:
    (no args)      Run the jobscheduler service
    health         Run health check and exit
    help           Print this help message

ENVIRONMENT VARIABLES:
    GRPC_PORT                 gRPC server port (default: 50054)
    HOST                      Server host (default: 0.0.0.0)
    K8S_NAMESPACE             Default Kubernetes namespace (default: default)
    IN_CLUSTER                Use in-cluster config (default: true)
    KUBECONFIG                Path to kubeconfig file
    JOB_TTL_SECONDS           Job TTL in seconds (default: 86400)
    DEFAULT_CPU_REQUEST       Default CPU request (default: 100m)
    DEFAULT_MEMORY_REQUEST    Default memory request (default: 64Mi)
    DEFAULT_CPU_LIMIT         Default CPU limit (default: 200m)
    DEFAULT_MEMORY_LIMIT      Default memory limit (default: 128Mi)
    TIMER_IMAGE               Timer job image (default: timer-service:latest)
    TIMER_CONTROL_PLANE_ENDPOINT  Timer control plane endpoint
    LOG_LEVEL                 Log level (default: info)
    LOG_FORMAT                Log format: json or text (default: json)

EXAMPLES:
    # Run the service
    jobscheduler

    # Run with custom configuration
    GRPC_PORT=8080 K8S_NAMESPACE=jobs LOG_LEVEL=debug jobscheduler

    # Health check
    jobscheduler health
`)
}
