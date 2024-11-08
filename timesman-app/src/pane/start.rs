use crate::app::Event;
use crate::config::Config;
use crate::req::Requester;

use super::{pane_menu, Pane};

pub struct StartPane {
    config: Config,
}

impl Pane for StartPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("server");
            ui.text_edit_singleline(&mut self.config.server);
            if ui.button("connect").clicked() {
                event =
                    Some(Event::Connect(Requester::new(&self.config.server)));
            }
        });

        event
    }
    fn reload(&mut self) {}
}

impl StartPane {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
