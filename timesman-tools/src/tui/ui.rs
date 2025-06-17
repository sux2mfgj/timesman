use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, Paragraph, Wrap,
    },
    Frame,
};

use super::app::{App, AppMode};

pub fn render(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),      // Main content
            Constraint::Length(3),   // Status bar
        ])
        .split(size);

    // Render main content based on mode
    match app.mode {
        AppMode::TimesList => render_times_list(f, app, chunks[0]),
        AppMode::PostsList => render_posts_list(f, app, chunks[0]),
        AppMode::TodosList => render_todos_list(f, app, chunks[0]),
        AppMode::TodoDetail => render_todo_detail(f, app, chunks[0]),
        AppMode::CreateTimes => render_create_times(f, app, chunks[0]),
        AppMode::EditTimes => render_edit_times(f, app, chunks[0]),
        AppMode::CreatePost => render_create_post(f, app, chunks[0]),
        AppMode::EditPost => render_edit_post(f, app, chunks[0]),
        AppMode::CreateTodoDetail => render_create_todo(f, app, chunks[0]),
        AppMode::EditTodoDetail => render_edit_todo(f, app, chunks[0]),
        AppMode::Help => render_help(f, app, chunks[0]),
    }

    // Render status bar
    render_status_bar(f, app, chunks[1]);

    // Render error popup if there's an error
    if app.error_message.is_some() {
        render_error_popup(f, app);
    }
}

