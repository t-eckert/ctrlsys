# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is `ctrlsys`, a hobby project control system platform built around Kubernetes for homelab use. It follows a microservices architecture with a control plane pattern, using both Rust and Go.

## Key Commands

### Development Workflow
```bash
task info              # Project overview and available commands
task build:all         # Build all components
task test:all          # Test all components
task check:all         # Format + lint + test all components
task dev               # Full development cycle (check:all)
```

### Component-Specific Development
```bash
task timer:dev         # Timer service development cycle
task api:dev           # API service development cycle
task cli:dev           # CLI development cycle
task lib:dev           # Library development cycle
```

### JobScheduler Service (Go)
```bash
cd services/jobscheduler
task build             # Build the service
task test              # Run tests
task run:debug         # Run with debug logging
task proto:generate    # Generate protobuf code
task container:build   # Build container image
task k8s:apply         # Deploy to Kubernetes
```

### Protocol Buffers with Buf
```bash
task proto:generate       # Generate code for all languages
task proto:generate:go    # Generate Go code only
task proto:generate:rust  # Generate Rust code only
task proto:lint           # Lint protobuf files
task proto:check          # Lint and build protobuf files
task proto:build          # Build protobuf modules
```

### Kubernetes & Local Development
```bash
task k8s:cluster:start    # Start local Kind cluster
task k8s:dev              # Full local K8s setup
task timer:deploy:local   # Deploy timer service locally
task k8s:cluster:status   # Check cluster status
```

## Architecture

### Language Split
- **Rust**: Primary language for most components (timer service, API, CLI, shared libs)
- **Go**: JobScheduler service only (`services/jobscheduler/`)

### Key Services
1. **JobScheduler** (`services/jobscheduler/`): Go-based Kubernetes job scheduling service
   - ConnectRPC API for creating and managing jobs (HTTP/JSON compatible)
   - Kubernetes-native job execution
   - Extensible job registry pattern (Timer jobs implemented, weather/health planned)
   - Uses Zap for structured logging

2. **Timer Service** (`jobs/timer/`): Rust-based timer microservice
   - gRPC interface for timer operations
   - Kubernetes deployment ready

3. **Control Plane** (`apps/controlplane/`): Central coordination (early stage)
4. **API** (`apps/api/`): API gateway service (minimal implementation)
5. **CLI** (`cli/`): Command-line interface using Clap

### ConnectRPC Communication
- JobSchedulerService: `ScheduleJob`, `GetJobStatus`, `ListJobs`, `CancelJob`
- TimerService: `CheckTimer`, `StreamTimer`
- ControlPlaneService: `ReportTimerComplete`
- **Protocol**: Uses ConnectRPC for Go services (gRPC-compatible HTTP/JSON)
- **Proto files**: Centralized in `proto/` directory with proper package structure
- **Generated code**: Located in `gen/go/` and `gen/rust/` directories

### Build System
- **Task**: Primary build automation (distributed Taskfiles)
- **Buf**: Protocol buffer management and code generation
- **Cargo**: Rust workspace at root level
- **Go modules**: Independent module for JobScheduler service
- **Podman**: Container runtime (not Docker)

## Development Patterns

### Code Organization
- Component-specific Taskfiles keep tasks close to code
- Shared libraries in `lib/` (UUID, nomenclator, location, slugs)
- Kubernetes manifests in component `k8s/` directories

### Version Management (JobScheduler)
- Dynamic versioning using Git information
- Build-time injection via ldflags
- `./jobscheduler version` and `./jobscheduler version-full` commands

### Testing & Quality
- Run `task check:all` before commits
- Component-level testing: `task <component>:test`
- Clippy linting with `-D warnings`
- Automatic formatting with `cargo fmt`

### Container & Kubernetes
- All services designed for Kubernetes deployment
- RBAC configurations included
- Non-root containers with security contexts
- Health checks and readiness probes
- Local development with Kind cluster

## Important Notes

- This is a personal hobby project for homelab use
- Use Podman instead of Docker for containers
- JobScheduler service is separate Go module (not part of Cargo workspace)
- Timer service reports completion to control plane endpoint
- Kubernetes Jobs are the execution primitive for scheduled work
- **Buf manages all protobuf files**: Use `task proto:generate` before building
- **Generated code is gitignored**: Always regenerate after proto changes
- **Proto organization**: Follow `ctrlsys.<service>.v1` package naming pattern
- **ConnectRPC**: JobScheduler uses ConnectRPC for modern HTTP/JSON APIs
- **Module structure**: Generated Go code has separate module in `gen/go/`