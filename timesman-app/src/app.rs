use core::fmt;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

use crate::config::Config;
use crate::log::LogRecord;
use crate::pane::config::ConfigPane;
use crate::pane::log::LogPane;
use crate::pane::select_pane::SelectPane;
use crate::pane::start::StartPane;
use crate::pane::times::TimesPane;
use crate::req::{Requester, Times};
use eframe;
use egui::{FontData, FontDefinitions, FontFamily};

pub enum Event {
    Connect(Requester),
    Select(Requester, Times),
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

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Event>;
}

pub struct App {
    pane_stack: VecDeque<Box<dyn Pane>>,
    logs: Arc<Mutex<Vec<LogRecord>>>,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        config: Config,
        logs: Arc<Mutex<Vec<LogRecord>>>,
    ) -> Self {
        Self::config_font(cc, &config);
        let mut stack: VecDeque<Box<dyn Pane>> = VecDeque::new();
        stack.push_front(Box::new(StartPane::new(config)));
        Self {
            pane_stack: stack,
            logs,
        }
    }

    fn config_font(cc: &eframe::CreationContext<'_>, config: &Config) {
        let mut fonts = FontDefinitions::default();

        for font in &config.fonts {
            let name = font.name.clone();
            info!(format!("Loading font ({})", &name));

            fonts.font_data.insert(
                name.clone().to_owned(),
                FontData::from_owned(font.data.clone()),
            );

            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, name.to_owned());
        }

        cc.egui_ctx.set_fonts(fonts);
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

        let event = match pane.update(ctx, _frame) {
            Some(event) => event,
            None => {
                return;
            }
        };

        match event {
            Event::Connect(req) => {
                self.pane_stack.push_front(Box::new(SelectPane::new(req)));
            }
            Event::Select(req, times) => self
                .pane_stack
                .push_front(Box::new(TimesPane::new(req, times))),
            Event::Pop => {
                self.pane_stack.pop_front();
            }
            Event::Logs => {}
            Event::Config => {}
        }

        /*
        if let Some(event) = self.current_pane.update(ctx, _frame) {
            debug!("Event: {}", event);

            match event {
                Event::Connect(req) => {
                    let c = self.current_pane;
                    self.pane_stack.push(c);
                }
                Event::Select(req, tid) => {}
                Event::Pop => {}
                Event::Logs => {}
                Event::Config => {}
            }
        }
        */
    }
}