fn render_times_list(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Times List")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let items: Vec<ListItem> = app
        .times_list
        .iter()
        .enumerate()
        .map(|(i, times)| {
            let style = if i == app.selected_times_index {
                Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = vec![Line::from(vec![
                Span::styled(format!("[{}] ", times.id), Style::default().fg(Color::Yellow)),
                Span::styled(&times.title, style),
                Span::styled(
                    format!(" ({})", times.created_at.format("%Y-%m-%d %H:%M")),
                    Style::default().fg(Color::Gray),
                ),
            ])];

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(list, area);

    // Render help text at the bottom
    let help_area = Rect {
        x: area.x + 1,
        y: area.y + area.height - 2,
        width: area.width - 2,
        height: 1,
    };

    let help_text = if app.times_list.is_empty() {
        "No times entries. Press 'n' to create new, 'r' to refresh, 'h' for help, 'q' to quit"
    } else {
        "↑↓: Navigate | Enter: View posts | t: View todos | n: New | e: Edit | d: Delete | r: Refresh | h: Help | q: Quit"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(help, help_area);
}

fn render_posts_list(f: &mut Frame, app: &App, area: Rect) {
    let selected_times = app.get_selected_times();
    let title = if let Some(times) = selected_times {
        format!("Posts for: {}", times.title)
    } else {
        "Posts".to_string()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));

    let items: Vec<ListItem> = app
        .posts_list
        .iter()
        .enumerate()
        .map(|(i, post)| {
            let style = if i == app.selected_post_index {
                Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let preview = if post.post.len() > 80 {
                format!("{}...", &post.post[..77])
            } else {
                post.post.clone()
            };

            let content = vec![Line::from(vec![
                Span::styled(format!("[{}] ", post.id), Style::default().fg(Color::Yellow)),
                Span::styled(preview, style),
                Span::styled(
                    format!(" ({})", post.created_at.format("%Y-%m-%d %H:%M")),
                    Style::default().fg(Color::Gray),
                ),
            ])];

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(list, area);

    // Render help text at the bottom
    let help_area = Rect {
        x: area.x + 1,
        y: area.y + area.height - 2,
        width: area.width - 2,
        height: 1,
    };

    let help_text = if app.posts_list.is_empty() {
        "No posts. Press 'n' to create new, Esc to go back, 'h' for help, 'q' to quit"
    } else {
        "↑↓: Navigate | n: New | e: Edit | d: Delete | r: Refresh | Esc: Back | h: Help | q: Quit"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(help, help_area);
}

fn render_todos_list(f: &mut Frame, app: &App, area: Rect) {
    let selected_times = app.get_selected_times();
    let title = if let Some(times) = selected_times {
        format!("Todos for: {}", times.title)
    } else {
        "Todos".to_string()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Magenta));

    let items: Vec<ListItem> = app
        .todos_list
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            let style = if i == app.selected_todo_index {
                Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let status_icon = if todo.done_at.is_some() { "✓" } else { "○" };
            let status_color = if todo.done_at.is_some() { Color::Green } else { Color::Yellow };

            // Add detail indicator
            let detail_indicator = if todo.detail.is_some() && !todo.detail.as_ref().unwrap().trim().is_empty() {
                " 📝"
            } else {
                ""
            };

            let detail_preview = if let Some(detail) = &todo.detail {
                if detail.len() > 35 {
                    format!(" - {}...", &detail[..32])
                } else {
                    format!(" - {}", detail)
                }
            } else {
                String::new()
            };

            let content = vec![Line::from(vec![
                Span::styled(format!("[{}] ", todo.id), Style::default().fg(Color::Yellow)),
                Span::styled(status_icon, Style::default().fg(status_color)),
                Span::styled(format!(" {}", todo.content), style),
                Span::styled(detail_indicator, Style::default().fg(Color::Cyan)),
                Span::styled(detail_preview, Style::default().fg(Color::Gray)),
                Span::styled(
                    format!(" ({})", todo.created_at.format("%Y-%m-%d %H:%M")),
                    Style::default().fg(Color::Gray),
                ),
            ])];

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(list, area);

    // Render help text at the bottom
    let help_area = Rect {
        x: area.x + 1,
        y: area.y + area.height - 2,
        width: area.width - 2,
        height: 1,
    };

    let help_text = if app.todos_list.is_empty() {
        "No todos. Press 'n' to create new, Esc to go back, 'h' for help, 'q' to quit"
    } else {
        "↑↓: Navigate | Enter/d: View detail | n: New | e: Edit | Del: Delete | r: Refresh | Esc: Back | h: Help | q: Quit"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(help, help_area);
}

fn render_todo_detail(f: &mut Frame, app: &App, area: Rect) {
    if let Some(todo) = app.get_selected_todo() {
        let status = if todo.done_at.is_some() { "COMPLETED" } else { "PENDING" };
        let status_color = if todo.done_at.is_some() { Color::Green } else { Color::Yellow };

        let title = format!("Todo Detail - {}", todo.content);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Magenta));

        let inner = block.inner(area);
        f.render_widget(block, area);

        // Layout for content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Status info
                Constraint::Min(1),    // Detail content
                Constraint::Length(1), // Help
            ])
            .split(inner);

        // Render status info
        let status_text = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::White)),
                Span::styled(status, Style::default().fg(status_color)),
            ]),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::White)),
                Span::styled(
                    todo.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ];

        let status_paragraph = Paragraph::new(status_text)
            .style(Style::default().fg(Color::White));
        f.render_widget(status_paragraph, chunks[0]);

        // Render detail content
        let detail_text = if let Some(detail) = &todo.detail {
            detail.clone()
        } else {
            "No detailed description available.".to_string()
        };

        let detail_lines: Vec<Line> = detail_text
            .lines()
            .skip(app.detail_scroll_offset)
            .map(|line| Line::from(line))
            .collect();

        let detail_paragraph = Paragraph::new(detail_lines)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Detail")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Blue)),
            );
        f.render_widget(detail_paragraph, chunks[1]);

        // Render help
        let help_text = "↑↓: Scroll | PgUp/PgDn: Page scroll | e: Edit | Esc: Back | h: Help | q: Quit";
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[2]);
    }
}

fn render_create_todo(f: &mut Frame, app: &App, area: Rect) {
    let title = if let Some(times) = app.get_selected_times() {
        format!("Create Todo for: {}", times.title)
    } else {
        "Create Todo".to_string()
    };

    render_todo_input_dialog(
        f,
        area,
        &title,
        "Enter todo content:",
        &app.input,
        "Enter detail (optional):",
        &app.detail_input,
        "Press Enter to create, Esc to cancel",
    );
}

