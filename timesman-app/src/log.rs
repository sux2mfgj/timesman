use std::fmt;

use crate::app::{Event, Pane};
use crate::req::Requester;

//TODO: use Log trait in the log crate.

enum LogLevel {
    Error,
    Info,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = match self {
            LogLevel::Error => "ERROR",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        };

        write!(f, "{}", t)
    }
}

struct LogRecord {
    level: LogLevel,
    text: String,
}

static mut LOGS: Vec<LogRecord> = vec![];

pub struct Logger {}

impl Logger {
    //pub fn set_log_pane(logp: &LogPane) { log_pane = logp; }

    fn log(level: LogLevel, text: String) {
        eprintln!(
            "{}: {}: {}",
            level,
            chrono::Utc::now().format("%Y-%m-%d %h%M").to_string(),
            text
        );

        unsafe {
            LOGS.push(LogRecord { level, text });
        }
    }
    pub fn info(text: String) {
        Self::log(LogLevel::Info, text);
    }
    pub fn error(text: String) {
        Self::log(LogLevel::Error, text);
    }
    pub fn debug(text: String) {
        Self::log(LogLevel::Debug, text);
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
