use super::ui;
use egui::{Key, Modifiers};

use timesman_bstore::StoreType;

#[derive(Default, PartialEq)]
enum StoreKind {
    #[default]
    Memory,
    #[cfg(feature = "sqlite")]
    Sqlite,
}

pub struct StartUI {
    store: StoreKind,
    error_text: Option<String>,
}

#[derive(Clone)]
pub enum UIRequest {
    Start(StoreType, Option<String>),
    Close,
}

#[derive(Copy, Clone)]
pub enum UIResponse {}

impl StartUI {
    pub fn new() -> Self {
        Self {
            store: StoreKind::default(),
            error_text: None,
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        resps: &Vec<UIResponse>,
    ) -> Result<Vec<UIRequest>, String> {
        // self.handle_ui_response(resps);

        let mut ui_reqs = vec![];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.radio_value(&mut self.store, StoreKind::Memory, "Temporay");
            #[cfg(feature = "sqlite")]
            ui.radio_value(&mut self.store, StoreKind::Sqlite, "Sqlite");

            ui.separator();

            match self.store {
                StoreKind::Memory => {}
                #[cfg(feature = "sqlite")]
                StoreKind::Sqlite => {
                    ui.label("database file");
                    if let Some(db_file) = &self.param {
                        ui.label(format!("File: {db_file}"));
                    } else {
                        ui.label("Please select a db file");
                    }
                    if ui.button("Select").clicked() {
                        self.file_dialog.select_file();
                    }
                    if let Some(db_file_path) =
                        self.file_dialog.update(ctx).selected()
                    {
                        self.param =
                            Some(db_file_path.to_string_lossy().to_string());
                    }

                    ui.label("file path");
                    if let Some(file_path) = &self.path {
                        ui.label(format!("File: {:?}", file_path));
                    } else {
                        ui.label("Please select a file path");
                    }

                    if ui.button("Default").clicked() {
                        let db_path = dirs::data_dir()
                            .unwrap()
                            .join("timesman")
                            .join("sqlite.db");
                        self.param =
                            Some(db_path.to_string_lossy().to_string());

                        let file_path = dirs::data_dir()
                            .unwrap()
                            .join("timesman")
                            .join("files");
                        self.path = Some(file_path);
                    }
                }
            }

            //ui.separator();
            //ui.checkbox(&mut self.server_enable, "Enable server");
            //ui.text_edit_singleline(&mut self.server);

            ui.separator();

            if ui.button("Start").clicked() {
                self.start(&mut ui_reqs);
            }
            ui.separator();
            if let Some(e) = &self.error_text {
                ui.label(format!("{e}"));
            }
        });

        self.consume_keys(ctx, &mut ui_reqs);

        Ok(ui_reqs)
    }

    fn start(&self, req: &mut Vec<UIRequest>) {
        let store = match self.store {
            StoreKind::Memory => Some(StoreType::Memory),
            #[cfg(feature = "sqlite")]
            StoreKind::Sqlite => {
                if self.param.is_none() || self.path.is_none() {
                    self.error_text =
                        Some("db file or path is not specified".to_string());
                    None
                } else {
                    let db_file = self.param.clone().unwrap();
                    let file_path = self.path.clone().unwrap();
                    todo!();
                    //Some(StoreType::Sqlite(db_file, file_path))
                }
            }
        };

        if let Some(s) = store {
            // let server = if self.server_enable {
            //     Some(self.server.clone())
            // } else {
            //     None
            // };
            req.push(UIRequest::Start(s, None));
        }
    }

    fn consume_keys(&mut self, ctx: &egui::Context, reqs: &mut Vec<UIRequest>) {
        if ui::consume_escape(ctx) {
            reqs.push(UIRequest::Close);
        }

        if ui::consume_key(ctx, Key::T) {
            self.store = StoreKind::Memory;
        }
        #[cfg(feature = "sqlite")]
        if ui::consume_key(ctx, Key::S) {
            self.store = StoreKind::Sqlite;
        }

        // if ui::consume_key(ctx, Key::D) {
        //     self.use_default();
        // }

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::Enter) {
            self.start(reqs);
        }
    }
}
