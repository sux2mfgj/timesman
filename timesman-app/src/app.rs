use core::fmt;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::runtime;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::log::LogRecord;
use crate::pane::config::ConfigPane;
use crate::pane::log::LogPane;
use crate::pane::select_pane::SelectPane;
use crate::pane::start::StartPane;
use crate::pane::times::TimesPane;
use crate::pane::Pane;

use eframe;
use store::{Store, Times};

pub enum Event {
    Connect(Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>),
    Select(Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>, Times),
    Pop,
    Logs,
    Config,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Connect(_) => {
                write!(f, "Connect")
            }
            Event::Select(_, _) => {
                write!(f, "Disconnect")
            }
            Event::Pop => {
                write!(f, "Pop")
            }
            Event::Logs => {
                write!(f, "Logs")
            }
            Event::Config => {
                write!(f, "Config")
            }
        }
    }
}

pub struct App {
    pane_stack: VecDeque<Box<dyn Pane>>,
    logs: Arc<std::sync::Mutex<Vec<LogRecord>>>,
    config: Config,
    rt: runtime::Runtime,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        config: Config,
        logs: Arc<std::sync::Mutex<Vec<LogRecord>>>,
    ) -> Result<Self, String> {
        let mut stack: VecDeque<Box<dyn Pane>> = VecDeque::new();
        stack.push_front(Box::new(StartPane::new(config.clone())));

        config.fonts.load_fonts(cc);

        Ok(Self {
            pane_stack: stack,
            logs,
            config,
            rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let pane: &mut Box<dyn Pane> = match self.pane_stack.front_mut() {
            Some(pane) => pane,
            None => {
                unimplemented!("shoud close app");
            }
        };

        let event = match pane.update(ctx, _frame, &self.rt) {
            Some(event) => event,
            None => {
                return;
            }
        };

        match event {
            Event::Connect(store) => {
                self.pane_stack
                    .push_front(Box::new(SelectPane::new(store, &self.rt)));
            }
            Event::Select(store, times) => self
                .pane_stack
                .push_front(Box::new(TimesPane::new(store, times, &self.rt))),
            Event::Pop => {
                self.pane_stack.pop_front();
                let p: &mut Box<dyn Pane> = match self.pane_stack.front_mut() {
                    Some(p) => p,
                    None => {
                        return;
                    }
                };

                p.reload(&self.rt);
            }
            Event::Logs => {
                self.pane_stack
                    .push_front(Box::new(LogPane::new(self.logs.clone())));
            }
            Event::Config => {
                self.pane_stack.push_front(Box::new(ConfigPane::new()));
            }
        }
    }
}
