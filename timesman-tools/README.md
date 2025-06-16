# TimesMan Tools CLI

A command line interface for interacting with TimesMan time tracking system via gRPC.

## Overview

TimesMan Tools provides a comprehensive CLI for managing time entries (Times) and their associated posts through a gRPC connection to a TimesMan server. This tool allows you to create, read, update, and delete both time entries and posts programmatically.

## Installation & Setup

### Prerequisites

- Rust 1.70+ with Cargo
- Running TimesMan gRPC server (timesman-server)

### Building

```bash
# Build the CLI tool
cargo build -p timesman-tools

# Or build and run directly
cargo run -p timesman-tools -- [OPTIONS] <COMMAND>
```

### Server Configuration

By default, the CLI connects to `http://127.0.0.1:8080/`. You can specify a different server:

```bash
cargo run -p timesman-tools -- --conn-type grpc --server "http://localhost:9090" <COMMAND>
```

## Usage

### Basic Syntax

```bash
timesman-tools --conn-type <CONNECTION_TYPE> [--server <SERVER_URL>] <COMMAND>
```

### TUI Mode (Recommended)

For an interactive experience, use the TUI (Text User Interface) mode:

```bash
cargo run -p timesman-tools -- --conn-type grpc tui
```

The TUI provides a full-screen interface with:
- **Times List**: Browse, create, edit, and delete time entries
- **Posts List**: Manage posts within selected time entries  
- **Keyboard Navigation**: Intuitive arrow key navigation
- **Real-time Updates**: Automatic refresh and status updates
- **Error Handling**: Clear error messages and recovery
- **Help System**: Built-in help accessible with 'h' key

#### Global Options

- `--conn-type <TYPE>` (required) - Connection type (currently only `grpc` supported)
- `--server <URL>` (optional) - Server URL (default: `http://127.0.0.1:8080/`)
- `--help` - Show help information
- `--version` - Show version information

### TUI Keyboard Shortcuts

#### Times List View
- `↑/↓` - Navigate list
- `Enter` - View posts for selected time entry
- `n` - Create new time entry
- `e` - Edit selected time entry
- `d` - Delete selected time entry
- `r` - Refresh list

#### Posts List View  
- `↑/↓` - Navigate list
- `n` - Create new post
- `e` - Edit selected post
- `d` - Delete selected post
- `r` - Refresh list
- `Esc` - Return to times list

#### Global Shortcuts
- `h` - Show help
- `q` / `Ctrl+Q` - Quit application
- `Esc` - Cancel current action / Go back

## Commands

### TUI Mode

```bash
cargo run -p timesman-tools -- --conn-type grpc tui
```

**Interactive Text User Interface** - Full-featured TUI for managing times and posts with keyboard navigation.

### Times Management

#### List All Times
```bash
cargo run -p timesman-tools -- --conn-type grpc get-times-list
```

**Example Output:**
```
1 My Project 2024-01-15 10:30:00 2024-01-15 14:20:00
2 Personal Tasks 2024-01-16 09:00:00
```

#### Create New Time Entry
```bash
cargo run -p timesman-tools -- --conn-type grpc create-times --title "Project Name"
```

**Options:**
- `--title <TITLE>` (required) - Title for the new time entry

**Example:**
```bash
cargo run -p timesman-tools -- --conn-type grpc create-times --title "Website Redesign"
```

#### Update Time Entry
```bash
cargo run -p timesman-tools -- --conn-type grpc update-times --tid <ID> --title "New Title"
```

**Options:**
- `--tid <ID>` (required) - Time entry ID to update
- `--title <TITLE>` (required) - New title for the time entry

**Example:**
```bash
cargo run -p timesman-tools -- --conn-type grpc update-times --tid 1 --title "Website Redesign v2"
```

#### Delete Time Entry
```bash
cargo run -p timesman-tools -- --conn-type grpc delete-times --tid <ID>
```

**Options:**
- `--tid <ID>` (required) - Time entry ID to delete

**Example:**
```bash
cargo run -p timesman-tools -- --conn-type grpc delete-times --tid 1
```

### Posts Management

#### List Posts for Time Entry
```bash
cargo run -p timesman-tools -- --conn-type grpc get-post-list --tid <TIME_ID>
```

**Options:**
- `--tid <ID>` (required) - Time entry ID to list posts for

**Example Output:**
```
ID: 1, Post: Initial project setup completed, Created: 2024-01-15 10:30:00, Updated: None, Tag: None
ID: 2, Post: Database schema designed, Created: 2024-01-15 11:45:00, Updated: Some(2024-01-15 12:00:00), Tag: Some(1)
```

#### Create New Post
```bash
cargo run -p timesman-tools -- --conn-type grpc create-post --tid <TIME_ID> --text "Post content"
```

**Options:**
- `--tid <ID>` (required) - Time entry ID to add post to
- `--text <TEXT>` (required) - Content of the post

