use crate::{app::Event, plugin::Plugin};

use super::{pane_menu, Pane};

pub struct ConfigPane {}

impl Pane for ConfigPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        plugin: &mut Plugin,
    ) -> Option<Event> {
        let mut event = None;
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui, plugin) {
                        event = Some(e);
                    }
                });
            });
        });

        event
    }

    fn reload(&mut self) {}
}

impl ConfigPane {
    pub fn new() -> Self {
        Self {}
    }
}
