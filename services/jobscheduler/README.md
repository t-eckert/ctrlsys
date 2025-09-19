# JobScheduler Service

A Kubernetes-native job scheduling microservice that provides gRPC APIs for creating and managing various types of jobs. The service is designed to be extensible and currently supports Timer jobs with plans for additional job types like weather reporting and health checking.

## Features

- **gRPC API**: Robust gRPC interface for job scheduling and management
- **Extensible Design**: Plugin-like architecture for easy addition of new job types
- **Kubernetes Native**: Direct integration with Kubernetes Jobs API
- **Timer Jobs**: Built-in support for timer-based jobs with configurable duration
- **Resource Management**: Configurable CPU/memory requests and limits
- **Health Checks**: Built-in health and readiness probes
- **Observability**: Structured logging with configurable levels
- **Security**: RBAC-enabled with least-privilege access

## Architecture

```
┌─────────────────┐    gRPC     ┌─────────────────┐    K8s API    ┌─────────────────┐
│                 │────────────▶│                 │──────────────▶│                 │
│   gRPC Client   │             │  JobScheduler   │               │  Kubernetes     │
│                 │◀────────────│   Service       │◀──────────────│    Cluster      │
└─────────────────┘             └─────────────────┘               └─────────────────┘
                                          │
                                          ▼
                                ┌─────────────────┐
                                │                 │
                                │   Job Registry  │
                                │   - Timer Job   │
                                │   - Future Jobs │
                                └─────────────────┘
```

## Quick Start

### Prerequisites

- Go 1.21+
- Protocol Buffers compiler (`protoc`)
- Kubernetes cluster access
- Podman (for containerization)
- Git (for version information)

### Development Setup

1. **Clone and navigate to the service:**
   ```bash
   cd services/jobscheduler
   ```

2. **Install development tools:**
   ```bash
   task tools:install
   ```

3. **Generate protobuf code:**
   ```bash
   task proto:generate
   ```

4. **Run tests:**
   ```bash
   task test
   ```

5. **Run locally:**
   ```bash
   task run:debug
   ```

### Building and Deployment

1. **Build the service:**
   ```bash
   task build
   ```

2. **Build container image:**
   ```bash
   task container:build
   ```

3. **Deploy to Kubernetes:**
   ```bash
   task k8s:apply
   ```

## Usage

### Scheduling a Timer Job

```bash
# Using grpcurl
grpcurl -plaintext -d '{
  "name": "my-timer",
  "timer_job": {
    "duration_seconds": 300,
    "timer_name": "build-timeout",
    "control_plane_endpoint": "http://control-plane:50053"
  }
}' localhost:50054 jobscheduler.JobScheduler/ScheduleJob
```

### Getting Job Status

```bash
grpcurl -plaintext -d '{
  "job_id": "job-abc123"
}' localhost:50054 jobscheduler.JobScheduler/GetJobStatus
```

### Listing Jobs

```bash
grpcurl -plaintext -d '{}' localhost:50054 jobscheduler.JobScheduler/ListJobs
```

## Configuration

The service can be configured using environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `GRPC_PORT` | `50054` | gRPC server port |
| `K8S_NAMESPACE` | `default` | Default Kubernetes namespace |
| `IN_CLUSTER` | `true` | Use in-cluster Kubernetes config |
| `LOG_LEVEL` | `info` | Logging level (trace, debug, info, warn, error) |
| `TIMER_IMAGE` | `timer-service:latest` | Default timer job container image |
| `DEFAULT_CPU_REQUEST` | `100m` | Default CPU request for jobs |
| `DEFAULT_MEMORY_REQUEST` | `64Mi` | Default memory request for jobs |

## API Reference

### gRPC Service: `JobScheduler`

#### Methods

- **`ScheduleJob`**: Schedule a new job
- **`GetJobStatus`**: Get the status of a job
- **`ListJobs`**: List jobs with optional filtering
- **`CancelJob`**: Cancel a running job

### Job Types

#### Timer Job

Runs a timer service for a specified duration.

**Configuration:**
- `duration_seconds`: Timer duration in seconds (required)
- `timer_name`: Human-readable timer name (required)
- `control_plane_endpoint`: Endpoint for completion reporting (required)
- `image`: Container image override (optional)
- `log_level`: Logging level (optional)
- `env`: Additional environment variables (optional)

## Development

### Project Structure

```
services/jobscheduler/
├── main.go                    # Application entry point
├── proto/                     # Protocol buffer definitions
│   └── jobscheduler.proto
├── internal/
│   ├── config/               # Configuration management
│   ├── server/               # gRPC server implementation
│   ├── jobs/                 # Job type handlers and registry
│   └── k8s/                  # Kubernetes client and operations
├── k8s/                      # Kubernetes manifests
├── scripts/                  # Build and utility scripts
└── Taskfile.yml             # Task automation
```

### Adding New Job Types

1. **Define protobuf message** in `proto/jobscheduler.proto`
2. **Implement JobHandler interface** in `internal/jobs/`
3. **Register handler** in `internal/jobs/registry.go`
4. **Update request validation** in `internal/k8s/job_creator.go`

### Available Tasks

```bash
task --list
```

Key tasks:
- `task dev` - Full development cycle (check + run with debug)
- `task check` - Format, vet, and test code
- `task container:build` - Build container image with version info
- `task k8s:apply` - Deploy to Kubernetes
- `task health` - Run health check
- `task version:info` - Show version information that will be embedded

### Version Management

The service uses dynamic versioning based on Git information:

```bash
# Build with version information (automatically detected)
task build

# Show what version information will be embedded
task version:info

# Check version in built binary
./jobscheduler version        # Short version
./jobscheduler version-full   # Detailed version with build info

# Build container with version information
task container:build
```

Version information includes:
- **Version**: Git describe output (tags, commits)
- **Git Commit**: Short commit hash with dirty indicator
- **Build Date**: ISO 8601 timestamp
- **Go Version**: Go compiler version used

## Security

- **RBAC**: Service uses least-privilege RBAC for Kubernetes access
- **Non-root**: Container runs as non-root user
- **Read-only filesystem**: Container uses read-only root filesystem
- **Security contexts**: Proper security contexts applied

## Monitoring

- **Health checks**: Built-in health and readiness endpoints
- **Structured logging**: JSON-formatted logs for easy parsing
- **Metrics**: Prometheus-compatible metrics endpoint (port 8080)

## Troubleshooting

### Common Issues

1. **Permission denied on Kubernetes operations**
   - Ensure RBAC is properly applied: `kubectl apply -f k8s/rbac.yaml`

2. **Cannot connect to Kubernetes cluster**
   - Check `IN_CLUSTER` environment variable
   - Verify kubeconfig path if running locally

3. **Protobuf generation fails**
   - Install protoc: `brew install protobuf` (macOS) or equivalent
   - Install Go plugins: `task tools:install`

### Debugging

1. **Enable debug logging:**
   ```bash
   LOG_LEVEL=debug task run
   ```

2. **Check service logs in Kubernetes:**
   ```bash
   task k8s:logs
   ```

3. **Port forward for local testing:**
   ```bash
   task k8s:port-forward
   ```

## Future Enhancements

- **Weather Reporter Jobs**: Scheduled weather data collection
- **Health Check Jobs**: Periodic service health monitoring
- **Webhook Jobs**: HTTP webhook execution
- **Batch Processing**: Support for batch job execution
- **Job Templates**: Predefined job configurations
- **Job Dependencies**: Job execution dependencies and workflows