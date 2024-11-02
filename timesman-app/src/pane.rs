pub mod config;
pub mod log;
pub mod start;
pub mod times;

use crate::app::Event;
use crate::req::{Requester, Times};
use egui;

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Event;
}

pub fn pane_menu(ui: &mut egui::Ui) -> Option<Event> {
    let mut e = None;
    if ui.button("Show logs").clicked() {
        e = Some(Event::Logs);
    }

    if ui.button("Config").clicked() {
        e = Some(Event::ToConfig);
    }

    if ui.button("To start pane").clicked() {
        e = Some(Event::ToStart);
    }

    if let Some(_) = &e {
        ui.close_menu();
    }

    e
}
