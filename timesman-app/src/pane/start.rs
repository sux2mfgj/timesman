use std::path::PathBuf;
use std::sync::Arc;

use crate::app::Event;
use crate::config::Config;

use super::Pane;

use egui_file_dialog::FileDialog;
use store::json::JsonStore;
use store::ram::RamStore;
use store::remote::RemoteStore;
use store::sqlite3::SqliteStoreBuilder;
use store::Store;
use tokio::runtime;
use tokio::sync::mpsc::{self};
use tokio::sync::Mutex;

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
    file_dialog: FileDialog,
    json_file: Option<PathBuf>,
}

impl StartPane {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            errmsg: None,
            store: BackingStore::Remote,
            file_dialog: FileDialog::new(),
            json_file: None,
        }
    }

    fn start(&self, rt: &runtime::Runtime) -> Result<Event, String> {
        let store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>> =
            match self.store {
                BackingStore::Remote => {
                    let server = self.config.params.remote.server.clone();
                    Arc::new(Mutex::new(Box::new(RemoteStore::new(server))))
                }
                BackingStore::Memory => {
                    Arc::new(Mutex::new(Box::new(RamStore::new())))
                }
                BackingStore::Json => {
                    if let Some(path) = &self.json_file {
                        let store = JsonStore::build(path.clone())?;
                        Arc::new(Mutex::new(Box::new(store)))
                    } else {
                        return Err(format!("You should select the json file"));
                    }
                }
                BackingStore::Sqlite3 => {
                    let path = self.config.params.sqlite.db.clone();
                    let store = SqliteStoreBuilder::new(&path);
                    let store =
                        rt.block_on(async move { store.build().await })?;
                    Arc::new(Mutex::new(Box::new(store)))
                }
            };

        {
            let store = store.clone();
            let (tx, mut rx) = mpsc::channel::<Result<(), String>>(8);

            rt.block_on(async move {
                let store = store.lock().await;
                tx.send(store.check().await).await.unwrap();
            });

            rx.blocking_recv().ok_or("failed to setup backing store")?
        }?;

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
                event = self.times_menu(ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            self.show_latest_log(ui);
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
                    let server = &mut self.config.params.remote.server;
                    ui.text_edit_singleline(server);
                }
                BackingStore::Json => {
                    ui.label("File");
                    if ui.button("Select").clicked() {
                        self.file_dialog.select_file();
                    }

                    if let Some(path) = self.file_dialog.update(ctx).selected()
                    {
                        self.json_file = Some(path.to_path_buf());
                    }

                    if let Some(path) = &self.json_file {
                        ui.label(format!("{:?}", path));
                    }
                }
                BackingStore::Sqlite3 => {
                    ui.label("database file");

                    if ui.button("Select").clicked() {
                        self.file_dialog.select_file();
                    }

                    if let Some(path) = self.file_dialog.update(ctx).selected()
                    {
                        self.config.params.sqlite.db =
                            path.to_string_lossy().to_string();
                    }
                    ui.label(&self.config.params.sqlite.db);
                }
            }

            ui.separator();
            if ui.button("Start").clicked() {
                match self.start(rt) {
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

    fn reload(&mut self, _rt: &runtime::Runtime) {}
}
