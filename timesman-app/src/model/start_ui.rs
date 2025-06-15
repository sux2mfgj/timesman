use super::ui;
use egui::{Key, Modifiers};

use timesman_bstore::StoreType;

#[derive(Default, PartialEq)]
enum StoreKind {
    #[default]
    Memory,
    #[cfg(feature = "local")]
    Local,
    #[cfg(feature = "grpc")]
    Grpc,
}

pub struct StartUI {
    store: StoreKind,
    param: Option<String>,
    error_text: Option<String>,
}

#[derive(Clone)]
pub enum UIRequest {
    Start(StoreType, Option<String>),
    Close,
}

#[derive(Clone)]
pub enum UIResponse {
    Error(String),
}

impl StartUI {
    pub fn new() -> Self {
        Self {
            store: StoreKind::default(),
            param: None,
            error_text: None,
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        resps: &Vec<UIResponse>,
    ) -> Result<Vec<UIRequest>, String> {
        self.handle_ui_response(resps);

        let mut ui_reqs = vec![];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.radio_value(&mut self.store, StoreKind::Memory, "Temporay");
            #[cfg(feature = "local")]
            ui.radio_value(&mut self.store, StoreKind::Local, "Local");
            #[cfg(feature = "grpc")]
            ui.radio_value(&mut self.store, StoreKind::Grpc, "gRPC Server");

            ui.separator();

            match self.store {
                StoreKind::Memory => {}
                #[cfg(feature = "local")]
                StoreKind::Local => {
                    ui.label("database file");
                    if let Some(db_file) = &self.param {
                        ui.label(format!("{db_file}"));
                    } else {
                        ui.label("Please select a db file");
                    }
                }
                #[cfg(feature = "grpc")]
                StoreKind::Grpc => {
                    ui.label("server URL");
                    let mut server_url = self.param.clone().unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
                    if ui.text_edit_singleline(&mut server_url).changed() {
                        self.param = Some(server_url);
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

    fn handle_ui_response(&mut self, resps: &Vec<UIResponse>) {
        for resp in resps {
            match resp {
                UIResponse::Error(err) => {
                    self.error_text = Some(err.clone());
                }
            }
        }
    }

    fn start(&mut self, req: &mut Vec<UIRequest>) {
        // Clear any previous error messages
        self.error_text = None;
        
        let store = match self.store {
            StoreKind::Memory => Some(StoreType::Memory),
            #[cfg(feature = "local")]
            StoreKind::Local => {
                if self.param.is_none() {
                    self.error_text =
                        Some("db file or path is not specified".to_string());
                    None
                } else {
                    let db_file = self.param.clone().unwrap();
                    Some(StoreType::Local(db_file))
                }
            }
            #[cfg(feature = "grpc")]
            StoreKind::Grpc => {
                let server_url = self.param.clone().unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
                Some(StoreType::Grpc(server_url))
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

    fn use_default(&mut self) {
        match self.store {
            StoreKind::Memory => {}
            #[cfg(feature = "local")]
            StoreKind::Local => {
                let db_path = dirs::data_dir()
                    .unwrap()
                    .join("timesman")
                    .join("unqlite.db");
                self.param = Some(db_path.to_string_lossy().to_string());
            }
            #[cfg(feature = "grpc")]
            StoreKind::Grpc => {
                self.param = Some("http://127.0.0.1:8080".to_string());
            }
        }
    }

    fn consume_keys(&mut self, ctx: &egui::Context, reqs: &mut Vec<UIRequest>) {
        if ui::consume_escape(ctx) {
            reqs.push(UIRequest::Close);
        }

        if ui::consume_key(ctx, Key::T) {
            self.store = StoreKind::Memory;
        }

        #[cfg(feature = "local")]
        if ui::consume_key(ctx, Key::L) {
            self.store = StoreKind::Local;
        }

        #[cfg(feature = "grpc")]
        if ui::consume_key(ctx, Key::G) {
            self.store = StoreKind::Grpc;
        }

        if ui::consume_key(ctx, Key::D) {
            self.use_default();
        }

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::Enter) {
            self.start(reqs);
        }
    }
}
