use chrono::{Local, NaiveDateTime};
use core::fmt;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Info,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => {
                write!(f, "Error")
            }
            LogLevel::Info => {
                write!(f, "Info")
            }
            LogLevel::Debug => {
                write!(f, "Debug")
            }
        }
    }
}

#[derive(Debug)]
pub struct LogRecord {
    pub level: LogLevel,
    pub text: String,
    pub time: NaiveDateTime,
}

impl LogRecord {
    pub fn show(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(self.time.format("%Y-%m-%d %H:%M").to_string());
            ui.separator();
            ui.label(format!("{}", self.level));
            ui.separator();
            ui.label(&self.text);
        });
    }
}

static LOGS: OnceCell<Arc<Mutex<Vec<LogRecord>>>> = OnceCell::new();

pub fn register(logs: Arc<Mutex<Vec<LogRecord>>>) {
    LOGS.set(logs).unwrap();
}

pub fn log(level: LogLevel, text: String) {
    println!("{} {}", level, text);
    LOGS.get_or_init(|| Arc::new(Mutex::new(vec![])))
        .lock()
        .unwrap()
        .push(LogRecord {
            level,
            text,
            time: Local::now().naive_local(),
        });
}

#[macro_export]
macro_rules! info {
    ($text:expr) => {
        $crate::log::log($crate::log::LogLevel::Info, $text.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Info, format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($text:expr) => {
        $crate::log::log($crate::log::LogLevel::Error, $text.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Error, format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($text:expr) => {
        $crate::log::log($crate::log::LogLevel::Debug, $text.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Debug, format!($fmt, $($arg)*))
    };
}

// TODO: define macro to define macro
/*
macro_rules! log_define {
    ($i:ident, $type:expr) => {
        #[macro_rules]
        module_rule! $i {
            ($text:expr) => {
                $crate::log::log($crate::log::$type, $text.to_string())
            };
            ($fmt:expr, $($arg:tt)*) => {
                $crate::log::log($crate::log::$type, format!($fmt, $($arg)*))
            };
        }
    }
}
*/
