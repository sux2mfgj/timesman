use super::ui;
use egui::{Key, Modifiers};

use timesman_bstore::StoreType;

#[derive(Default, PartialEq)]
enum StoreKind {
    #[default]
    Memory,
    #[cfg(feature = "local")]
    Local,
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

#[derive(Copy, Clone)]
pub enum UIResponse {}

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
        _resps: &Vec<UIResponse>,
    ) -> Result<Vec<UIRequest>, String> {
        // self.handle_ui_response(resps);

        let mut ui_reqs = vec![];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.radio_value(&mut self.store, StoreKind::Memory, "Temporay");
            #[cfg(feature = "local")]
            ui.radio_value(&mut self.store, StoreKind::Local, "Local");

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

    fn start(&mut self, req: &mut Vec<UIRequest>) {
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

        if ui::consume_key(ctx, Key::D) {
            self.use_default();
        }

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::Enter) {
            self.start(reqs);
        }
    }
}
