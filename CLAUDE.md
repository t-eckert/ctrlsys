# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is `ctrlsys`, a personal hobby project and development platform. It's a Rust Cargo workspace with three crates: a CLI binary, a server binary, and a shared library for managing homelab infrastructure and development workflows.

**Architecture:**
- **Workspace Structure**: Three separate crates (cli, server, lib) in a Cargo workspace
- **CLI** (`cs`): Command-line client using Clap for argument parsing and Ratatui for interactive TUI modes
- **Server**: Axum-based REST API + WebSocket server with PostgreSQL backend
- **Shared library**: Common code including models, services, controllers, utilities
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

```bash
# Build specific crates
cargo build -p cli           # Build CLI binary
cargo build -p server        # Build server binary
cargo build -p lib           # Build shared library

# Run binaries
cargo run -p cli -- <args>
cargo run -p server

# Examples:
cargo run -p cli -- config show
cargo run -p cli -- timer list
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

This is a Cargo workspace with three crates:

```
ctrlsys/                  # Workspace root
├── Cargo.toml            # Workspace definition
├── migrations/           # SQL migrations (SQLx)
├── Taskfile.yml          # Task automation
├── notebook/
│   └── Ideas.md          # Feature ideas and CLI design
├── cli/                  # CLI binary crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs       # CLI entry point
│       ├── client.rs     # HTTP client for API calls
│       ├── commands/     # Command handlers (timer, location, weather, database, etc.)
│       └── tui/          # Ratatui TUI components (timer_watch, watch_all, etc.)
├── server/               # Server binary crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs       # Server entry point
│       ├── auth.rs       # Auth middleware
│       └── state.rs      # Application state
└── lib/                  # Shared library crate
    ├── Cargo.toml
    └── src/
        ├── lib.rs        # Library exports
        ├── config/       # Configuration (CLI & Server)
        ├── models/       # Database models (timer, location, database, etc.)
        ├── controllers/  # API controllers (REST endpoints)
        ├── services/     # Business logic (timer, location, weather, geocoding, database)
        ├── db/           # Database utilities and migrations
        ├── ws/           # WebSocket handlers (real-time updates)
        ├── uuid.rs       # UUID generation utilities
        ├── nomenclator.rs # Random name generator (adjective-noun)
        ├── location.rs   # Location data structure with timezone
        └── slug.rs       # String slugification
