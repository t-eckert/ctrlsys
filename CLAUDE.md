# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is `ctrlsys`, a personal hobby project and development platform. It's a Rust project with two binaries (CLI and Server) and shared utility libraries for managing homelab infrastructure and development workflows.

**Architecture:**
- **CLI** (`cs`): Command-line client using Clap for argument parsing, will use Ratatui for interactive TUI modes
- **Server**: Axum-based REST API + WebSocket server with PostgreSQL backend
- **Shared libraries**: UUID, nomenclator, location, slug utilities
- **Authentication**: API token-based (Bearer tokens)
- **Database**: PostgreSQL with SQLx and migrations

## Key Commands

### Development Workflow
```bash
task                   # Show logo and list all available tasks
task build:all         # Build all binaries
task build:release     # Build release version
task test:all          # Run all tests
task check:all         # Format + lint + test (run before commits)
task dev               # Full development cycle (check:all)
```

### Building Specific Binaries
**IMPORTANT:** Always use `--features` flag when building binaries directly!

```bash
# CLI
cargo build --bin cli --features cli
cargo run --bin cli --features cli -- <args>

# Server
cargo build --bin server --features server
cargo run --bin server --features server

# Examples:
cargo run --bin cli --features cli -- config show
cargo run --bin cli --features cli -- timer list
```

### Code Quality
```bash
task fmt:all           # Format all code with cargo fmt
task clippy:all        # Lint with clippy (-D warnings)
task clean:all         # Clean build artifacts
```

### Quick Aliases
```bash
task b                 # Build all (alias)
task t                 # Test all (alias)
task c                 # Check all (alias)
```

### Utilities
```bash
task watch:all         # Watch for changes and rebuild
task logo              # Display the ctrlsys logo
```

## Project Structure

This is a Rust project with two binaries and shared library modules:

```
ctrlsys/
├── src/
│   ├── bin/
│   │   ├── cli/
│   │   │   ├── main.rs       # CLI entry point
│   │   │   ├── client.rs     # HTTP client for API calls
│   │   │   └── commands/     # Command handlers (timer, location, task, etc.)
│   │   └── server/
│   │       ├── main.rs       # Server entry point
│   │       ├── auth.rs       # Auth middleware
│   │       └── state.rs      # Application state
│   ├── config/           # Configuration (CLI & Server)
│   ├── models/           # Database models (server-only)
│   ├── controllers/      # API controllers (server-only)
│   ├── services/         # Business logic (server-only)
│   ├── db/               # Database utilities (server-only)
│   ├── ws/               # WebSocket handlers (server-only)
│   ├── lib.rs            # Library exports
│   ├── uuid.rs           # UUID generation utilities
│   ├── nomenclator.rs    # Random name generator (adjective-noun)
│   ├── location.rs       # Location data structure
│   └── slug.rs           # String slugification
├── migrations/           # SQL migrations (SQLx)
├── Cargo.toml            # Package config with feature flags
├── Taskfile.yml          # Task automation
└── notebook/
    └── Ideas.md          # Feature ideas and CLI design
```

### Binaries

**CLI** (`src/bin/cli/`): Command-line interface
- Uses Clap with derive API for argument parsing
- Verb-noun command model: `cs <resource> <action>`
- HTTP client to communicate with server
- Configuration stored in `~/.config/ctrlsys/config.toml`
- Commands: timer, location, task, template, db, config

**Server** (`src/bin/server/`): Backend API server
- Axum web framework with async/await
- PostgreSQL database via SQLx
- API token authentication (Bearer tokens)
- REST API + WebSocket endpoints
- Configuration via environment variables

## Core Libraries & Modules

### Shared Utilities
- **uuid** (`src/uuid.rs`): UUID v4 generation
- **nomenclator** (`src/nomenclator.rs`): Random adjective-noun names (2,500 combinations)
- **location** (`src/location.rs`): Geographic location data structure
- **slug** (`src/slug.rs`): String slugification
- **config** (`src/config/`): Configuration management for CLI and server

