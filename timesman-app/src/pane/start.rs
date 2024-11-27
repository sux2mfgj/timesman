use std::sync::Arc;

use crate::app::Event;
use crate::config::Config;

use super::{pane_menu, Pane};

// use std::sync::Mutex;
use store::ram::RamStore;
use tokio::runtime;
use tokio::sync::Mutex;
// use store::remote::RemoteStore;
// use store::sqlite3::SqliteStore;
use store::Store;

#[derive(PartialEq)]
enum BackingStore {
    Remote,
    Memory,
    Sqlite3,
    Json,
}

pub struct StartPane {
    config: Config,
    errmsg: Option<String>,
    store: BackingStore,
}

impl StartPane {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            errmsg: None,
            store: BackingStore::Remote,
        }
    }

    fn start(&self) -> Result<Event, String> {
        let store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>> =
            match self.store {
                BackingStore::Remote => unimplemented!(),
                //     Rc::new(RefCell::new(RemoteStore::new(
                //     self.config.store.clone(),
                // ))),
                BackingStore::Memory => {
                    Arc::new(Mutex::new(Box::new(RamStore::new())))
                }
                BackingStore::Json => {
                    return Err("Not yet iplemented".to_string());
                }
                BackingStore::Sqlite3 => unimplemented!(),
                // Rc::new(RefCell::new(SqliteStore::new(
                //     &self.config.store.clone(),
                // ))),
            };

        {
            // let store_ref = store.borrow_mut();
            // store_ref.check()?;

            // store.check()?;
        }

        Ok(Event::Connect(store))
    }
}

impl Pane for StartPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        rt: &runtime::Runtime,
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
            ui.radio_value(&mut self.store, BackingStore::Remote, "Server");
            ui.label("Local");
            ui.radio_value(&mut self.store, BackingStore::Memory, "Temporay");
            ui.radio_value(&mut self.store, BackingStore::Json, "Json");
            ui.radio_value(&mut self.store, BackingStore::Sqlite3, "Sqlite");

            ui.separator();
            ui.label("Configurations:");

            match self.store {
                BackingStore::Memory => {}
                BackingStore::Remote => {
                    ui.label("Server");
                    ui.text_edit_singleline(&mut self.config.store);
                }
                BackingStore::Json => {}
                BackingStore::Sqlite3 => {
                    ui.label("database file");
                    ui.text_edit_singleline(&mut self.config.store);
                }
            }

            ui.separator();
            if ui.button("Start").clicked() {
                match self.start() {
                    Ok(e) => {
                        event = Some(e);
                    }
                    Err(e) => {
                        self.errmsg = Some(e);
                    }
                };
            }

            if let Some(e) = &self.errmsg {
                ui.label(format!("error: {e}"));
            }
        });

        event
    }
    fn reload(&mut self, rt: &runtime::Runtime) {}
}
