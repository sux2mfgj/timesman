# TimesMan Server

A high-performance gRPC server for the TimesMan time tracking system, built with Rust and Tokio for async operations.

## Features

- **gRPC API**: Fast, type-safe communication using Protocol Buffers
- **Multiple Storage Backends**: Memory, Local UnQLite database, and JSON file storage
- **Async/Await**: Built on Tokio for high concurrency
- **Configurable**: TOML-based configuration with sensible defaults
- **Times Management**: Create, read, update, and delete time tracking sessions
- **Posts & Todos**: Manage posts and todo items within each time session

## Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- Protocol Buffers compiler (protoc)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd timesman/timesman-server

# Build the server
cargo build --release
```

### Basic Usage

```bash
# Run with default configuration (Memory store, gRPC on 127.0.0.1:8080)
cargo run

# Run with custom config file
cargo run -- --config /path/to/config.toml
```

## Configuration

The server uses TOML configuration files. Create a `config.toml` file:

```toml
listen = "127.0.0.1:8080"
front_type = "Grpc"

[store]
type = "Memory"  # Options: "Memory", "Local", "Json"
```

### Storage Options

#### Memory Store (Default)
```toml
[store]
type = "Memory"
```
- Fast in-memory storage
- Data is lost when server stops
- Perfect for development and testing

#### Local UnQLite Database
```toml
[store]
type = "Local"
path = "~/Library/Application Support/timesman/unqlite.db"
```
- Persistent local database
- Requires `local` feature: `cargo build --features local`

#### JSON File Storage
```toml
[store]
type = "Json"
path = "./timesman_data.json"
create = true  # Create file if it doesn't exist
```
- Human-readable JSON storage
- Requires `json` feature: `cargo build --features json`

## API Reference

The server exposes the following gRPC endpoints:

### Times Management
- `GetTimes()` - Retrieve all time tracking sessions
- `CreateTimes(TimesTitle)` - Create a new time session
- `UpdateTimes(Times)` - Update existing time session
- `DeleteTimes(TimesId)` - Delete a time session

### Posts Management
- `GetPosts(TimesId)` - Get all posts for a time session
- `CreatePost(CreatePostParams)` - Add a new post
- `UpdatePost(UpdatePostParam)` - Update existing post
- `DeletePost(DeletePostParam)` - Delete a post

### Todo Management
- `GetTodos(TimesId)` - Get all todos for a time session
- `CreateTodo(CreateTodoParams)` - Create a new todo
- `DoneTodo(DoneTodoParams)` - Mark todo as done/undone

## Development

### Building with Features

```bash
# Build with all features
cargo build --features "grpc,local,json"

# Build with specific features
cargo build --features "grpc,local"
```

### Available Features
- `grpc` (default): Enable gRPC server support
- `local`: Enable UnQLite local database storage
- `json`: Enable JSON file storage

### Project Structure

```
timesman-server/
   src/
      main.rs          # Server entry point and CLI
      lib.rs           # Server trait definition
      config.rs        # Configuration management
      grpc.rs          # gRPC service implementation
   config.toml          # Default configuration
   Cargo.toml          # Dependencies and features
```

## Architecture

The server implements a modular architecture:

- **TimesManServer Trait**: Defines the server interface
- **GrpcServer**: gRPC implementation using Tonic
- **Store Abstraction**: Pluggable storage backends via timesman-bstore
- **Configuration**: Flexible TOML-based configuration system

## Dependencies

Key dependencies include:
- `tonic`: gRPC framework for Rust
- `tokio`: Async runtime
- `timesman-bstore`: Storage abstraction layer
- `timesman-grpc`: Protocol buffer definitions
- `toml`: Configuration parsing

## Error Handling

The server provides comprehensive error handling:
- gRPC status codes for API errors
- Detailed error messages for debugging
- Graceful handling of storage backend failures

## Performance

- Async/await for high concurrency
- Zero-copy message handling where possible
- Efficient storage backend implementations
- Minimal memory footprint with smart defaults

## License

[Add your license information here]