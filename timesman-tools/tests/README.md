# TimesMan Tools Test Suite

This directory contains comprehensive tests for the TimesMan CLI tool.

## Test Structure

### Unit Tests (`src/tests.rs`)
- **MockClient**: A complete mock implementation of the `Client` trait for testing
- **Client trait tests**: Tests all CRUD operations for Times and Posts
- **Command execution tests**: Tests the `run_command` function with various scenarios
- **Error handling tests**: Verifies proper error propagation and handling

### Integration Tests (`tests/integration_tests.rs`)
- **CLI binary tests**: Tests the actual command-line interface using `assert_cmd`
- **Argument validation**: Ensures proper validation of required and optional arguments
- **Help system tests**: Verifies help text and usage information
- **Error message tests**: Confirms appropriate error messages for invalid input

### Argument Parsing Tests (`tests/cli_parsing_tests.rs`)
- **Command parsing**: Tests all command variants and their arguments
- **Flag validation**: Tests both short and long form flags
- **Data validation**: Tests edge cases like Unicode, special characters, and numeric limits
- **Error cases**: Tests invalid arguments and missing required parameters

## Test Coverage

### Commands Tested
- ✅ `get-times-list` - List all time entries
- ✅ `create-times` - Create new time entry
- ✅ `delete-times` - Delete time entry
- ✅ `update-times` - Update time entry
- ✅ `get-post-list` - List posts for time entry
- ✅ `create-post` - Create new post
- ✅ `delete-post` - Delete post
- ✅ `update-post` - Update post

### Scenarios Tested
- ✅ Successful operations
- ✅ Error handling (connection failures, invalid IDs)
- ✅ Argument validation
- ✅ Help text and usage
- ✅ Edge cases (Unicode, large numbers, empty strings)
- ✅ Mock client functionality

## Running Tests

```bash
# Run all tests
cargo test -p timesman-tools

# Run only unit tests
cargo test -p timesman-tools --lib

# Run only integration tests  
cargo test -p timesman-tools --test integration_tests

# Run with verbose output
cargo test -p timesman-tools -- --nocapture

# Run specific test
cargo test -p timesman-tools test_mock_client_create_times
```

## Test Dependencies

- `assert_cmd` - CLI testing framework
- `predicates` - Assertion helpers for CLI output
- `mockall` - Mock generation (available but not currently used)
- `tokio-test` - Async testing utilities
- `tempfile` - Temporary file management

## Mock Client

The `MockClient` provides a fully functional in-memory implementation of the `Client` trait:

- Maintains separate storage for Times and Posts
- Implements proper ID generation and validation
- Supports error injection for testing failure scenarios
- Provides sample data generation for testing

### Usage Example

```rust
// Create empty mock client
let client = MockClient::new();

// Create client with sample data
let client = MockClient::new().with_sample_data();

// Create client that returns errors
let client = MockClient::new().with_error("Connection failed");
```

## Continuous Integration

These tests are designed to run in CI environments and provide comprehensive coverage of:

1. **Functional correctness** - All operations work as expected
2. **Error handling** - Failures are handled gracefully
3. **CLI interface** - Command-line parsing and validation
4. **Edge cases** - Boundary conditions and unusual input

The test suite achieves high coverage while maintaining fast execution times suitable for development workflows.