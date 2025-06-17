# TimesMan CLI Usage Guide

This guide provides comprehensive documentation for using the TimesMan command-line interface to manage todos with detailed descriptions.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Command Overview](#command-overview)
- [Todo Detail Commands](#todo-detail-commands)
- [Examples and Use Cases](#examples-and-use-cases)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Installation

### Prerequisites

- Rust 1.70 or later
- A running TimesMan gRPC server

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd timesman

# Build the CLI tool
cargo build --release -p timesman-tools

# The binary will be available at:
# ./target/release/timesman-tools
```

### Using Pre-built Binaries

Download the appropriate binary for your platform from the releases page.

## Quick Start

### 1. Start the Server

```bash
# Start the TimesMan server (default port: 8080)
cargo run -p timesman-server
```

### 2. Basic Usage

```bash
# Create a new times entry
timesman-tools --conn-type grpc create-times --title "My Project"

# Create a simple todo
timesman-tools --conn-type grpc create-todo --tid 1 --content "Complete documentation"

# Create a todo with detailed description
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Implement authentication" \
    --detail "Design and implement secure user authentication with JWT tokens"

# List all todos
timesman-tools --conn-type grpc get-todo-list --tid 1
```

## Command Overview

### Global Options

All commands require the following global options:

- `--conn-type <TYPE>`: Connection type (currently only `grpc` is supported)
- `--server <URL>`: Server URL (optional, defaults to `http://127.0.0.1:8080/`)

### General Commands

| Command | Description |
|---------|-------------|
| `get-times-list` | List all times entries |
| `create-times --title <TITLE>` | Create a new times entry |
| `delete-times --tid <ID>` | Delete a times entry |
| `update-times --tid <ID> --title <TITLE>` | Update times entry title |
| `tui` | Launch interactive TUI mode |

### Basic Todo Commands

| Command | Description |
|---------|-------------|
| `get-todo-list --tid <ID>` | List all todos for a times entry |
| `create-todo --tid <ID> --content <TEXT>` | Create a simple todo |
| `delete-todo --tid <ID> --tdid <TODO_ID>` | Delete a todo |
| `update-todo --tid <ID> --tdid <TODO_ID> --content <TEXT>` | Update todo content |

## Todo Detail Commands

### Create Todo with Detail

Creates a new todo with an optional detailed description.

```bash
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid <TIMES_ID> \
    --content "<BRIEF_DESCRIPTION>" \
    --detail "<DETAILED_DESCRIPTION>"
```

**Parameters:**
- `--tid, -t <TIMES_ID>`: Times entry ID (required)
- `--content, -c <TEXT>`: Brief description of the todo (required)
- `--detail, -d <TEXT>`: Detailed description (required)

**Example:**
```bash
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Implement user registration" \
    --detail "Create user registration system:
- Email validation
- Password strength requirements
- Email confirmation workflow
- Database schema for users table"
```

### Get Todo Detail

Retrieves the complete information for a specific todo, including its detailed description.

```bash
timesman-tools --conn-type grpc get-todo-detail \
    --tid <TIMES_ID> \
    --tdid <TODO_ID>
```

**Parameters:**
- `--tid, -t <TIMES_ID>`: Times entry ID (required)
- `--tdid <TODO_ID>`: Todo ID (required)

**Example:**
```bash
timesman-tools --conn-type grpc get-todo-detail --tid 1 --tdid 3
```

**Sample Output:**
```
Todo ID: 3, Content: Implement user registration
Detail: Create user registration system:
- Email validation
- Password strength requirements
- Email confirmation workflow
- Database schema for users table
Created: 2023-12-16 10:30:00, Done: None
```

### Update Todo Detail

Updates the detailed description of an existing todo.

```bash
timesman-tools --conn-type grpc update-todo-detail \
    --tid <TIMES_ID> \
    --tdid <TODO_ID> \
    --detail "<NEW_DETAIL>"
```

**Parameters:**
- `--tid, -t <TIMES_ID>`: Times entry ID (required)
- `--tdid <TODO_ID>`: Todo ID (required)
- `--detail, -d <TEXT>`: New detailed description (required)

**Example:**
```bash
timesman-tools --conn-type grpc update-todo-detail \
    --tid 1 \
    --tdid 3 \
    --detail "Updated user registration requirements:
- OAuth integration (Google, GitHub)
- Multi-factor authentication
- GDPR compliance
- Rate limiting for security"
```

### Mark Todo Done

Marks a todo as completed while preserving its detailed description.

```bash
timesman-tools --conn-type grpc mark-todo-done \
    --tid <TIMES_ID> \
    --tdid <TODO_ID> \
    --done
```

**Parameters:**
- `--tid, -t <TIMES_ID>`: Times entry ID (required)
- `--tdid <TODO_ID>`: Todo ID (required)
- `--done, -D`: Flag to mark as done (required)

**Example:**
```bash
timesman-tools --conn-type grpc mark-todo-done --tid 1 --tdid 3 --done
```

### Mark Todo Undone

Marks a todo as pending (undoes completion).

```bash
timesman-tools --conn-type grpc mark-todo-undone \
    --tid <TIMES_ID> \
    --tdid <TODO_ID>
```

**Parameters:**
- `--tid, -t <TIMES_ID>`: Times entry ID (required)
- `--tdid <TODO_ID>`: Todo ID (required)

**Example:**
```bash
timesman-tools --conn-type grpc mark-todo-undone --tid 1 --tdid 3
```

## Examples and Use Cases

### Project Management Workflow

This example demonstrates a complete project management workflow using todo details.

#### 1. Set Up Project

```bash
# Create a new project
timesman-tools --conn-type grpc create-times --title "Web Application Project"
# Output: Created times: 1 Web Application Project 2023-12-16 10:00:00
```

#### 2. Plan Project Tasks

```bash
# Create planning tasks with detailed descriptions
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Database Design" \
    --detail "Design database schema:
- User management tables (users, roles, permissions)
- Application data tables (posts, comments, files)
- Audit trail tables (user_actions, system_logs)
- Indexes for performance optimization
- Foreign key constraints and relationships"

timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "API Development" \
    --detail "Develop REST API endpoints:
- Authentication endpoints (login, logout, refresh)
- User management CRUD operations
- Data endpoints with pagination
- File upload/download functionality
- Error handling and validation"

timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Frontend Development" \
    --detail "Build React frontend:
- Component library setup (Material-UI/Chakra)
- Authentication flow and protected routes
- Data display components with sorting/filtering
- Form components with validation
- Responsive design for mobile/desktop"
```

#### 3. Track Progress

```bash
# List all project tasks
timesman-tools --conn-type grpc get-todo-list --tid 1

# Get detailed view of specific task
timesman-tools --conn-type grpc get-todo-detail --tid 1 --tdid 1
```

#### 4. Update Task Details

```bash
# Update database design task with additional requirements
timesman-tools --conn-type grpc update-todo-detail \
    --tid 1 \
    --tdid 1 \
    --detail "Updated database schema design:
- User management tables (users, roles, permissions)
- Application data tables (posts, comments, files)
- Audit trail tables (user_actions, system_logs)
- Indexes for performance optimization
- Foreign key constraints and relationships

ADDITIONAL REQUIREMENTS:
- Data encryption for sensitive fields
- Backup and recovery procedures
- Database migration scripts
- Performance testing with large datasets"
```

#### 5. Complete Tasks

```bash
# Mark database design as complete
timesman-tools --conn-type grpc mark-todo-done --tid 1 --tdid 1 --done

# If needed, reopen task for additional work
timesman-tools --conn-type grpc mark-todo-undone --tid 1 --tdid 1
```

### Bug Tracking Workflow

Use detailed descriptions to track bug investigation and resolution.

```bash
# Create bug report with investigation details
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Fix login timeout issue" \
    --detail "BUG REPORT:
Issue: Users experiencing login timeouts after 5 minutes
Environment: Production server
Browser: Chrome 119, Firefox 120
Steps to reproduce:
1. Login to application
2. Leave browser idle for 5+ minutes
3. Attempt to navigate to protected page
4. Redirected to login with timeout error

INVESTIGATION NOTES:
- JWT token expiry set to 15 minutes (correct)
- Session storage cleared on timeout (correct)
- Server logs show no errors
- Frontend not handling 401 responses properly

TODO:
- Fix frontend token refresh logic
- Add proper error handling for 401 responses
- Test fix across all supported browsers"

# Update with resolution details
timesman-tools --conn-type grpc update-todo-detail \
    --tid 1 \
    --tdid 5 \
    --detail "BUG REPORT: [RESOLVED]
Issue: Users experiencing login timeouts after 5 minutes
Resolution: Fixed automatic token refresh mechanism

CHANGES MADE:
- Updated axios interceptor for 401 handling
- Implemented automatic token refresh before expiry
- Added proper error messaging for expired sessions
- Updated session storage management

TESTING:
- Verified fix works in Chrome, Firefox, Safari
- Tested with various timeout scenarios
- Confirmed proper error messages display
- No regression in normal login flow

DEPLOYED: 2023-12-16 15:30 UTC
MONITORING: Check error rates for next 24 hours"

# Mark as complete
timesman-tools --conn-type grpc mark-todo-done --tid 1 --tdid 5 --done
```

### Meeting Notes and Action Items

Use todo details to capture meeting notes and action items.

```bash
# Create meeting follow-up with detailed notes
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Sprint Planning Meeting Follow-up" \
    --detail "SPRINT PLANNING MEETING - 2023-12-16
Attendees: John, Sarah, Mike, Lisa
Duration: 2 hours

DECISIONS MADE:
- Sprint length: 2 weeks
- Story points capacity: 40 points
- Definition of done updated
- Testing requirements clarified

ACTION ITEMS:
1. John: Set up automated testing pipeline (Due: 2023-12-18)
2. Sarah: Create UI mockups for new features (Due: 2023-12-19)
3. Mike: Database performance analysis (Due: 2023-12-20)
4. Lisa: Update project documentation (Due: 2023-12-21)

RISKS IDENTIFIED:
- Third-party API integration may take longer than estimated
- Holiday schedule may impact sprint velocity
- New team member onboarding in progress

NEXT MEETING: 2023-12-30 10:00 AM - Sprint Review"
```

### Research and Learning Tasks

Use detailed descriptions to track research findings and learning progress.

```bash
# Create research task with findings
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Research GraphQL vs REST API" \
    --detail "RESEARCH TOPIC: GraphQL vs REST for our API redesign

REQUIREMENTS:
- Support for mobile and web clients
- Real-time data updates
- Complex data relationships
- Developer experience

GRAPHQL PROS:
- Single endpoint, flexible queries
- Strongly typed schema
- Excellent tooling (Apollo, GraphiQL)
- Efficient data fetching
- Real-time subscriptions

GRAPHQL CONS:
- Learning curve for team
- Caching complexity
- File upload challenges
- Query complexity analysis needed

REST PROS:
- Team already familiar
- Simple caching strategy
- HTTP status codes
- Mature ecosystem

REST CONS:
- Multiple endpoints to maintain
- Over/under-fetching data
- Versioning challenges

RECOMMENDATION:
Start with REST API improvements, consider GraphQL for v2.0

RESOURCES:
- https://graphql.org/learn/
- Apollo Server documentation
- REST API best practices guide"
```

### Unicode and International Content

The CLI fully supports Unicode characters and international content.

```bash
# Create multilingual task
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Internationalization (i18n) Implementation" \
    --detail "🌍 INTERNATIONALIZATION PROJECT

Support for multiple languages and regions:

📋 PHASE 1: Setup
- Configure i18n framework (react-i18next)
- Set up translation workflow
- Define string extraction process

🌐 SUPPORTED LANGUAGES:
- English (US/UK) 🇺🇸🇬🇧
- Español (ES/MX) 🇪🇸🇲🇽
- Français (FR/CA) 🇫🇷🇨🇦
- Deutsch (DE/AT) 🇩🇪🇦🇹
- 中文简体 (China) 🇨🇳
- 中文繁體 (Taiwan) 🇹🇼
- العربية (Arabic) 🇸🇦
- Русский (Russian) 🇷🇺
- 日本語 (Japanese) 🇯🇵

📱 SPECIAL CONSIDERATIONS:
- RTL languages (Arabic) layout
- Date/time formatting per locale
- Number formatting (1,000 vs 1.000)
- Currency symbols and placement

🧪 TESTING PLAN:
- Verify UI layout with different text lengths
- Test character encoding (UTF-8)
- Validate RTL text alignment
- Check font rendering for all languages"

# Example with Arabic content
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Arabic language support" \
    --detail "دعم اللغة العربية في التطبيق

المتطلبات:
- تخطيط النص من اليمين إلى اليسار (RTL)
- دعم الخطوط العربية
- تنسيق التواريخ والأرقام
- ترجمة جميع النصوص

Arabic language support implementation:
- Right-to-left (RTL) text layout
- Arabic font support
- Date and number formatting
- Complete text translation"
```

## Best Practices

### 1. Organizing Projects

- Use descriptive times entry titles for different projects
- Create todos with clear, actionable content
- Use details for comprehensive task descriptions

### 2. Writing Effective Details

**Good Detail Example:**
```
TASK: Implement user authentication

REQUIREMENTS:
- Email/password login
- JWT token management
- Password reset flow

ACCEPTANCE CRITERIA:
- Users can register with email validation
- Login sessions persist for 24 hours
- Password reset emails sent within 5 minutes
- All forms have proper validation

TECHNICAL NOTES:
- Use bcrypt for password hashing
- Store JWT secrets in environment variables
- Rate limit login attempts (5 per minute)

DEPENDENCIES:
- Email service configuration
- Database user table setup
```

**Avoid:**
- Vague descriptions ("Fix the thing")
- Missing context or requirements
- No acceptance criteria

### 3. Status Management

- Mark todos as done only when completely finished
- Use the undone feature if additional work is needed
- Keep details updated with current status

### 4. Command Line Tips

**Use Shell History:**
```bash
# Use up arrow to repeat commands with modifications
# Use Ctrl+R to search command history
```

**Script Common Operations:**
```bash
#!/bin/bash
# create-project.sh
PROJECT_NAME="$1"
TID=$(timesman-tools --conn-type grpc create-times --title "$PROJECT_NAME" | grep -o '[0-9]\+')

echo "Created project $PROJECT_NAME with ID: $TID"

# Create default tasks
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid $TID \
    --content "Project Setup" \
    --detail "Initial project setup tasks"
```

**Use Environment Variables:**
```bash
# Set default server in your shell profile
export TIMESMAN_SERVER="http://127.0.0.1:8080/"

# Use in commands
timesman-tools --conn-type grpc --server $TIMESMAN_SERVER get-times-list
```

### 5. Backup and Migration

```bash
# Export all data (pseudo-command, actual implementation may vary)
timesman-tools --conn-type grpc export-data > backup-$(date +%Y%m%d).json

# Import data
timesman-tools --conn-type grpc import-data < backup-20231216.json
```

## Troubleshooting

### Common Issues

#### Connection Problems

**Error:** `gRPC error: Connection refused`

**Solutions:**
1. Verify the server is running:
   ```bash
   cargo run -p timesman-server
   ```

2. Check the server URL:
   ```bash
   timesman-tools --conn-type grpc --server http://127.0.0.1:8080/ get-times-list
   ```

3. Verify port availability:
   ```bash
   netstat -an | grep 8080
   ```

#### Invalid Arguments

**Error:** `Missing required argument: content`

**Solution:** Ensure all required parameters are provided:
```bash
# Incorrect
timesman-tools --conn-type grpc create-todo --tid 1

# Correct
timesman-tools --conn-type grpc create-todo --tid 1 --content "Task description"
```

#### ID Not Found Errors

**Error:** `Times with id 999 not found`

**Solutions:**
1. List available times entries:
   ```bash
   timesman-tools --conn-type grpc get-times-list
   ```

2. Use correct ID from the list:
   ```bash
   timesman-tools --conn-type grpc get-todo-list --tid 1
   ```

#### Unicode Display Issues

**Problem:** Unicode characters not displaying correctly

**Solutions:**
1. Ensure terminal supports UTF-8:
   ```bash
   echo $LANG
   # Should show something like: en_US.UTF-8
   ```

2. Set locale if needed:
   ```bash
   export LANG=en_US.UTF-8
   ```

3. Use a terminal that supports Unicode (most modern terminals do)

### Debug Mode

Enable detailed logging for troubleshooting:

```bash
# Set environment variable for debug output
export RUST_LOG=debug
timesman-tools --conn-type grpc get-times-list
```

### Performance Issues

If commands are slow:

1. Check server performance
2. Verify network connectivity
3. Consider using local server for development

### Getting Help

```bash
# General help
timesman-tools --help

# Command-specific help
timesman-tools --conn-type grpc create-todo-with-detail --help

# List all available commands
timesman-tools --conn-type grpc --help
```

## Advanced Usage

### Scripting and Automation

Create shell scripts for common workflows:

```bash
#!/bin/bash
# daily-standup.sh
echo "Creating daily standup tasks..."

TID=1  # Your project ID

timesman-tools --conn-type grpc create-todo-with-detail \
    --tid $TID \
    --content "Daily Standup - $(date +%Y-%m-%d)" \
    --detail "DAILY STANDUP AGENDA:

YESTERDAY'S ACCOMPLISHMENTS:
- [Fill in completed tasks]

TODAY'S GOALS:
- [Fill in planned tasks]

BLOCKERS/IMPEDIMENTS:
- [Fill in any obstacles]

TEAM UPDATES:
- [Any important announcements]

ACTION ITEMS:
- [Follow-up tasks from standup]"
```

### Integration with Other Tools

```bash
# Export todo list to file
timesman-tools --conn-type grpc get-todo-list --tid 1 > todos.txt

# Create todo from file
TODO_CONTENT=$(cat requirements.txt)
timesman-tools --conn-type grpc create-todo-with-detail \
    --tid 1 \
    --content "Implementation Requirements" \
    --detail "$TODO_CONTENT"
```

---

For more information, see:
- [API Documentation](API_DOCUMENTATION.md) - Complete gRPC API reference
- [TUI User Guide](TUI_USER_GUIDE.md) - Interactive text interface guide
- [Developer Guide](DEVELOPER_GUIDE.md) - Development and contribution guide