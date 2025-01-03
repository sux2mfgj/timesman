pub mod config;
pub mod log;
pub mod select;
pub mod start;
pub mod test;
pub mod times;

use crate::app::Event;
use egui;
use tokio::runtime;

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        rt: &runtime::Runtime,
    ) -> Option<Event>;

    fn reload(&mut self, rt: &runtime::Runtime);

    fn times_menu(&self, ui: &mut egui::Ui) -> Option<Event> {
        let mut e = None;
        ui.menu_button("Times", |ui| {
            e = self.times_menu_content(ui);
        });

        e
    }

    fn times_menu_content(&self, ui: &mut egui::Ui) -> Option<Event> {
        let mut e = None;
        if ui.button("Show logs").clicked() {
            e = Some(Event::Logs);
        }

        if ui.button("Config").clicked() {
            e = Some(Event::Config);
        }

        if ui.button("Back").clicked() {
            e = Some(Event::Pop);
        }

        if let Some(_) = &e {
            ui.close_menu();
        }

        e
    }

    fn show_latest_log(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Log");
            ui.separator();
            if let Some(l) = crate::log::latest() {
                l.show(ui);
            }
        });
    }
}
