# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is `ctrlsys`, a personal hobby project and development platform. It's a Rust project with two binaries (CLI and Server) and shared utility libraries. The project is in early development stages.

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
cargo build --bin cli      # Build just the CLI
cargo build --bin server   # Build just the server
cargo run --bin cli        # Run the CLI
cargo run --bin server     # Run the server
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
│   │   ├── cli.rs        # CLI binary
│   │   └── server.rs     # Server binary
│   ├── lib.rs            # Library exports
│   ├── uuid.rs           # UUID generation utilities
│   ├── nomenclator.rs    # Random name generator (adjective-noun)
│   ├── location.rs       # Location data structure
│   └── slug.rs           # String slugification
├── Cargo.toml            # Rust package configuration (defines both binaries)
├── Taskfile.yml          # Task automation
└── notebook/
    └── Ideas.md          # Future CLI design ideas
```

### Binaries

**CLI** (`src/bin/cli.rs`): Command-line interface for interacting with ctrlsys
- Planned to use verb-noun command model (e.g., `cs get location`)
- Will expose utility functions and resource management

**Server** (`src/bin/server.rs`): Backend server component
- Will provide API endpoints
- Designed for homelab deployment

## Core Libraries

### UUID (`src/uuid.rs`)
- UUID v4 generation
- Uses the `uuid` crate with fast-rng

### Nomenclator (`src/nomenclator.rs`)
- Generates random adjective-noun names (e.g., "brave-compass")
- 50 adjectives × 50 nouns = 2,500 possible combinations
- Useful for naming projects, containers, resources

### Location (`src/location.rs`)
- Data structure for geographic locations
- Fields: id (UUID), name, longitude, latitude

### Slug (`src/slug.rs`)
- String slugification utilities
- Uses the `slugify` crate

## Future Direction

The project is planned to become a CLI tool called `cs` (ControlSystem) with a verb-noun command model similar to kubectl, Docker, and PowerShell. See `notebook/Ideas.md` for the full CLI design.

Planned command structure:
```bash
cs list <resource>     # List resources
cs get <resource>      # Get specific resource
cs add <resource>      # Add new resource
cs set <resource>      # Update resource
cs exec <command>      # Execute utilities
```

Planned resources include locations, weather, time, tasks, links, notes, books, and more.

## Development Patterns

### Testing & Quality
- **Always run `task check:all` before commits**
- All code must pass clippy with `-D warnings`
- Maintain test coverage for all modules
- Use `cargo fmt` for consistent formatting

### Code Organization
- Keep utilities in focused, single-purpose modules
- Each module should have its own tests
- Use the library pattern (both binary and lib)

## Important Notes

- This is a personal hobby project in early development
- **Two binaries, one codebase**: Both CLI and Server share the same library code
- Currently minimal implementations (binaries just print messages)
- Focus is on building reusable utilities first
- Both binaries are defined in Cargo.toml with explicit `[[bin]]` sections
- All shared code lives in library modules (uuid, nomenclator, location, slug)
- No external services, APIs, or database yet
- Pure Rust project (no Go, no protobuf, no Kubernetes)
- Uses Cargo for all build/test operations