# Todo Detail Functionality Implementation Task List

## Overview
Implementation of rich todo detail functionality to enhance the basic todo system with explanations, background information, links, and other descriptive content.

## Task List

### 1. Data Structure Enhancement ✅ (In Progress)
**Location**: `timesman-type/src/lib.rs`
- [x] Add `detail` field to Todo struct (Optional<String>)
- [x] Update existing Todo tests with detail field
- [ ] Add new tests for todo detail functionality
- [ ] Test serialization/deserialization with detail field

### 2. Protocol Buffer Updates
**Location**: `timesman-grpc/proto/timesman.proto`
- [ ] Add `detail` field to Todo message
- [ ] Add new gRPC endpoints:
  - [ ] `GetTodoDetail(TodoDetailParams)` - Get full todo with detail
  - [ ] `UpdateTodoDetail(UpdateTodoDetailParams)` - Update todo detail
- [ ] Define new message types:
  - [ ] `TodoDetailParams` (tid, tdid)
  - [ ] `UpdateTodoDetailParams` (tid, tdid, detail)

### 3. gRPC Type Conversions
**Location**: `timesman-grpc/src/lib.rs`
- [ ] Update Todo Into/From conversions to handle detail field
- [ ] Ensure backward compatibility for todos without details

### 4. Server Implementation
**Location**: `timesman-server/src/grpc.rs`
- [ ] Implement `GetTodoDetail` endpoint
- [ ] Implement `UpdateTodoDetail` endpoint
- [ ] Update existing todo operations to handle detail field
- [ ] Ensure proper error handling for missing todos/details

### 5. Client Interface Enhancement
**Location**: `timesman-tools/src/main.rs`
- [ ] Update Client trait with todo detail methods:
  - [ ] `get_todo_detail(tid: u64, tdid: u64) -> Result<Todo, String>`
  - [ ] `update_todo_detail(tid: u64, tdid: u64, detail: String) -> Result<Todo, String>`
  - [ ] `create_todo_with_detail(tid: u64, content: String, detail: Option<String>) -> Result<Todo, String>`

### 6. gRPC Client Implementation
**Location**: `timesman-tools/src/grpc.rs`
- [ ] Implement `get_todo_detail` method
- [ ] Implement `update_todo_detail` method
- [ ] Update `create_todo` to support detail parameter
- [ ] Add proper error handling and gRPC conversions

### 7. Mock Client Updates
**Location**: `timesman-tools/src/mock_client.rs`
- [ ] Add todo detail support to MockClient
- [ ] Implement mock versions of detail methods
- [ ] Update sample data to include todos with details
- [ ] Ensure test compatibility

### 8. CLI Commands Enhancement
**Location**: `timesman-tools/src/main.rs`
- [ ] Add new CLI commands:
  - [ ] `get-todo-detail --tid <id> --tdid <todo_id>`
  - [ ] `update-todo-detail --tid <id> --tdid <todo_id> --detail <text>`
  - [ ] `create-todo-with-detail --tid <id> --content <summary> --detail <detail>`
- [ ] Update existing todo commands to show detail when available
- [ ] Add proper argument parsing and validation

### 9. TUI Application Enhancement
**Location**: `timesman-tools/src/tui/app.rs`
- [ ] Add new TUI modes:
  - [ ] `TodoDetail` - View todo detail
  - [ ] `EditTodoDetail` - Edit todo detail
- [ ] Add todo detail handling methods
- [ ] Update keyboard navigation for todo details
- [ ] Add todo detail input handling

### 10. TUI Interface Rendering
**Location**: `timesman-tools/src/tui/ui.rs`
- [ ] Add todo detail view rendering
- [ ] Support multi-line text display for details
- [ ] Add detail editing interface
- [ ] Update help system with todo detail shortcuts
- [ ] Add visual indicators for todos with details

