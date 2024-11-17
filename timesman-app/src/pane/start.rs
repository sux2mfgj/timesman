use crate::app::Event;
use crate::config::{Config, StoreType};
use crate::store::ram::RamStore;
use crate::store::remote::RemoteStore;
use crate::store::Store;
use std::cell::RefCell;
use std::rc::Rc;

use super::{pane_menu, Pane};

pub struct StartPane {
    config: Config,
    errmsg: Option<String>,
}

impl StartPane {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            errmsg: None,
        }
    }

    fn connect(&mut self, target: String) -> Result<Event, String> {
        let stype = match Config::detect_store_type(target) {
            Ok(t) => t,
            Err(e) => {
                return Err(e);
            }
        };

        let store: Rc<RefCell<dyn Store>> = match stype {
            StoreType::Memory => Rc::new(RefCell::new(RamStore::new())),
            StoreType::Remote(server) => {
                Rc::new(RefCell::new(RemoteStore::new(server)))
            }
            _ => {
                unimplemented!();
            }
        };

        Ok(Event::Connect(store))
    }
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
                match self.connect(self.config.store.clone()) {
                    Ok(e) => {
                        event = Some(e);
                    }
                    Err(e) => {
                        self.errmsg = Some(e);
                    }
                }
            }
            if let Some(e) = &self.errmsg {
                ui.label(format!("error: {e}"));
            }
        });

        event
    }
    fn reload(&mut self) {}
}