fn render_edit_todo(f: &mut Frame, app: &App, area: Rect) {
    render_todo_input_dialog(
        f,
        area,
        "Edit Todo",
        "Edit todo content:",
        &app.input,
        "Edit detail:",
        &app.detail_input,
        "Ctrl+S: Save | Ctrl+X: Cancel | Enter: New line | Ctrl+↑/↓: Navigate paragraphs",
    );
}

fn render_todo_input_dialog(
    f: &mut Frame,
    area: Rect,
    title: &str,
    content_label: &str,
    content_input: &str,
    detail_label: &str,
    detail_input: &str,
    help: &str,
) {
    // Create centered dialog
    let popup_area = centered_rect(80, 50, area);

    // Clear the area
    f.render_widget(Clear, popup_area);

    // Create the dialog block
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    // Layout for content input, detail input, and help
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Content label
            Constraint::Length(3), // Content input box
            Constraint::Length(1), // Detail label
            Constraint::Min(5),    // Detail input box
            Constraint::Length(2), // Help text
        ])
        .split(inner);

    // Render content label
    let content_label_paragraph = Paragraph::new(content_label)
        .style(Style::default().fg(Color::White));
    f.render_widget(content_label_paragraph, chunks[0]);

    // Render content input box
    let content_input_paragraph = Paragraph::new(content_input)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        );
    f.render_widget(content_input_paragraph, chunks[1]);

    // Render detail label
    let detail_label_paragraph = Paragraph::new(detail_label)
        .style(Style::default().fg(Color::White));
    f.render_widget(detail_label_paragraph, chunks[2]);

    // Render detail input box
    let detail_input_paragraph = Paragraph::new(detail_input)
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        );
    f.render_widget(detail_input_paragraph, chunks[3]);

    // Render help text
    let help_paragraph = Paragraph::new(help)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(help_paragraph, chunks[4]);
}

fn render_create_times(f: &mut Frame, app: &App, area: Rect) {
    render_input_dialog(
        f,
        area,
        "Create New Times",
        "Enter title:",
        &app.input,
        "Press Enter to create, Esc to cancel",
    );
}

fn render_edit_times(f: &mut Frame, app: &App, area: Rect) {
    render_input_dialog(
        f,
        area,
        "Edit Times",
        "Edit title:",
        &app.input,
        "Press Enter to save, Esc to cancel",
    );
}

fn render_create_post(f: &mut Frame, app: &App, area: Rect) {
    let title = if let Some(times) = app.get_selected_times() {
        format!("Create Post for: {}", times.title)
    } else {
        "Create Post".to_string()
    };

    render_input_dialog(
        f,
        area,
        &title,
        "Enter post content:",
        &app.input,
        "Press Enter to create, Esc to cancel",
    );
}

fn render_edit_post(f: &mut Frame, app: &App, area: Rect) {
    render_input_dialog(
        f,
        area,
        "Edit Post",
        "Edit post content:",
        &app.input,
        "Press Enter to save, Esc to cancel",
    );
}

fn render_input_dialog(
    f: &mut Frame,
    area: Rect,
    title: &str,
    label: &str,
    input: &str,
    help: &str,
) {
    // Create centered dialog
    let popup_area = centered_rect(60, 20, area);

    // Clear the area
    f.render_widget(Clear, popup_area);

    // Create the dialog block
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    // Layout for label, input, and help
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Label
            Constraint::Length(3), // Input box
            Constraint::Min(1),    // Help text
        ])
        .split(inner);

    // Render label
    let label_paragraph = Paragraph::new(label)
        .style(Style::default().fg(Color::White));
    f.render_widget(label_paragraph, chunks[0]);

    // Render input box
    let input_paragraph = Paragraph::new(input)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        );
    f.render_widget(input_paragraph, chunks[1]);

    // Render help text
    let help_paragraph = Paragraph::new(help)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(help_paragraph, chunks[2]);
}

