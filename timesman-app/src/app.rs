use core::fmt;
use std::sync::Arc;
use std::sync::Mutex;

use crate::log::LogRecord;
use crate::pane::config::ConfigPane;
use crate::pane::log::LogPane;
use crate::pane::start::StartPane;
use crate::pane::times::TimesPane;
use crate::req::{Requester, Times};
use eframe;
use egui::{FontData, FontDefinitions, FontFamily};

pub enum Event {
    ToStart,
    ToConfig,
    OpenTimes(Times),
    Logs,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::ToStart => {
                write!(f, "ToStart")
            }
            Event::ToConfig => {
                write!(f, "ToWrite")
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

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Option<Event>;
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
        if let Some(event) = self.pane.update(ctx, _frame, &self.req) {
            debug!("Event: {}", event);

            match event {
                Event::OpenTimes(times) => {
                    self.pane = Box::new(TimesPane::new(times, &self.req));
                }
                Event::ToStart => {
                    self.pane = Box::new(StartPane::new(&self.req));
                }
                Event::ToConfig => {
                    self.pane = Box::new(ConfigPane::new());
                }
                Event::Logs => {
                    self.pane = Box::new(LogPane::new(self.logs.clone()));
                }
            }
        }
    }
}
