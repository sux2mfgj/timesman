use std::f32::consts::E;
use std::path::PathBuf;
use std::sync::Arc;

use crate::app::Event;
use crate::config::Config;

use super::Pane;

use egui_file_dialog::FileDialog;
#[cfg(feature = "grpc")]
use timesman_bstore::grpc::GrpcStore;
#[cfg(feature = "json")]
use timesman_bstore::json::JsonStore;
use timesman_bstore::ram::RamStore;
#[cfg(feature = "http")]
use timesman_bstore::remote::RemoteStore;
#[cfg(feature = "sqlite")]
use timesman_bstore::sqlite::SqliteStoreBuilder;
use timesman_bstore::{Store, StoreParam, StoreType};
use tokio::runtime;
use tokio::sync::mpsc::{self};
use tokio::sync::Mutex;

pub struct StartPane {
    config: Config,
    errmsg: Option<String>,
    store: StoreType,
    file_dialog: FileDialog,
    json_file: Option<PathBuf>,
}

impl StartPane {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            errmsg: None,
            store: StoreType::default(),
            file_dialog: FileDialog::new(),
            json_file: None,
        }
    }

    fn start(&self) -> Result<Event, String> {
        match self.store {
            #[cfg(feature = "http")]
            StoreType::Remote => {
                Ok(Event::Connect(
                    StoreType::Remote,
                    StoreParam::Remote(self.config.params.remote.server.clone()),
                ))
            }
            #[cfg(feature = "grpc")]
            StoreType::Grpc => {
                Ok(Event::Connect(
                    StoreType::Grpc,
                    StoreParam::Grpc(self.config.params.grpc.server.clone()),
                ))
            }
            StoreType::Memory => {
                Ok(Event::Connect(StoreType::Memory, StoreParam::None))
            }
            #[cfg(feature = "json")]
            StoreType::Json => {
                if let Some(path) = &self.json_file {
                    Ok(Event::Connect(
                        StoreType::Json,
                        StoreParam::Json(path.clone()),
                    ))
                } else {
                    Err(format!("You should select the json file"))
                }
            }
            #[cfg(feature = "sqlite")]
            StoreType::Sqlite => {
                Ok(Event::Connect(
                    StoreType::Sqlite,
                    StoreParam::Sqlite(self.config.params.sqlite.db.clone()),
                ))
            }
            _ => Err("Invalid store type".to_string()),
        }
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
            #[cfg(feature = "http")]
            ui.radio_value(&mut self.store, StoreType::Remote, "Server");
            #[cfg(feature = "grpc")]
            ui.radio_value(&mut self.store, StoreType::Grpc, "Grpc");
            ui.label("Local");
            ui.radio_value(&mut self.store, StoreType::Memory, "Temporay");
            #[cfg(feature = "json")]
            ui.radio_value(&mut self.store, StoreType::Json, "Json");
            #[cfg(feature = "sqlite")]
            ui.radio_value(&mut self.store, StoreType::Sqlite, "Sqlite");

            ui.separator();
            ui.label("Configurations:");

            match self.store {
                StoreType::Memory => {}
                #[cfg(feature = "http")]
                StoreType::Remote => {
                    ui.label("Server");
                    let server = &mut self.config.params.remote.server;
                    ui.text_edit_singleline(server);
                }
                #[cfg(feature = "grpc")]
                StoreType::Grpc => {
                    ui.label("Server");
                    let server = &mut self.config.params.grpc.server;
                    ui.text_edit_singleline(server);
                }
                #[cfg(feature = "json")]
                StoreType::Json => {
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
                #[cfg(feature = "sqlite")]
                StoreType::Sqlite => {
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

    fn reload(&mut self, _rt: &runtime::Runtime) {}
}
