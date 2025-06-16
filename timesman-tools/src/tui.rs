pub mod app;
mod ui;

use app::{App, AppResult};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

use crate::Client;

pub fn run_tui(client: Box<dyn Client>) -> Result<(), String> {
    // Setup terminal
    enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| format!("Failed to setup terminal: {}", e))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)
        .map_err(|e| format!("Failed to create terminal: {}", e))?;

    // Create app and run it
    let app = App::new(client);
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode().map_err(|e| format!("Failed to disable raw mode: {}", e))?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(|e| format!("Failed to restore terminal: {}", e))?;
    terminal.show_cursor()
        .map_err(|e| format!("Failed to show cursor: {}", e))?;

    if let Err(err) = res {
        return Err(format!("TUI error: {}", err));
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, mut app: App) -> AppResult<()> {
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.handle_key_event(key) {
                Ok(should_quit) => {
                    if should_quit {
                        return Ok(());
                    }
                }
                Err(err) => return Err(err.into()),
            }
        }
    }
}