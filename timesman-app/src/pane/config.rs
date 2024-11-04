use crate::app::{Event, Pane};

use super::pane_menu;

pub struct ConfigPane {}

impl Pane for ConfigPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        _req: &crate::req::Requester,
    ) -> Option<Event> {
        let mut event = None;
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui) {
                        event = Some(e);
                    }
                });
            });
        });

        event
    }
}

impl ConfigPane {
    pub fn new() -> Self {
        Self {}
    }
}
