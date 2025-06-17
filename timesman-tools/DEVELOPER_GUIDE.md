# TimesMan Developer Guide

This comprehensive guide provides developers with the knowledge needed to understand, extend, and contribute to the TimesMan todo detail functionality.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Code Organization](#code-organization)
- [Design Patterns](#design-patterns)
- [Extension Points](#extension-points)
- [Development Workflow](#development-workflow)
- [Testing Strategy](#testing-strategy)
- [Contributing Guidelines](#contributing-guidelines)
- [Advanced Topics](#advanced-topics)

## Architecture Overview

TimesMan follows a modular architecture with clear separation of concerns across multiple crates in a Rust workspace.

### High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   timesman-app  │    │ timesman-tools  │    │timesman-server  │
│   (GUI Client)  │    │  (CLI Client)   │    │  (gRPC Server)  │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────┴───────────┐
                    │    timesman-grpc        │
                    │ (Service Definitions)   │
                    └─────────────┬───────────┘
                                  │
                         ┌────────┴────────┐
                         │  timesman-type  │
                         │  (Data Types)   │
                         └────────┬────────┘
                                  │
                         ┌────────┴────────┐
                         │ timesman-bstore │
                         │ (Storage Layer) │
                         └─────────────────┘
```

### Todo Detail Architecture

The todo detail functionality is implemented across several layers:

#### 1. Data Layer (`timesman-type`)
- **Core Type**: `Todo` struct with optional `detail` field
- **Serialization**: Serde support for JSON/MessagePack
- **Validation**: Input validation and sanitization
- **Backwards Compatibility**: Graceful handling of todos without details

#### 2. Storage Layer (`timesman-bstore`)
- **Trait-based Design**: `TodoStore` trait for storage abstraction
- **Multiple Backends**: Memory, Local (UnQLite), JSON file, gRPC remote
- **Feature Gates**: Conditional compilation for different storage backends
- **CRUD Operations**: Create, Read, Update, Delete with detail support

#### 3. gRPC Layer (`timesman-grpc`)
- **Service Definition**: Protocol buffer definitions in `timesman.proto`
- **Message Types**: `Todo`, `CreateTodoParams`, `UpdateTodoDetailParams`
- **Conversion Logic**: Rust types ↔ Protocol buffer types
- **Error Handling**: gRPC status codes and error messages

#### 4. Server Layer (`timesman-server`)
- **gRPC Implementation**: Server-side endpoint implementations
- **Business Logic**: Validation, authorization, data transformation
- **Error Mapping**: Domain errors to gRPC status codes
- **Async Support**: Tokio-based async/await patterns

#### 5. Client Layer (`timesman-tools`)
- **CLI Interface**: Command-line argument parsing with Clap
- **gRPC Client**: Tonic-based gRPC client implementation
- **TUI Interface**: Ratatui-based terminal user interface
- **Mock Client**: Testing utilities and mock implementations

## Code Organization

### Directory Structure

```
timesman-tools/
├── src/
│   ├── main.rs              # CLI entry point and command parsing
│   ├── grpc.rs             # gRPC client implementation
│   ├── mock_client.rs      # Mock client for testing
│   ├── tests.rs            # Unit and integration tests
│   └── tui/                # TUI implementation (by dev2)
├── tests/
│   └── integration_tests.rs # CLI integration tests
├── API_DOCUMENTATION.md     # gRPC API reference
├── CLI_USAGE.md            # User guide for CLI
└── DEVELOPER_GUIDE.md      # This file
```

### Key Source Files

#### `main.rs` - CLI Entry Point
```rust
// Command parsing with Clap derive API
#[derive(Parser)]
#[command(name = "timesman-tools")]
pub struct Cli {
    #[command(flatten)]
    global_opts: GlobalOptions,
    #[command(subcommand)]
    command: Commands,
}

// Todo detail commands
#[derive(Subcommand)]
enum Commands {
    CreateTodoWithDetail {
        #[arg(short, long)]
        tid: u64,
        #[arg(short, long)]
        content: String,
        #[arg(short, long)]
        detail: String,
    },
    GetTodoDetail {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
    },
    // ... other commands
}
```

#### `grpc.rs` - gRPC Client Implementation
```rust
// Client trait implementation for gRPC
impl Client for GrpcClient {
    async fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String> {
        let request = TodoDetailParams { tid, tdid };
        let response = self.client.get_todo_detail(request).await
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        // Convert gRPC response to domain type
        convert_grpc_todo(response.into_inner())
    }
    
    async fn update_todo_detail(&mut self, tid: u64, tdid: u64, detail: String) -> Result<Todo, String> {
        let request = UpdateTodoDetailParams { tid, tdid, detail };
        let response = self.client.update_todo_detail(request).await
            .map_err(|e| format!("gRPC error: {}", e))?;
            
        convert_grpc_todo(response.into_inner())
    }
}
```

#### `mock_client.rs` - Testing Utilities
```rust
// Mock implementation for testing
pub struct MockClient {
    times_data: Vec<Times>,
    todo_data: HashMap<u64, Vec<Todo>>,
    next_times_id: u64,
    next_todo_id: u64,
}

impl MockClient {
    pub fn with_sample_data(mut self) -> Self {
        // Pre-populate with test data including detailed todos
        self.add_sample_todos_with_details();
        self
    }
    
    fn add_sample_todos_with_details(&mut self) {
        let todos = vec![
            Todo {
                id: 1,
                content: "Implement authentication".to_string(),
                detail: Some("Complete OAuth 2.0 integration with Google".to_string()),
                created_at: Utc::now().naive_utc(),
                done_at: None,
            },
            // ... more sample data
        ];
        self.todo_data.insert(1, todos);
    }
}
```

## Design Patterns

### 1. Trait-Based Architecture

The codebase uses traits extensively for abstraction and testability:

```rust
// Client trait abstracts different client implementations
#[async_trait]
pub trait Client {
    async fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String>;
    async fn update_todo_detail(&mut self, tid: u64, tdid: u64, detail: String) -> Result<Todo, String>;
    async fn create_todo_with_detail(&mut self, tid: u64, content: String, detail: String) -> Result<Todo, String>;
    // ... other methods
}

// Multiple implementations: GrpcClient, MockClient, HttpClient (future)
impl Client for GrpcClient { /* gRPC implementation */ }
impl Client for MockClient { /* Mock implementation */ }
```

### 2. Error Handling Pattern

Consistent error handling across the codebase:

```rust
// Custom error types for different layers
#[derive(Debug, thiserror::Error)]
pub enum TodoError {
    #[error("Todo not found: {id}")]
    NotFound { id: u64 },
    
    #[error("Invalid detail length: {length}, max: {max}")]
    DetailTooLong { length: usize, max: usize },
    
    #[error("Storage error: {source}")]
    Storage { #[from] source: StorageError },
}

// Error conversion for different contexts
impl From<TodoError> for tonic::Status {
    fn from(err: TodoError) -> Self {
        match err {
            TodoError::NotFound { .. } => Status::not_found(err.to_string()),
            TodoError::DetailTooLong { .. } => Status::invalid_argument(err.to_string()),
            TodoError::Storage { .. } => Status::internal(err.to_string()),
        }
    }
}
```

### 3. Builder Pattern for Test Data

Flexible test data creation:

```rust
// Test data builder
pub struct TodoBuilder {
    id: u64,
    content: String,
    detail: Option<String>,
    created_at: NaiveDateTime,
    done_at: Option<NaiveDateTime>,
}

impl TodoBuilder {
    pub fn new() -> Self {
        Self {
            id: 1,
            content: "Default task".to_string(),
            detail: None,
            created_at: Utc::now().naive_utc(),
            done_at: None,
        }
    }
    
    pub fn with_detail<S: Into<String>>(mut self, detail: S) -> Self {
        self.detail = Some(detail.into());
        self
    }
    
    pub fn with_unicode_detail(mut self) -> Self {
        self.detail = Some("Unicode test: 🚀 ñáéíóú 中文 العربية".to_string());
        self
    }
    
    pub fn build(self) -> Todo {
        Todo {
            id: self.id,
            content: self.content,
            detail: self.detail,
            created_at: self.created_at,
            done_at: self.done_at,
        }
    }
}

// Usage in tests
let todo = TodoBuilder::new()
    .with_detail("Detailed description")
    .build();
```

### 4. Command Pattern for CLI

Clean separation of command parsing and execution:

```rust
// Command execution trait
trait CommandExecutor {
    async fn execute(&self, client: &mut dyn Client) -> Result<(), String>;
}

// Individual command implementations
struct CreateTodoWithDetailCommand {
    tid: u64,
    content: String,
    detail: String,
}

impl CommandExecutor for CreateTodoWithDetailCommand {
    async fn execute(&self, client: &mut dyn Client) -> Result<(), String> {
        let todo = client.create_todo_with_detail(self.tid, self.content.clone(), self.detail.clone()).await?;
        println!("Created todo: {}", format_todo(&todo));
        Ok(())
    }
}
```

## Extension Points

### 1. Adding New Storage Backends

To add a new storage backend:

1. **Implement Storage Traits** in `timesman-bstore`:
```rust
// Implement the core storage traits
impl Store for YourStorageBackend {
    // Implement required methods
}

impl TodoStore for YourStorageBackend {
    async fn get_todo_detail(&self, tid: u64, tdid: u64) -> Result<Todo, StorageError> {
        // Your implementation
    }
    
    async fn update_todo_detail(&self, tid: u64, tdid: u64, detail: String) -> Result<Todo, StorageError> {
        // Your implementation
    }
}
```

2. **Add Feature Gate** in `Cargo.toml`:
```toml
[features]
your-backend = ["dep:your-backend-crate"]
```

3. **Register Backend** in server configuration:
```rust
#[cfg(feature = "your-backend")]
fn create_your_backend_store(config: &Config) -> Result<YourStorageBackend, Error> {
    YourStorageBackend::new(&config.your_backend)
}
```

### 2. Adding New Client Types

To add a new client implementation (e.g., HTTP client):

1. **Implement Client Trait**:
```rust
pub struct HttpClient {
    base_url: String,
    client: reqwest::Client,
}

#[async_trait]
impl Client for HttpClient {
    async fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String> {
        let url = format!("{}/api/times/{}/todos/{}", self.base_url, tid, tdid);
        let response = self.client.get(&url).send().await
            .map_err(|e| format!("HTTP error: {}", e))?;
        // Parse response and return Todo
    }
}
```

2. **Add to CLI Options**:
```rust
#[derive(ValueEnum, Clone)]
enum ConnectionType {
    Grpc,
    Http,  // New option
}
```

### 3. Extending Todo Detail Features

To add new todo detail features:

1. **Extend Data Type** in `timesman-type`:
```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub content: String,
    pub detail: Option<String>,
    pub tags: Vec<String>,        // New field
    pub priority: Priority,       // New field
    pub attachments: Vec<File>,   // New field
    pub created_at: NaiveDateTime,
    pub done_at: Option<NaiveDateTime>,
}
```

2. **Update gRPC Proto** in `timesman-grpc/proto/timesman.proto`:
```protobuf
message Todo {
    uint64 id = 1;
    string content = 2;
    optional string detail = 3;
    repeated string tags = 4;      // New field
    Priority priority = 5;         // New field
    repeated File attachments = 6; // New field
    google.protobuf.Timestamp created_at = 7;
    optional google.protobuf.Timestamp done_at = 8;
}
```

3. **Add Client Methods**:
```rust
#[async_trait]
pub trait Client {
    // Existing methods...
    
    async fn add_todo_tags(&mut self, tid: u64, tdid: u64, tags: Vec<String>) -> Result<Todo, String>;
    async fn set_todo_priority(&mut self, tid: u64, tdid: u64, priority: Priority) -> Result<Todo, String>;
    async fn attach_file(&mut self, tid: u64, tdid: u64, file: File) -> Result<Todo, String>;
}
```

### 4. Adding New CLI Commands

To add new CLI commands:

1. **Extend Commands Enum**:
```rust
#[derive(Subcommand)]
enum Commands {
    // Existing commands...
    
    AddTodoTags {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
    },
}
```

2. **Add Command Handler**:
```rust
Commands::AddTodoTags { tid, tdid, tags } => {
    let updated_todo = client.add_todo_tags(tid, tdid, tags).await?;
    println!("Updated todo: {}", format_todo(&updated_todo));
}
```

3. **Add Tests**:
```rust
#[test]
fn test_add_todo_tags_command() {
    let mut client = MockClient::new().with_sample_data();
    // Test the new command
}
```

## Development Workflow

### 1. Setting Up Development Environment

```bash
# Clone repository
git clone <repository-url>
cd timesman

# Install Rust toolchain
rustup update stable

# Install development dependencies
cargo install cargo-watch      # For auto-rebuild
cargo install cargo-tarpaulin  # For coverage
cargo install criterion        # For benchmarks

# Install protobuf compiler (for gRPC)
# On macOS:
brew install protobuf
# On Ubuntu:
sudo apt-get install protobuf-compiler

# Build all workspace members
cargo build --all-features
```

### 2. Running Tests

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test --lib todo_detail_tests
cargo test --test integration_tests

# Run tests with coverage
cargo tarpaulin --out Html

# Run performance tests
cargo bench
```

### 3. Development Server

```bash
# Terminal 1: Start server with hot reload
cargo watch -x 'run -p timesman-server'

# Terminal 2: Test CLI commands
cargo run -p timesman-tools -- --conn-type grpc get-times-list

# Terminal 3: Run tests on file changes
cargo watch -x test
```

### 4. Code Quality Checks

```bash
# Format code
cargo fmt

# Lint code
cargo clippy --all-features -- -D warnings

# Check for security vulnerabilities
cargo audit

# Check for outdated dependencies
cargo outdated
```

## Testing Strategy

### 1. Test Pyramid

The project follows a comprehensive testing strategy:

```
                    ┌─────────────────┐
                    │   E2E Tests     │  (Few, High-level)
                    │  (CLI + Server) │
                    └─────────────────┘
                  ┌───────────────────────┐
                  │  Integration Tests    │  (Some, Component-level)
                  │  (gRPC, Storage)      │
                  └───────────────────────┘
              ┌─────────────────────────────────┐
              │        Unit Tests               │  (Many, Function-level)
              │  (Data types, Conversions)      │
              └─────────────────────────────────┘
```

### 2. Test Categories

#### Unit Tests (`src/tests.rs`)
- **Data Structure Tests**: Serialization, validation, equality
- **Conversion Tests**: gRPC ↔ Domain type conversions
- **Mock Client Tests**: Business logic without network
- **Performance Tests**: Benchmarking critical operations

#### Integration Tests (`tests/integration_tests.rs`)
- **CLI Integration**: Command parsing and execution
- **gRPC Integration**: End-to-end gRPC communication
- **Storage Integration**: Database operations
- **Error Handling**: Error propagation and formatting

#### End-to-End Tests
- **Workflow Tests**: Complete user scenarios
- **Compatibility Tests**: Backward compatibility validation
- **Load Tests**: High-concurrency scenarios

### 3. Test Data Management

```rust
// Centralized test data creation
pub struct TestDataFactory;

impl TestDataFactory {
    pub fn create_todo_with_detail() -> Todo {
        Todo {
            id: 1,
            content: "Test task".to_string(),
            detail: Some("Detailed description".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            done_at: None,
        }
    }
    
    pub fn create_unicode_todo() -> Todo {
        Todo {
            id: 2,
            content: "Unicode test".to_string(),
            detail: Some("🚀 测试 العربية ñáéíóú".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            done_at: None,
        }
    }
    
    pub fn create_large_detail_todo() -> Todo {
        let large_detail = "x".repeat(50000); // 50KB detail
        Todo {
            id: 3,
            content: "Large detail test".to_string(),
            detail: Some(large_detail),
            created_at: chrono::Utc::now().naive_utc(),
            done_at: None,
        }
    }
}
```

### 4. Performance Testing

```rust
// Benchmark example
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_todo_detail_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("todo_detail");
    
    for size in [100, 1000, 10000].iter() {
        let detail = "x".repeat(*size);
        
        group.bench_with_input(
            BenchmarkId::new("serialization", size),
            &detail,
            |b, detail| {
                let todo = Todo {
                    id: 1,
                    content: "Test".to_string(),
                    detail: Some(detail.clone()),
                    created_at: chrono::Utc::now().naive_utc(),
                    done_at: None,
                };
                
                b.iter(|| {
                    serde_json::to_string(&todo).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_todo_detail_operations);
criterion_main!(benches);
```

## Contributing Guidelines

### 1. Code Style

Follow Rust community standards:

```rust
// Use descriptive names
fn create_todo_with_detail(tid: u64, content: String, detail: String) -> Result<Todo, TodoError> {
    // Implementation
}

// Prefer explicit error handling
match client.get_todo_detail(tid, tdid).await {
    Ok(todo) => println!("Todo: {}", todo.content),
    Err(e) => eprintln!("Error: {}", e),
}

// Use appropriate visibility
pub struct Todo {           // Public API
    pub id: u64,
    pub content: String,
    pub(crate) detail: Option<String>, // Internal API
}

// Document public APIs
/// Retrieves a todo with its detailed description.
/// 
/// # Arguments
/// * `tid` - The times entry ID
/// * `tdid` - The todo ID
/// 
/// # Returns
/// The todo with its detail, or an error if not found
/// 
/// # Examples
/// ```
/// let todo = client.get_todo_detail(1, 5).await?;
/// println!("Detail: {}", todo.detail.unwrap_or_default());
/// ```
pub async fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String>;
```

### 2. Commit Message Format

Use conventional commit format:

```
type(scope): brief description

Detailed explanation of the change, its motivation, and impact.

Fixes #123
Co-authored-by: Name <email>
```

Examples:
- `feat(cli): add get-todo-detail command`
- `fix(grpc): handle empty detail field correctly`
- `test(integration): add Unicode support tests`
- `docs(api): update todo detail endpoint examples`

### 3. Pull Request Process

1. **Create Feature Branch**:
```bash
git checkout -b feature/todo-detail-attachments
```

2. **Make Changes with Tests**:
```bash
# Implement feature
# Add comprehensive tests
cargo test

# Ensure code quality
cargo fmt
cargo clippy
```

3. **Update Documentation**:
```bash
# Update relevant .md files
# Add code examples
# Update API documentation
```

4. **Submit Pull Request**:
- Clear title and description
- Link to related issues
- Include testing evidence
- Request specific reviewers

### 4. Code Review Checklist

#### For Authors:
- [ ] All tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Performance impact considered
- [ ] Backward compatibility maintained
- [ ] Error handling comprehensive

#### For Reviewers:
- [ ] Logic is correct and efficient
- [ ] Tests cover edge cases
- [ ] API design is intuitive
- [ ] Security implications considered
- [ ] Documentation is clear
- [ ] Code is maintainable

## Advanced Topics

### 1. Async/Await Patterns

The codebase uses Tokio for async programming:

```rust
// Proper async client implementation
impl Client for GrpcClient {
    async fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String> {
        // Use ? for early return on errors
        let request = TodoDetailParams { tid, tdid };
        
        // Await gRPC call
        let response = self.client
            .get_todo_detail(Request::new(request))
            .await
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        // Convert and return
        convert_grpc_todo(response.into_inner())
    }
}

// Async testing patterns
#[tokio::test]
async fn test_concurrent_todo_operations() {
    let mut client = MockClient::new();
    
    // Test concurrent operations
    let tasks = (1..=10).map(|i| {
        let content = format!("Task {}", i);
        let detail = format!("Detail for task {}", i);
        client.create_todo_with_detail(1, content, detail)
    });
    
    let results = futures::future::join_all(tasks).await;
    assert_eq!(results.len(), 10);
}
```

### 2. Error Handling Strategies

Comprehensive error handling across layers:

```rust
// Domain-specific error types
#[derive(Debug, thiserror::Error)]
pub enum TodoDetailError {
    #[error("Detail too long: {length} characters (max: {max})")]
    DetailTooLong { length: usize, max: usize },
    
    #[error("Invalid UTF-8 in detail field")]
    InvalidEncoding,
    
    #[error("Todo {tdid} not found in times {tid}")]
    TodoNotFound { tid: u64, tdid: u64 },
    
    #[error("Storage backend unavailable: {reason}")]
    StorageUnavailable { reason: String },
}

// Error conversion chain
impl From<TodoDetailError> for tonic::Status {
    fn from(err: TodoDetailError) -> Self {
        use TodoDetailError::*;
        match err {
            DetailTooLong { .. } => Status::invalid_argument(err.to_string()),
            InvalidEncoding => Status::invalid_argument(err.to_string()),
            TodoNotFound { .. } => Status::not_found(err.to_string()),
            StorageUnavailable { .. } => Status::unavailable(err.to_string()),
        }
    }
}

// Client-side error handling
async fn handle_todo_operation() -> Result<(), Box<dyn std::error::Error>> {
    match client.get_todo_detail(1, 5).await {
        Ok(todo) => {
            println!("Found todo: {}", todo.content);
            if let Some(detail) = todo.detail {
                println!("Detail: {}", detail);
            }
        }
        Err(e) if e.contains("not found") => {
            eprintln!("Todo doesn't exist, create it first");
            return Ok(()); // Recoverable error
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            return Err(e.into()); // Propagate serious errors
        }
    }
    Ok(())
}
```

### 3. Memory Management and Performance

Optimize for performance-critical paths:

```rust
// Efficient string handling
impl Todo {
    // Use Cow for potentially owned/borrowed strings
    pub fn get_display_detail(&self) -> std::borrow::Cow<str> {
        match &self.detail {
            Some(detail) if detail.len() > 100 => {
                // Truncate long details for display
                format!("{}...", &detail[..97]).into()
            }
            Some(detail) => detail.into(),
            None => "No detail".into(),
        }
    }
    
    // Avoid unnecessary clones
    pub fn has_detail(&self) -> bool {
        self.detail.is_some()
    }
    
    pub fn detail_length(&self) -> usize {
        self.detail.as_ref().map_or(0, |d| d.len())
    }
}

// Memory-efficient batch operations
impl MockClient {
    pub async fn get_todos_batch(&self, requests: &[(u64, u64)]) -> Vec<Result<Todo, String>> {
        // Pre-allocate result vector
        let mut results = Vec::with_capacity(requests.len());
        
        for &(tid, tdid) in requests {
            results.push(self.get_todo_detail(tid, tdid).await);
        }
        
        results
    }
}
```

### 4. Internationalization Support

Unicode and internationalization considerations:

```rust
// Unicode validation and normalization
use unicode_normalization::UnicodeNormalization;

impl Todo {
    pub fn set_detail(&mut self, detail: String) -> Result<(), TodoDetailError> {
        // Normalize Unicode
        let normalized: String = detail.nfc().collect();
        
        // Validate length (consider grapheme clusters, not bytes)
        use unicode_segmentation::UnicodeSegmentation;
        let grapheme_count = normalized.graphemes(true).count();
        
        if grapheme_count > MAX_DETAIL_GRAPHEMES {
            return Err(TodoDetailError::DetailTooLong {
                length: grapheme_count,
                max: MAX_DETAIL_GRAPHEMES,
            });
        }
        
        // Check for control characters
        if normalized.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
            return Err(TodoDetailError::InvalidCharacters);
        }
        
        self.detail = Some(normalized);
        Ok(())
    }
}

// Testing with various languages
#[cfg(test)]
mod unicode_tests {
    use super::*;
    
    #[test]
    fn test_todo_detail_multilingual() {
        let test_cases = vec![
            ("English", "Basic Latin text with punctuation!"),
            ("Japanese", "日本語のテキストです。"),
            ("Arabic", "هذا نص باللغة العربية"),
            ("Emoji", "Task with emojis: 🚀 ✅ 📝 💡"),
            ("Mixed", "Mixed: Hello 世界 مرحبا 🌍"),
        ];
        
        for (language, text) in test_cases {
            let mut todo = Todo::default();
            assert!(todo.set_detail(text.to_string()).is_ok(), 
                   "Failed to set {} detail", language);
            
            // Test serialization roundtrip
            let json = serde_json::to_string(&todo).unwrap();
            let deserialized: Todo = serde_json::from_str(&json).unwrap();
            assert_eq!(todo.detail, deserialized.detail);
        }
    }
}
```

## Conclusion

This developer guide provides the foundation for understanding and extending the TimesMan todo detail functionality. The modular architecture, comprehensive testing strategy, and clear patterns make it easy to add new features and maintain existing code.

Key principles to remember:
- **Modularity**: Keep concerns separated across crates
- **Testability**: Write tests first, use dependency injection
- **Performance**: Profile before optimizing, handle Unicode correctly
- **Reliability**: Comprehensive error handling and validation
- **Maintainability**: Clear code, good documentation, consistent patterns

For questions or contributions, please refer to the project's issue tracker and follow the contributing guidelines outlined above.

---

**Last Updated**: December 2023  
**Version**: 1.0.0  
**Authors**: dev3 (primary), TimesMan development team