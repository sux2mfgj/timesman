use super::{pane_menu, Pane};
use crate::app::Event;
use crate::config::Config;

use tokio::runtime;

pub struct ConfigPane {
    config: Config,
    edit_mode: bool,
}

impl Pane for ConfigPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        _rt: &runtime::Runtime,
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
            ui.horizontal(|ui| {
                ui.label("Default Store Type");
                ui.separator();
                if self.edit_mode {
                    ui.text_edit_singleline(&mut self.config.params.store);
                } else {
                    ui.label(format!("{}", self.config.params.store));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Sqlite DB path");
                ui.separator();
                if self.edit_mode {
                    ui.text_edit_singleline(&mut self.config.params.sqlite.db);
                } else {
                    ui.label(format!("{}", self.config.params.sqlite.db));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Remote server URL");
                ui.separator();
                if self.edit_mode {
                    ui.text_edit_singleline(
                        &mut self.config.params.remote.server,
                    );
                } else {
                    ui.label(format!("{}", self.config.params.remote.server));
                }
            });

            ui.separator();

            ui.label("Fonts");
            for (i, font) in self.config.fonts.fonts.iter().enumerate() {
                ui.label(format!("{}: {}", i, font.name));
            }
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            if self.edit_mode {
                if ui.button("Done").clicked() {
                    self.edit_mode = false;
                    event = Some(Event::UpdateConfig(self.config.clone()));
                }
            } else {
                if ui.button("Edit").clicked() {
                    self.edit_mode = true;
                }
            }
        });

        event
    }

    fn reload(&mut self, _rt: &runtime::Runtime) {}
}

impl ConfigPane {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            edit_mode: false,
        }
    }
}
