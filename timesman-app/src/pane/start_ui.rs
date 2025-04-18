use super::{ui, PaneModel, PaneRequest, PaneResponse};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;
use timesman_bstore::StoreType;

#[derive(Clone)]
pub enum UIRequest {
    Start(StoreType, Option<String>),
    Close,
}

#[derive(Copy, Clone)]
pub enum UIResponse {}

pub trait StartPaneTrait {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg: &Vec<UIResponse>,
    ) -> Result<Vec<UIRequest>, String>;
}

pub struct StartPane {
    store: StoreKind,
    param: Option<String>,
    path: Option<PathBuf>,
    server_enable: bool,
    server: String,
    file_dialog: FileDialog,
    error_text: Option<String>,
}

#[derive(Default, PartialEq)]
enum StoreKind {
    #[default]
    Memory,
    #[cfg(feature = "sqlite")]
    Sqlite,
}

impl StartPaneTrait for StartPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        resps: &Vec<UIResponse>,
    ) -> Result<Vec<UIRequest>, String> {
        self.handle_ui_response(resps);

        let mut ui_reqs = vec![];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.radio_value(&mut self.store, StoreKind::Memory, "Temporay");
            #[cfg(feature = "sqlite")]
            ui.radio_value(&mut self.store, StoreKind::Sqlite, "Sqlite");

            ui.separator();

            match self.store {
                StoreKind::Memory => {}
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

            ui.separator();
            ui.checkbox(&mut self.server_enable, "Enable server");
            ui.text_edit_singleline(&mut self.server);

            ui.separator();

            if ui.button("Start").clicked() {
                let store = match self.store {
                    StoreKind::Memory => Some(StoreType::Memory),
                    #[cfg(feature = "sqlite")]
                    StoreKind::Sqlite => {
                        if self.param.is_none() || self.path.is_none() {
                            self.error_text = Some(
                                "db file or path is not specified".to_string(),
                            );
                            None
                        } else {
                            let db_file = self.param.clone().unwrap();
                            let file_path = self.path.clone().unwrap();
                            Some(StoreType::Sqlite(db_file, file_path))
                        }
                    }
                };
                if let Some(s) = store {
                    let server = if self.server_enable {
                        Some(self.server.clone())
                    } else {
                        None
                    };
                    ui_reqs.push(UIRequest::Start(s, server));
                }
            }
            ui.separator();
            if let Some(e) = &self.error_text {
                ui.label(format!("{e}"));
            }
        });

        let r = self.consume_keys(ctx);
        ui_reqs = vec![ui_reqs, r].concat();

        Ok(ui_reqs)
    }
}

impl StartPane {
    pub fn new() -> Self {
        let store = StoreKind::default();
        Self {
            store,
            param: None,
            path: None,
            server_enable: false,
            server: "127.0.0.1:8080".to_string(),
            file_dialog: FileDialog::new(),
            error_text: None,
        }
    }

    fn handle_ui_response(&self, resps: &Vec<UIResponse>) {
        for _r in resps {
            todo!();
        }
    }

    fn consume_keys(&self, ctx: &egui::Context) -> Vec<UIRequest> {
        let mut reqs = vec![];

        if ui::consume_escape(ctx) {
            reqs.push(UIRequest::Close);
        }

        reqs
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {}
}
