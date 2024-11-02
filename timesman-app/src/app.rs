use core::fmt;
use std::sync::Arc;
use std::sync::Mutex;

use crate::log::LogRecord;
use crate::pane::log::LogPane;
use crate::pane::start::StartPane;
use crate::pane::times::TimesPane;
use crate::req::{Requester, Times};
use eframe;
use egui::{FontData, FontDefinitions, FontFamily};
use log::info;

pub enum Event {
    Nothing,
    ToStart,
    OpenTimes(Times),
    Logs,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Nothing => {
                write!(f, "Nothing")
            }
            Event::ToStart => {
                write!(f, "ToStart")
            }
            Event::OpenTimes(_t) => {
                //TODO: show the times info
                write!(f, "OpenTimes")
            }
            Event::Logs => {
                write!(f, "Logs")
            }
        }
    }
}

impl Event {
    fn is_nothing(&self) -> bool {
        match self {
            Event::Nothing => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Event;
}

pub struct App {
    req: Requester,
    pane: Box<dyn Pane>,
    logs: Arc<Mutex<Vec<LogRecord>>>,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        logs: Arc<Mutex<Vec<LogRecord>>>,
    ) -> Self {
        Self::config_font(cc);
        let req = Requester::new(&"http://localhost:8080".to_string());
        Self {
            pane: Box::new(StartPane::new(&req)),
            req,
            logs,
        }
    }

    fn config_font(cc: &eframe::CreationContext<'_>) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "ja".to_owned(),
            FontData::from_static(include_bytes!(
                "../fonts/ja/NotoSansJP-VariableFont_wght.ttf"
            )),
        );

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "ja".to_owned());

        cc.egui_ctx.set_fonts(fonts);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let event = self.pane.update(ctx, _frame, &self.req);

        if !event.is_nothing() {
            info!("Event: {}", event);
        }

        match event {
            Event::Nothing => {}
            Event::OpenTimes(times) => {
                self.pane = Box::new(TimesPane::new(times, &self.req));
            }
            Event::ToStart => {
                self.pane = Box::new(StartPane::new(&self.req));
            }
            Event::Logs => {
                self.pane = Box::new(LogPane::new(self.logs.clone()));
            }
        }
    }
}