### Server-Only Modules (compiled with `--features server`)
- **models** (`src/models/`): Database models for timers, locations, tasks, templates, databases
- **controllers** (`src/controllers/`): API route handlers
- **services** (`src/services/`): Business logic layer
- **db** (`src/db/`): Database connection pooling and migrations
- **ws** (`src/ws/`): WebSocket handlers for real-time updates

### CLI-Only Modules (compiled with `--features cli`)
- **client** (`src/bin/cli/client.rs`): HTTP client wrapper
- **commands** (`src/bin/cli/commands/`): Command implementations

## Planned Features

### 1. Timers
- Create and manage timers from CLI
- Blocking mode: Full-screen TUI with live countdown (Ratatui)
- Non-blocking mode: Background timers with async updates
- WebSocket subscriptions for real-time timer updates

### 2. Time Zones
- Save locations with timezone information
- Query current time at any saved location
- Display time across multiple locations simultaneously

### 3. Postgres Database Management
- Create and manage databases on homelab Postgres server
- Track metadata about managed databases
- Safety features for destructive operations

### 4. Project Scaffolding
- Create and manage project templates
- Template variables and substitution
- Pre-built templates for common project types (Rust, SvelteKit, etc.)

### 5. Task/TODO Management
- Create and track tasks
- Start timers on tasks to track time spent
- Integrate with timer system for automatic time logging

See `notebook/Ideas.md` for additional feature ideas.

## Development Patterns

### Code Style
- **No emoji or icons in code**: Do not use emoji, checkmarks, or other Unicode icons in code, comments, output messages, or documentation
- Keep all text plain ASCII for maximum compatibility and clarity
- Use words instead of symbols (e.g., "Success" not "✓", "Error" not "✗")

### Feature Flags
The project uses Cargo features to conditionally compile code:
- `server`: Enables Axum, SQLx, and server-specific dependencies
- `cli`: Enables Clap, Ratatui, Reqwest, and CLI-specific dependencies
- Modules are conditionally compiled: `#[cfg(feature = "server")]`

### Database Migrations
- SQLx compile-time checked queries
- Migrations in `migrations/` directory
- Run migrations on server startup automatically
- Use `sqlx migrate` CLI for manual migration management

### Configuration
- **CLI**: TOML config at `~/.config/ctrlsys/config.toml`
- **Server**: Environment variables (`DATABASE_URL`, `CTRLSYS_PORT`, `CTRLSYS_API_TOKENS`)
- Config modules: `src/config/cli.rs` and `src/config/server.rs`

### Testing & Quality
- **Always run `task check:all` before commits**
- All code must pass clippy with `-D warnings`
- Maintain test coverage for all modules
- Use `cargo fmt` for consistent formatting

### Code Organization
- Server code uses controller → service pattern
- Keep utilities in focused, single-purpose modules
- Each module should have its own tests
- Shared types go in models

## Important Notes

- This is a personal hobby project for homelab and development workflow management
- **Two binaries, one codebase**: Both CLI and Server share library code
- **Feature-based compilation**: Always use `--features` flag when building binaries
- **Sprint 1 Complete**: Core architecture, config system, auth middleware, basic CLI/server structure
- **Database**: Requires PostgreSQL instance (configured via `DATABASE_URL`)
- **Authentication**: Server uses Bearer token auth, CLI stores token in config file
- **Pure Rust**: No Go, no protobuf, no Kubernetes (simplified from previous version)
- **SQLx**: Uses compile-time checked queries and migrations

### MCP Servers for Claude Code

For better Rust development experience, install these MCP servers:
1. **Rust Docs MCP**: Fetches documentation from docs.rs
2. **CrateDocs**: Quick crate documentation lookup

### Next Steps (Sprint 2)

Implement the first feature: **Timers**
1. Create timer service with CRUD operations
2. Add timer controller with REST endpoints
3. Implement WebSocket handler for live updates
4. Add CLI commands for timer management
5. Build Ratatui TUI for blocking timer watch mode