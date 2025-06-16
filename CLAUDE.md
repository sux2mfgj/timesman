# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TimesMan is a Rust workspace consisting of a time tracking GUI application and API server with multiple storage backends. The project is structured as a modular system with shared types and pluggable storage implementations.

### Core Components

- **timesman-app**: GUI application built with egui/eframe
- **timesman-server**: API server with HTTP and gRPC frontends  
- **timesman-bstore**: Storage abstraction layer with multiple backends
- **timesman-grpc**: gRPC service definitions and client/server code
- **timesman-type**: Shared data types (Times, Post, Todo, File)
- **timesman-tools**: CLI tools for testing and interaction

### Key Architecture Patterns

The project uses a trait-based storage abstraction where:
- `Store` trait manages collections of time entries
- `TimesStore` trait manages individual time entries
- `PostStore` trait manages posts within a time entry  
- `TodoStore` trait manages todos within a time entry

Storage backends are feature-gated and include:
- Memory (RAM-only)
- Local (UnQLite database)
- JSON file storage
- gRPC remote storage

## Common Development Commands

### Building and Running

```bash
# Build all workspace members
cargo build

# Run the GUI application
cargo run -p timesman-app

# Run the server with default config
cargo run -p timesman-server

# Run server with specific config
cargo run -p timesman-server -- --config timesman-server/config.toml

# Build with specific features
cargo build -p timesman-app --features grpc
cargo build -p timesman-bstore --features "local,grpc"
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p timesman-type
cargo test -p timesman-bstore

# Run tests with specific features
cargo test -p timesman-bstore --features local
```

### gRPC Development

```bash
# Generate gRPC code (automatically done via build.rs)
cargo build -p timesman-grpc

# Test gRPC server connection
./target/debug/timesman-tools --conn-type grpc create-post
```

### Database Setup (for local storage)

```bash
# Set up SQLite database (if using sqlite backend)
DATABASE_URL="sqlite:./sqlite.db" cargo sqlx database create
DATABASE_URL="sqlite:./sqlite.db" cargo sqlx migrate run 

# Generate Sea-ORM entities (if needed) 
sea-orm-cli generate entity -u sqlite:./sqlite.db -o src/sqlite/
```

## Configuration

### App Configuration
- App stores config in `~/.config/timesman/config.toml` (Linux/macOS) or `%APPDATA%\timesman\config.toml` (Windows)
- Storage type selection happens at runtime via UI

### Server Configuration  
- Server config in `timesman-server/config.toml`
- Supports different storage backends: Memory, Local (UnQLite), JSON file
- Can run HTTP and/or gRPC frontends

## Feature Flags

Key feature flags across the workspace:
- `local`: Enable UnQLite local database storage
- `json`: Enable JSON file storage  
- `grpc`: Enable gRPC client/server functionality
- `http`: Enable HTTP client functionality
- `server`: Include server dependencies in app build

## Data Model

The core data types are defined in `timesman-type`:
- `Times`: Top-level time tracking entry with title and timestamps
- `Post`: Text posts with optional file attachments and tags
- `Todo`: Simple todo items with done/pending status
- `File`: File attachments supporting Image, Text, and Other binary data

All types use `chrono::NaiveDateTime` for timestamps and implement standard traits (Clone, Debug, Serialize, Deserialize, PartialEq).