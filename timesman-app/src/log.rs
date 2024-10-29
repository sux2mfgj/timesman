use log;

use crate::app::{Event, Pane};
use crate::req::Requester;

//TODO: use Log trait in the log crate.

//static mut log_pane: Option<&LogPane> = None;

pub struct Logger {}

impl Logger {
    //pub fn set_log_pane(logp: &LogPane) { log_pane = logp; }

    fn log(level: &str, text: String) {
        eprintln!(
            "{}: {}: {}",
            level,
            chrono::Utc::now().format("%Y-%m-%d %h%M").to_string(),
            text
        );
    }
    pub fn info(text: String) {
        Self::log("INFO", text);
    }
    pub fn error(text: String) {
        Self::log("ERROR", text);
    }
    pub fn debug(text: String) {
        Self::log("DEBUG", text);
    }
}

pub struct LogPane {}

impl Pane for LogPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        _req: &Requester,
    ) -> Event {
        Event::Nothing
    }
}
