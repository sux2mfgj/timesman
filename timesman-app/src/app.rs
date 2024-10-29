use crate::req::{Requester, Times};
use crate::start::StartPane;
use crate::times::TimesPane;
use eframe;

pub enum Event {
    Nothing,
    OpenTimes(Times),
}

pub trait Pane {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, req: &Requester)
        -> Event;
}

pub struct App {
    req: Requester,
    pane: Box<dyn Pane>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let req = Requester::new(&"http://localhost:8080".to_string());
        Self {
            pane: Box::new(StartPane::new(&req)),
            req,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.pane.update(ctx, _frame, &self.req) {
            Event::Nothing => {}
            Event::OpenTimes(times) => {
                self.pane = Box::new(TimesPane::new(times, &self.req));
            }
        }
    }
}