### 11. Test Coverage
**Files**: Various test files
- [ ] Unit tests for todo detail data structures
- [ ] gRPC conversion tests with detail field
- [ ] CLI command tests for todo detail operations
- [ ] TUI interaction tests for todo details
- [ ] Integration tests for end-to-end todo detail workflow
- [ ] Mock client tests for detail functionality

### 12. Documentation Updates
- [ ] Update README with todo detail functionality
- [ ] Add examples of todo detail usage
- [ ] Document new CLI commands and keyboard shortcuts
- [ ] Update TUI documentation with detail features

## Implementation Notes

### Design Decisions
- **Backward Compatibility**: `detail` field is Optional to support existing todos
- **Separation of Concerns**: Keep `content` as summary, `detail` as rich description
- **User Experience**: Integrate seamlessly into existing CLI and TUI workflows

### Data Flow
1. User creates/updates todo with detail via CLI/TUI
2. Client sends gRPC request with detail data
3. Server stores todo with detail in backend
4. Client retrieves and displays todo with detail formatting

### Future Enhancements
- Rich text formatting (Markdown support)
- Todo detail templates
- Link validation and preview
- Todo detail history/versioning
- File attachments to todo details

## Status Legend
- ✅ Completed
- 🔄 In Progress  
- ⏳ Pending
- ❌ Blocked

## Current Status: ✅ Core Implementation Complete - Team Development Phase

## Team Task Assignments

### 👨‍💻 dev1 - Backend & Client Implementation
**Status**: ✅ **COMPLETED**  
**Assignment**: Complete backend infrastructure and client support  
**Details**: See [dev1.md](dev1.md)

**Completed Tasks**:
- ✅ Data Structure Enhancement (Todo with detail field)
- ✅ Protocol Buffer Updates (gRPC endpoints)
- ✅ Server Implementation (GetTodoDetail, UpdateTodoDetail)
- ✅ Client Trait Enhancement (all todo methods)
- ✅ gRPC Client Implementation (full functionality)
- ✅ MockClient Implementation (test support)
- ✅ CLI Commands (complete command set)

### 👨‍💻 dev2 - TUI Enhancement
**Status**: 🔄 **ASSIGNED & READY**  
**Assignment**: Enhance TUI interface for todo detail functionality  
**Details**: See [dev2.md](dev2.md)

**Assigned Tasks**:
- ⏳ TUI App State Management (todo modes)
- ⏳ Todo Detail View Implementation
- ⏳ Multi-line Text Editing Widget
- ⏳ Enhanced Keyboard Navigation
- ⏳ Todo List Enhancement (detail indicators)
- ⏳ Integration with Existing TUI Workflow

### 👨‍💻 dev3 - Testing & Documentation
**Status**: 🔄 **ASSIGNED & READY**  
**Assignment**: Comprehensive testing and documentation  
**Details**: See [dev3.md](dev3.md)

**Assigned Tasks**:
- ⏳ Unit Tests (data structures, conversions)
- ⏳ Integration Tests (server endpoints, CLI)
- ⏳ TUI Tests (after dev2 completion)
- ⏳ Performance Tests & Benchmarks
- ⏳ API Documentation
- ⏳ CLI Usage Guide
- ⏳ TUI User Manual
- ⏳ Developer Documentation

## Implementation Architecture Overview

### ✅ Completed Infrastructure
1. **Data Layer**: Todo struct with optional detail field
2. **Protocol Layer**: gRPC with TodoDetailParams, UpdateTodoDetailParams
3. **Server Layer**: Full todo detail endpoint implementation
4. **Client Layer**: Complete client trait and gRPC implementation
5. **CLI Layer**: All todo detail commands implemented
6. **Mock Layer**: Full test support with sample data

### 🔄 In Development
1. **TUI Layer**: Enhanced interface for todo details (dev2)
2. **Test Layer**: Comprehensive test coverage (dev3)
3. **Documentation**: Complete user and developer guides (dev3)