# TimesMan Todo Detail API Documentation

This document provides comprehensive documentation for the TimesMan gRPC API todo detail functionality, including endpoints, request/response formats, and examples.

## Table of Contents

- [Overview](#overview)
- [gRPC Service Definition](#grpc-service-definition)
- [Todo Detail Endpoints](#todo-detail-endpoints)
- [Data Types](#data-types)
- [Error Handling](#error-handling)
- [Authentication and Authorization](#authentication-and-authorization)
- [Examples](#examples)

## Overview

The TimesMan gRPC API provides comprehensive todo management capabilities with detailed descriptions. The API supports creating, reading, updating, and managing todos with optional detailed descriptions that can contain multi-line text, Unicode characters, and rich formatting.

### Base URL
Default server: `http://127.0.0.1:8080/`

### Protocol
gRPC over HTTP/2

## gRPC Service Definition

The todo detail functionality is part of the `TimesMan` service defined in `timesman.proto`.

```protobuf
service TimesMan {
    // Todo operations
    rpc GetTodos(TimesId) returns (TodoArray);
    rpc CreateTodo(CreateTodoParams) returns (Todo);
    rpc DoneTodo(DoneTodoParams) returns (Todo);
    
    // Todo detail operations
    rpc GetTodoDetail(TodoDetailParams) returns (Todo);
    rpc UpdateTodoDetail(UpdateTodoDetailParams) returns (Todo);
}
```

## Todo Detail Endpoints

### 1. Get Todos List

Retrieves all todos for a specific times entry, including their details.

**Endpoint:** `GetTodos`

**Request:**
```protobuf
message TimesId {
    uint64 id = 1;
}
```

**Response:**
```protobuf
message TodoArray {
    repeated Todo todos = 1;
}
```

**Example Request:**
```json
{
    "id": 1
}
```

**Example Response:**
```json
{
    "todos": [
        {
            "id": 1,
            "content": "Complete documentation",
            "detail": "Write comprehensive API documentation including examples and error handling",
            "created_at": {"seconds": 1671234567, "nanos": 0},
            "done_at": null
        },
        {
            "id": 2,
            "content": "Review code",
            "detail": null,
            "created_at": {"seconds": 1671234600, "nanos": 0},
            "done_at": {"seconds": 1671234800, "nanos": 0}
        }
    ]
}
```

### 2. Create Todo (with optional detail)

Creates a new todo with optional detailed description.

**Endpoint:** `CreateTodo`

**Request:**
```protobuf
message CreateTodoParams {
    uint64 tid = 1;
    string content = 2;
    optional string detail = 3;
}
```

**Response:**
```protobuf
message Todo {
    uint64 id = 1;
    string content = 2;
    optional string detail = 3;
    google.protobuf.Timestamp created_at = 4;
    optional google.protobuf.Timestamp done_at = 5;
}
```

**Example Request (with detail):**
```json
{
    "tid": 1,
    "content": "Implement new feature",
    "detail": "Create a new todo detail feature that supports:\n- Multi-line descriptions\n- Unicode characters: 🚀 ñáéíóú\n- Rich formatting options"
}
```

**Example Request (without detail):**
```json
{
    "tid": 1,
    "content": "Simple task"
}
```

**Example Response:**
```json
{
    "id": 3,
    "content": "Implement new feature",
    "detail": "Create a new todo detail feature that supports:\n- Multi-line descriptions\n- Unicode characters: 🚀 ñáéíóú\n- Rich formatting options",
    "created_at": {"seconds": 1671234567, "nanos": 0},
    "done_at": null
}
```

### 3. Get Todo Detail

Retrieves a specific todo with its complete details.

**Endpoint:** `GetTodoDetail`

**Request:**
```protobuf
message TodoDetailParams {
    uint64 tid = 1;
    uint64 tdid = 2;
}
```

**Response:**
```protobuf
message Todo {
    uint64 id = 1;
    string content = 2;
    optional string detail = 3;
    google.protobuf.Timestamp created_at = 4;
    optional google.protobuf.Timestamp done_at = 5;
}
```

**Example Request:**
```json
{
    "tid": 1,
    "tdid": 3
}
```

**Example Response:**
```json
{
    "id": 3,
    "content": "Implement new feature",
    "detail": "Create a new todo detail feature that supports:\n- Multi-line descriptions\n- Unicode characters: 🚀 ñáéíóú\n- Rich formatting options",
    "created_at": {"seconds": 1671234567, "nanos": 0},
    "done_at": null
}
```

### 4. Update Todo Detail

Updates the detailed description of an existing todo.

**Endpoint:** `UpdateTodoDetail`

**Request:**
```protobuf
message UpdateTodoDetailParams {
    uint64 tid = 1;
    uint64 tdid = 2;
    string detail = 3;
}
```

**Response:**
```protobuf
message Todo {
    uint64 id = 1;
    string content = 2;
    optional string detail = 3;
    google.protobuf.Timestamp created_at = 4;
    optional google.protobuf.Timestamp done_at = 5;
}
```

**Example Request:**
```json
{
    "tid": 1,
    "tdid": 3,
    "detail": "Updated detailed description with new requirements:\n- Add error handling\n- Include unit tests\n- Update documentation"
}
```

**Example Response:**
```json
{
    "id": 3,
    "content": "Implement new feature",
    "detail": "Updated detailed description with new requirements:\n- Add error handling\n- Include unit tests\n- Update documentation",
    "created_at": {"seconds": 1671234567, "nanos": 0},
    "done_at": null
}
```

### 5. Mark Todo Done/Undone

Marks a todo as completed or pending while preserving its detail.

**Endpoint:** `DoneTodo`

**Request:**
```protobuf
message DoneTodoParams {
    uint64 tid = 1;
    uint64 tdid = 2;
    bool done = 3;
}
```

**Response:**
```protobuf
message Todo {
    uint64 id = 1;
    string content = 2;
    optional string detail = 3;
    google.protobuf.Timestamp created_at = 4;
    optional google.protobuf.Timestamp done_at = 5;
}
```

**Example Request (mark as done):**
```json
{
    "tid": 1,
    "tdid": 3,
    "done": true
}
```

**Example Request (mark as pending):**
```json
{
    "tid": 1,
    "tdid": 3,
    "done": false
}
```

**Example Response:**
```json
{
    "id": 3,
    "content": "Implement new feature",
    "detail": "Updated detailed description with new requirements:\n- Add error handling\n- Include unit tests\n- Update documentation",
    "created_at": {"seconds": 1671234567, "nanos": 0},
    "done_at": {"seconds": 1671237600, "nanos": 0}
}
```

## Data Types

### Todo

Represents a todo item with optional detailed description.

```protobuf
message Todo {
    uint64 id = 1;              // Unique identifier
    string content = 2;          // Brief description/title
    optional string detail = 3;  // Optional detailed description
    google.protobuf.Timestamp created_at = 4;  // Creation timestamp
    optional google.protobuf.Timestamp done_at = 5;  // Completion timestamp
}
```

**Field Descriptions:**

- **id**: Unique identifier for the todo (auto-generated)
- **content**: Brief description or title of the todo (required)
- **detail**: Optional detailed description supporting:
  - Multi-line text (newlines preserved)
  - Unicode characters (UTF-8 encoded)
  - Special characters and symbols
  - Maximum recommended length: 100,000 characters
- **created_at**: Timestamp when the todo was created
- **done_at**: Timestamp when the todo was marked as done (null if pending)

### TodoArray

Container for multiple todo items.

```protobuf
message TodoArray {
    repeated Todo todos = 1;
}
```

### CreateTodoParams

Parameters for creating a new todo.

```protobuf
message CreateTodoParams {
    uint64 tid = 1;             // Times ID
    string content = 2;          // Todo content
    optional string detail = 3;  // Optional detail
}
```

### TodoDetailParams

Parameters for retrieving a specific todo.

```protobuf
message TodoDetailParams {
    uint64 tid = 1;   // Times ID
    uint64 tdid = 2;  // Todo ID
}
```

### UpdateTodoDetailParams

Parameters for updating todo details.

```protobuf
message UpdateTodoDetailParams {
    uint64 tid = 1;     // Times ID
    uint64 tdid = 2;    // Todo ID
    string detail = 3;   // New detail content
}
```

### DoneTodoParams

Parameters for marking todo as done/undone.

```protobuf
message DoneTodoParams {
    uint64 tid = 1;   // Times ID
    uint64 tdid = 2;  // Todo ID
    bool done = 3;     // Done status
}
```

## Error Handling

The API uses standard gRPC status codes to indicate errors.

### Common Error Codes

| Status Code | Description | Common Causes |
|-------------|-------------|---------------|
| `OK` (0) | Success | Request completed successfully |
| `INVALID_ARGUMENT` (3) | Invalid parameters | Missing required fields, invalid IDs |
| `NOT_FOUND` (5) | Resource not found | Times ID or Todo ID doesn't exist |
| `ALREADY_EXISTS` (6) | Resource exists | Attempting to create duplicate |
| `FAILED_PRECONDITION` (9) | Precondition failed | Invalid state transition |
| `ABORTED` (10) | Operation aborted | Database error, concurrent modification |
| `INTERNAL` (13) | Internal error | Server-side error, database connection |
| `UNAVAILABLE` (14) | Service unavailable | Server overloaded, maintenance |

### Error Response Format

```json
{
    "error": {
        "code": 5,
        "message": "Times with id 999 not found",
        "details": []
    }
}
```

### Error Examples

**Times Not Found:**
```json
{
    "error": {
        "code": 5,
        "message": "Times with id 999 not found"
    }
}
```

**Todo Not Found:**
```json
{
    "error": {
        "code": 5,
        "message": "Todo with id 123 not found"
    }
}
```

**Invalid Parameters:**
```json
{
    "error": {
        "code": 3,
        "message": "Content is required"
    }
}
```

## Authentication and Authorization

### Current Implementation

The current API implementation does not include authentication or authorization mechanisms. All endpoints are publicly accessible.

### Security Considerations

For production deployments, consider implementing:

- **Authentication**: Bearer tokens, API keys, or OAuth 2.0
- **Authorization**: Role-based access control (RBAC)
- **Rate Limiting**: Prevent abuse and ensure fair usage
- **Input Validation**: Sanitize and validate all inputs
- **Audit Logging**: Track API usage and modifications

### Planned Features

Future versions may include:
- User authentication
- Per-user todo isolation
- Team/organization-based access control
- API key management

## Examples

### Complete Workflow Example

This example demonstrates a complete workflow for managing todos with details.

#### 1. Create a Times Entry (prerequisite)

```bash
# Using timesman-tools CLI
timesman-tools --conn-type grpc create-times --title "Project Alpha"
# Response: Created times with ID 1
```

#### 2. Create Todo with Detail

```bash
# Create a todo with detailed description
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Implement user authentication" \
    --detail "Design and implement a secure user authentication system:
- Support email/password login
- Implement JWT token management
- Add password reset functionality
- Include rate limiting for security
- Write comprehensive tests"
# Response: Created todo with ID 1
```

#### 3. Create Simple Todo

```bash
# Create a simple todo without detail
timesman-tools --conn-type grpc create-todo \
    --tid 1 \
    --content "Update README"
# Response: Created todo with ID 2
```

#### 4. List All Todos

```bash
# Get all todos for the times entry
timesman-tools --conn-type grpc get-todo-list --tid 1
# Response: Lists all todos with their details (if any)
```

#### 5. Get Specific Todo Detail

```bash
# Get detailed information for a specific todo
timesman-tools --conn-type grpc get-todo-detail --tid 1 --tdid 1
# Response: Shows full todo information including detail
```

#### 6. Update Todo Detail

```bash
# Update the detailed description
timesman-tools --conn-type grpc update-todo-detail \
    --tid 1 \
    --tdid 1 \
    --detail "Updated authentication implementation plan:
- Implement OAuth 2.0 integration
- Add multi-factor authentication (MFA)
- Include social login options (Google, GitHub)
- Implement session management
- Add comprehensive audit logging
- Write integration tests"
```

#### 7. Mark Todo as Done

```bash
# Mark todo as completed
timesman-tools --conn-type grpc mark-todo-done --tid 1 --tdid 1 --done
# Response: Todo marked as DONE with timestamp
```

#### 8. Mark Todo as Pending

```bash
# Mark todo as pending (undo completion)
timesman-tools --conn-type grpc mark-todo-undone --tid 1 --tdid 1
# Response: Todo marked as PENDING
```

### Unicode and Special Characters Example

The API fully supports Unicode and special characters in todo details:

```bash
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Internationalization support" \
    --detail "Add i18n support for multiple languages:
🌍 Supported languages:
- English (US/UK)
- Español (ES/MX)
- Français (FR/CA)
- 中文 (简体/繁體)
- العربية
- Русский
- Deutsch

🚀 Implementation tasks:
- Set up translation framework
- Extract translatable strings
- Create language files
- Test RTL languages (Arabic)
- Validate Unicode handling"
```

### Large Detail Example

The API supports large detailed descriptions:

```bash
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Database optimization" \
    --detail "Comprehensive database optimization project

## Phase 1: Analysis
- Profile current query performance
- Identify bottlenecks and slow queries
- Analyze table structure and relationships
- Review indexing strategy

## Phase 2: Optimization
- Add missing indexes
- Optimize slow queries
- Normalize database schema
- Implement query caching

## Phase 3: Testing
- Performance testing with realistic data
- Load testing under high concurrency
- Verify data integrity after changes
- Benchmark before/after performance

## Phase 4: Monitoring
- Set up query performance monitoring
- Create alerting for slow queries
- Implement automated optimization checks
- Document optimization guidelines

## Success Criteria
- 50% reduction in average query time
- 99.9% uptime during optimization
- No data loss or corruption
- Full test coverage for critical paths"
```

### Error Handling Example

Example of handling common errors:

```bash
# Attempt to get todo from non-existent times
timesman-tools --conn-type grpc get-todo-detail --tid 999 --tdid 1
# Error: gRPC error: status: NotFound, message: "Times with id 999 not found"

# Attempt to get non-existent todo
timesman-tools --conn-type grpc get-todo-detail --tid 1 --tdid 999
# Error: gRPC error: status: NotFound, message: "Todo with id 999 not found"

# Missing required parameter
timesman-tools --conn-type grpc create-todo --tid 1
# Error: Missing required argument: content
```

## Performance Considerations

### Recommended Limits

- **Detail Length**: While the API supports large details, recommended maximum is 100,000 characters
- **Concurrent Requests**: API handles high concurrency efficiently (tested up to 1000+ ops/sec)
- **Batch Operations**: For bulk operations, consider multiple parallel requests

### Optimization Tips

1. **Use Appropriate Endpoints**: Use `GetTodos` for lists, `GetTodoDetail` for individual items
2. **Limit Detail Size**: Large details may impact serialization performance
3. **Connection Reuse**: Reuse gRPC connections for multiple requests
4. **Error Handling**: Implement proper retry logic for transient failures

## Rate Limiting

Currently, there are no rate limits imposed by the API. However, for production use, consider implementing:

- Per-client rate limiting
- Burst capacity management
- Fair usage policies

## Changelog

### Version 1.0.0
- Initial implementation of todo detail functionality
- Support for create, read, update operations
- Unicode and multi-line text support
- Basic error handling

### Future Versions
- Authentication and authorization
- Batch operations
- Advanced search and filtering
- Webhook notifications
- API versioning

---

For additional support or questions, please refer to the [CLI Usage Documentation](CLI_USAGE.md) or the [Developer Guide](DEVELOPER_GUIDE.md).