**Example:**
```bash
cargo run -p timesman-tools -- --conn-type grpc create-post --tid 1 --text "Completed user authentication module"
```

#### Update Post
```bash
cargo run -p timesman-tools -- --conn-type grpc update-post --tid <TIME_ID> --pid <POST_ID> --text "Updated content"
```

**Options:**
- `--tid <ID>` (required) - Time entry ID
- `--pid <ID>` (required) - Post ID to update
- `--text <TEXT>` (required) - New content for the post

**Example:**
```bash
cargo run -p timesman-tools -- --conn-type grpc update-post --tid 1 --pid 2 --text "Completed user authentication and authorization modules"
```

#### Delete Post
```bash
cargo run -p timesman-tools -- --conn-type grpc delete-post --tid <TIME_ID> --pid <POST_ID>
```

**Options:**
- `--tid <ID>` (required) - Time entry ID
- `--pid <ID>` (required) - Post ID to delete

**Example:**
```bash
cargo run -p timesman-tools -- --conn-type grpc delete-post --tid 1 --pid 2
```

## Workflow Examples

### Daily Work Tracking

```bash
# 1. Create a new time entry for today's work
cargo run -p timesman-tools -- --conn-type grpc create-times --title "Daily Development - 2024-01-15"

# 2. Add progress posts throughout the day
cargo run -p timesman-tools -- --conn-type grpc create-post --tid 1 --text "Started working on user login feature"
cargo run -p timesman-tools -- --conn-type grpc create-post --tid 1 --text "Implemented password validation"
cargo run -p timesman-tools -- --conn-type grpc create-post --tid 1 --text "Added unit tests for authentication module"

# 3. Review the day's work
cargo run -p timesman-tools -- --conn-type grpc get-post-list --tid 1
```

### Project Management

```bash
# List all current projects
cargo run -p timesman-tools -- --conn-type grpc get-times-list

# Create milestone posts
cargo run -p timesman-tools -- --conn-type grpc create-post --tid 2 --text "Phase 1 completed: Database design finished"
cargo run -p timesman-tools -- --conn-type grpc create-post --tid 2 --text "Phase 2 started: Frontend development begun"

# Update project status
cargo run -p timesman-tools -- --conn-type grpc update-times --tid 2 --title "Website Redesign - Phase 2"
```

## Error Handling

The CLI provides detailed error messages for common issues:

- **Connection errors**: Server unreachable or gRPC connection failed
- **Invalid IDs**: When referencing non-existent time entries or posts
- **Missing arguments**: When required parameters are not provided
- **Server errors**: When the TimesMan server returns an error

## Integration with Scripts

The CLI is designed to work well in automation scripts:

```bash
#!/bin/bash

# Create daily standup entry
TIMES_ID=$(cargo run -p timesman-tools -- --conn-type grpc create-times --title "Standup $(date +%Y-%m-%d)" | grep -o 'ID: [0-9]*' | cut -d' ' -f2)

# Add standup items
cargo run -p timesman-tools -- --conn-type grpc create-post --tid $TIMES_ID --text "Yesterday: Completed user authentication"
cargo run -p timesman-tools -- --conn-type grpc create-post --tid $TIMES_ID --text "Today: Working on user authorization"
cargo run -p timesman-tools -- --conn-type grpc create-post --tid $TIMES_ID --text "Blockers: Need database schema review"
```

## Troubleshooting

### Common Issues

1. **"gRPC error: Connection refused"**
   - Ensure the TimesMan server is running
   - Check the server URL (default: `http://127.0.0.1:8080/`)
   - Verify the server is configured to accept gRPC connections

2. **"Missing required argument"**
   - Use `--help` with any command to see required parameters
   - Ensure all required options are provided

3. **"Invalid ID" errors**
   - Use `get-times-list` to verify time entry IDs
   - Use `get-post-list --tid <ID>` to verify post IDs

### Debug Mode

For detailed error information, you can build and run in debug mode:

```bash
RUST_LOG=debug cargo run -p timesman-tools -- --conn-type grpc <COMMAND>
```

### Server Configuration

Ensure your TimesMan server is configured with gRPC support:

```toml
# timesman-server/config.toml
listen = "127.0.0.1:8080"
front_type = "Grpc"  # Enable gRPC frontend

[store]
type = "Local"  # or "Memory", "Json"
path = "~/Library/Application Support/timesman/unqlite.db"
```

## Development

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd timesman

# Build the tools
cargo build -p timesman-tools

# Run tests
cargo test -p timesman-tools
```

### Adding New Commands

The CLI is built with a modular architecture. To add new commands:

1. Add the command variant to the `Command` enum in `src/main.rs`
2. Implement the corresponding method in the `Client` trait
3. Add the gRPC implementation in `src/grpc.rs`
4. Update the `run_command` function to handle the new command

## License

This project is licensed under the same terms as the TimesMan project.