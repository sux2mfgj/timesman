use crate::app::Event;
use crate::config::{Config, StoreType};
use crate::store::ram::RamStore;
use std::cell::RefCell;
use std::rc::Rc;

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
            ui.text_edit_singleline(&mut self.config.store);
            if ui.button("connect").clicked() {
                let store = match &self.config.store_type {
                    StoreType::Memory => Rc::new(RefCell::new(RamStore::new())),
                    StoreType::Remote(server) => {
                        unimplemented!();
                    }
                };
                event = Some(Event::Connect(store));
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
