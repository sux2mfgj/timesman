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
        AppMode::CreateTimes => render_create_times(f, app, chunks[0]),
        AppMode::EditTimes => render_edit_times(f, app, chunks[0]),
        AppMode::CreatePost => render_create_post(f, app, chunks[0]),
        AppMode::EditPost => render_edit_post(f, app, chunks[0]),
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
        "↑↓: Navigate | Enter: View posts | n: New | e: Edit | d: Delete | r: Refresh | h: Help | q: Quit"
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
        Line::from("Input Dialogs:"),
        Line::from("  Enter     - Confirm action"),
        Line::from("  Esc       - Cancel"),
        Line::from("  Backspace - Delete character"),
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
        AppMode::CreateTimes => "Create Times",
        AppMode::EditTimes => "Edit Times",
        AppMode::CreatePost => "Create Post",
        AppMode::EditPost => "Edit Post",
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