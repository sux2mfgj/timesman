# TimesMan TUI Demo

This document demonstrates the Text User Interface (TUI) functionality of TimesMan Tools.

## Starting the TUI

```bash
cargo run -p timesman-tools -- --conn-type grpc tui
```

## Interface Overview

The TUI provides a full-screen terminal interface with the following components:

### Main Views

1. **Times List View** - Default starting view
   - Shows all time tracking entries
   - Displays ID, title, creation date, and last update
   - Supports navigation with arrow keys

2. **Posts List View** - Accessed by pressing Enter on a time entry
   - Shows all posts for the selected time entry
   - Displays post ID, content preview, and timestamps
   - Navigate back with Esc key

3. **Input Dialogs** - For creating and editing
   - Modal dialogs for text input
   - Real-time feedback and validation
   - Cancel with Esc, confirm with Enter

4. **Help View** - Press 'h' from any view
   - Complete keyboard shortcut reference
   - Context-sensitive help
   - Press any key to close

### Status Bar

The bottom status bar shows:
- Current mode/view
- Status messages (success/error)
- Loading indicators
- Real-time feedback

## Workflow Examples

### Creating a New Project

1. Start TUI: `cargo run -p timesman-tools -- --conn-type grpc tui`
2. Press `n` to create new time entry
3. Type project name (e.g., "Website Redesign")
4. Press Enter to confirm
5. Time entry appears in list

### Adding Project Updates

1. Navigate to your project with ↑/↓ keys
2. Press Enter to view posts
3. Press `n` to create new post
4. Type update (e.g., "Completed user authentication")
5. Press Enter to save
6. Post appears in list

### Managing Entries

1. **Edit**: Navigate to entry, press `e`, modify text, press Enter
2. **Delete**: Navigate to entry, press `d` (immediate deletion)
3. **Refresh**: Press `r` to reload data from server
4. **Navigate**: Use ↑/↓ arrows, Enter to drill down, Esc to go back

## Keyboard Reference

### Global Shortcuts
- `h` - Show help screen
- `q` / `Ctrl+Q` - Quit application
- `Esc` - Cancel current action / Go back
- `r` - Refresh current view

### Times List
- `↑/↓` - Navigate entries
- `Enter` - View posts for selected entry
- `n` - Create new time entry
- `e` - Edit selected entry
- `d` - Delete selected entry

### Posts List
- `↑/↓` - Navigate posts
- `n` - Create new post
- `e` - Edit selected post
- `d` - Delete selected post
- `Esc` - Return to times list

### Input Dialogs
- `Enter` - Confirm/Save
- `Esc` - Cancel
- `Backspace` - Delete character
- Any printable character - Type normally

## Error Handling

The TUI provides clear error feedback:

- **Connection Errors**: Red popup with connection failure details
- **Validation Errors**: Status bar messages for invalid input
- **Server Errors**: Detailed error messages with suggested actions
- **Recovery**: All errors can be dismissed with Esc key

## Features

### Real-time Updates
- Automatic refresh after operations
- Status messages for all actions
- Loading indicators for network operations

### Navigation
- Intuitive keyboard navigation
- Breadcrumb-style navigation (Times → Posts)
- Quick access shortcuts

### Data Management
- Full CRUD operations for times and posts
- Immediate visual feedback
- Undo-friendly (no destructive operations without confirmation)

### User Experience
- No mouse required - fully keyboard-driven
- Responsive design adapts to terminal size
- Consistent color scheme and visual hierarchy
- Built-in help system

## Tips for Effective Use

1. **Start with Help**: Press `h` on first use to learn shortcuts
2. **Use Refresh**: Press `r` if data seems stale
3. **Quick Navigation**: Enter/Esc for drilling down/up
4. **Batch Operations**: Create multiple entries quickly with `n` → type → Enter → `n`
5. **Error Recovery**: Use Esc to dismiss errors and try again

## Comparison with CLI Mode

| Feature | TUI Mode | CLI Mode |
|---------|----------|----------|
| **Usability** | Interactive, visual | Command-based |
| **Navigation** | Arrow keys, intuitive | Separate commands |
| **Feedback** | Real-time, visual | Text output |
| **Learning Curve** | Low (guided) | Medium (commands) |
| **Automation** | Manual | Scriptable |
| **Use Case** | Interactive management | Automation, scripts |

## Performance

The TUI is optimized for:
- Fast startup (< 1 second)
- Responsive navigation (immediate feedback)
- Efficient network usage (only loads when needed)
- Low memory footprint
- Works on any terminal size (minimum 80x24)

## Requirements

- Terminal with color support
- Minimum 80x24 terminal size
- Running TimesMan gRPC server
- Network connectivity to server

The TUI provides a modern, user-friendly interface for managing time tracking data while maintaining the flexibility of command-line tools.