```

### Binaries

**CLI** (`cli/`): Command-line interface crate
- Uses Clap with derive API for argument parsing
- Verb-noun command model: `cs <resource> <action>`
- HTTP client to communicate with server
- Configuration stored in `~/.config/ctrlsys/config.toml`
- Commands: timer, location, weather, database, config
- TUI modes: timer watch, watch-all, location world clock, weather dashboard

**Server** (`server/`): Backend API server crate
- Axum web framework with async/await
- PostgreSQL database via SQLx
- API token authentication (Bearer tokens)
- REST API + WebSocket endpoints
- Configuration via environment variables

## Core Libraries & Modules

**Shared Library** (`lib/`): All shared code between CLI and server

### Shared Utilities
- **uuid** (`lib/src/uuid.rs`): UUID v4 generation
- **nomenclator** (`lib/src/nomenclator.rs`): Random adjective-noun names (2,500 combinations)
- **location** (`lib/src/location.rs`): Geographic location data structure with timezone support
- **slug** (`lib/src/slug.rs`): String slugification
- **config** (`lib/src/config/`): Configuration management for CLI and server

### Library Modules (used by server, some by CLI)
- **models** (`lib/src/models/`): Database models for timers, locations, managed_databases
- **controllers** (`lib/src/controllers/`): API route handlers (timer, location, weather, database)
- **services** (`lib/src/services/`): Business logic layer (timer, location, weather, geocoding, database)
- **db** (`lib/src/db/`): Database connection pooling and migrations
- **ws** (`lib/src/ws/`): WebSocket handlers for real-time timer updates

## Implemented Features

### 1. Timers (Sprint 2 - COMPLETED)
- Create and manage timers from CLI
- Blocking mode: Full-screen TUI with live countdown (Ratatui)
- Non-blocking mode: Background timers with async updates
- WebSocket subscriptions for real-time timer updates
- Watch-all mode to monitor multiple running timers
- Smart filtering (hide timers completed >24 hours ago)

### 2. Locations & Timezones (Sprint 3 - COMPLETED)
- Save locations with timezone information
- Query current time at any saved location
- Display time across multiple locations simultaneously
- Live world clock TUI showing all locations
- Full timezone conversion support via chrono-tz
- Automatic geocoding for city names using OpenWeatherMap API
- Offline timezone lookup from coordinates using tzf-rs

### 3. Weather (Sprint 3 Enhancement - COMPLETED)
- Check weather at any saved location
- Live weather dashboard TUI for all locations
- Integration with OpenWeatherMap API
- Display temperature, conditions, and wind speed

### 4. Postgres Database Management (Sprint 4 - COMPLETED)
- Create and manage databases on homelab Postgres server
- Track metadata (owner, notes) about managed databases
- Safety features for destructive operations (protected databases, confirmation prompts)
- Automatic connection termination before database deletion
- SQL injection protection via database name validation

## Planned Features

### 5. Project Scaffolding
- Create and manage project templates
- Template variables and substitution
- Pre-built templates for common project types (Rust, SvelteKit, etc.)

### 6. Task/TODO Management
- Create and track tasks
- Start timers on tasks to track time spent
- Integrate with timer system for automatic time logging

See `notebook/Ideas.md` for additional feature ideas.

## Development Patterns

### Code Style
- **No emoji or icons in code**: Do not use emoji, checkmarks, or other Unicode icons in code, comments, output messages, or documentation
- Keep all text plain ASCII for maximum compatibility and clarity
- Use words instead of symbols (e.g., "Success" not "✓", "Error" not "✗")

### Workspace Structure
The project uses a Cargo workspace with three crates:
- **cli**: CLI binary with command handlers and TUI components
- **server**: Server binary with routing and application state
- **lib**: Shared library with models, services, controllers, and utilities
- Benefits: No feature flags needed, cleaner LSP experience, better separation of concerns

### Database Migrations
- SQLx compile-time checked queries
- Migrations in `migrations/` directory
- Run migrations on server startup automatically
- Use `sqlx migrate` CLI for manual migration management

### Configuration
- **CLI**: TOML config at `~/.config/ctrlsys/config.toml`
- **Server**: Environment variables (`DATABASE_URL`, `CTRLSYS_PORT`, `CTRLSYS_API_TOKENS`)
- Config modules: `lib/src/config/cli.rs` and `lib/src/config/server.rs`
- Config is shared via the lib crate

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
- **Cargo Workspace**: Three crates (cli, server, lib) in a workspace - no feature flags needed
- **Shared Library**: All common code lives in the `lib` crate
- **Sprint 1 Complete**: Core architecture, config system, auth middleware, basic CLI/server structure
- **Sprint 2 Complete**: Timer feature with CRUD operations, WebSocket updates, and TUI watch modes
- **Sprint 3 Complete**: Location/timezone feature with world clock TUI, weather integration, geocoding
- **Sprint 4 Complete**: Postgres database management with safety features
- **Database**: Requires PostgreSQL instance (configured via `DATABASE_URL`)
- **Authentication**: Server uses Bearer token auth, CLI stores token in config file
- **Pure Rust**: No Go, no protobuf, no Kubernetes (simplified from previous version)
- **SQLx**: Uses compile-time checked queries and migrations
- **OpenWeatherMap**: Weather and geocoding features require API key in config

### MCP Servers for Claude Code

For better Rust development experience, install these MCP servers:
1. **Rust Docs MCP**: Fetches documentation from docs.rs
2. **CrateDocs**: Quick crate documentation lookup

### Completed Sprints

**Sprint 1**: Core architecture, config system, auth middleware, basic CLI/server structure
**Sprint 2**: Timers - Full CRUD, WebSocket updates, TUI watch modes
**Sprint 3**: Locations & Timezones - World clocks, timezone conversions, live TUI, weather, geocoding
**Sprint 4**: Database Management - Create/manage Postgres databases with safety features
**Infrastructure**: Restructured to Cargo workspace (cli, server, lib) for better LSP support

### Next Sprint Options

Choose one of these features to implement next:
1. **Project Scaffolding** - Create and manage project templates with variable substitution
2. **Task/TODO Management** - Create and track tasks with timer integration