fn render_help(f: &mut Frame, _app: &App, area: Rect) {
    let help_text = vec![
        Line::from("TimesMan TUI Help"),
        Line::from(""),
        Line::from("Global Keys:"),
        Line::from("  h         - Show this help"),
        Line::from("  q / Ctrl+Q - Quit application"),
        Line::from("  Esc       - Go back / Cancel"),
        Line::from(""),
        Line::from("Times List:"),
        Line::from("  ↑/↓       - Navigate list"),
        Line::from("  Enter     - View posts for selected times"),
        Line::from("  t         - View todos for selected times"),
        Line::from("  n         - Create new times entry"),
        Line::from("  e         - Edit selected times"),
        Line::from("  d         - Delete selected times"),
        Line::from("  r         - Refresh list"),
        Line::from(""),
        Line::from("Posts List:"),
        Line::from("  ↑/↓       - Navigate list"),
        Line::from("  n         - Create new post"),
        Line::from("  e         - Edit selected post"),
        Line::from("  d         - Delete selected post"),
        Line::from("  r         - Refresh list"),
        Line::from("  Esc       - Back to times list"),
        Line::from(""),
        Line::from("Todos List:"),
        Line::from("  ↑/↓       - Navigate list"),
        Line::from("  Enter/d   - View todo detail"),
        Line::from("  n         - Create new todo"),
        Line::from("  e         - Edit selected todo"),
        Line::from("  Del       - Delete selected todo"),
        Line::from("  r         - Refresh list"),
        Line::from("  Esc       - Back to times list"),
        Line::from(""),
        Line::from("Todo Detail:"),
        Line::from("  ↑/↓       - Scroll detail content"),
        Line::from("  PgUp/PgDn - Page scroll detail"),
        Line::from("  e         - Edit todo"),
        Line::from("  Esc       - Back to todos list"),
        Line::from(""),
        Line::from("Input Dialogs:"),
        Line::from("  Enter     - Confirm action"),
        Line::from("  Esc       - Cancel"),
        Line::from("  Backspace - Delete character"),
        Line::from(""),
        Line::from("Todo Detail Edit Mode:"),
        Line::from("  Ctrl+S    - Save changes"),
        Line::from("  Ctrl+X    - Cancel changes"),
        Line::from("  Enter     - New line in detail"),
        Line::from("  Ctrl+↑/↓  - Navigate paragraphs"),
        Line::from(""),
        Line::from("Press any key to close help"),
    ];

    let help_paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, area);
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_style = if app.loading {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Green)
    };

    let mode_text = match app.mode {
        AppMode::TimesList => "Times List",
        AppMode::PostsList => "Posts List",
        AppMode::TodosList => "Todos List",
        AppMode::TodoDetail => "Todo Detail",
        AppMode::CreateTimes => "Create Times",
        AppMode::EditTimes => "Edit Times",
        AppMode::CreatePost => "Create Post",
        AppMode::EditPost => "Edit Post",
        AppMode::CreateTodoDetail => "Create Todo",
        AppMode::EditTodoDetail => "Edit Todo",
        AppMode::Help => "Help",
    };

    let status_text = if app.loading {
        "Loading...".to_string()
    } else {
        app.status_message.clone()
    };

    let status_line = Line::from(vec![
        Span::styled(format!("[{}] ", mode_text), Style::default().fg(Color::Cyan)),
        Span::styled(status_text, status_style),
    ]);

    let status_paragraph = Paragraph::new(status_line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Left);

    f.render_widget(status_paragraph, area);
}

fn render_error_popup(f: &mut Frame, app: &App) {
    if let Some(error_msg) = &app.error_message {
        let popup_area = centered_rect(60, 20, f.area());

        // Clear the area
        f.render_widget(Clear, popup_area);

        // Create error dialog
        let error_paragraph = Paragraph::new(error_msg.as_str())
            .block(
                Block::default()
                    .title("Error")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Red)),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(error_paragraph, popup_area);

        // Add help text at bottom
        let help_area = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + popup_area.height - 2,
            width: popup_area.width - 2,
            height: 1,
        };

        let help = Paragraph::new("Press Esc to close")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(help, help_area);
    }
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}