pub mod config;
pub mod log;
pub mod select_pane;
pub mod start;
pub mod times;

use crate::app::Event;
use crate::plugin::Plugin;
use egui;

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        plugin: &mut Plugin,
    ) -> Option<Event>;

    fn reload(&mut self);
}

pub fn pane_menu(ui: &mut egui::Ui, plugin: &mut Plugin) -> Option<Event> {
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

    plugin.update_bar(ui);
    // ui.separator();
    // ui.label(format!("Pomodolo: 25:{:02}", 0));
    // if ui.button("start").clicked() {}

    if let Some(_) = &e {
        ui.close_menu();
    }

    e
}
