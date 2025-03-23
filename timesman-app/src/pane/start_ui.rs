use super::{ui, PaneModel, PaneRequest, PaneResponse};
use egui_file_dialog::FileDialog;
use timesman_bstore::StoreType;

#[derive(Clone)]
pub enum UIRequest {
    Start(StoreType),
    Close,
}

#[derive(Copy, Clone)]
pub enum UIResponse {
    Ok,
    Err,
}

pub trait StartPaneTrait {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg: &Vec<UIResponse>,
    ) -> Result<Vec<UIRequest>, String>;
}

pub struct StartPane {
    store: StoreKind,
    param: String,
    file_dialog: FileDialog,
    error_text: Option<String>,
}

#[derive(Default, PartialEq)]
enum StoreKind {
    #[default]
    Memory,
    #[cfg(feature = "sqlite")]
    Sqlite,
    #[cfg(feature = "grpc")]
    Grpc,
}

impl StartPaneTrait for StartPane {
    fn update(
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
            #[cfg(feature = "grpc")]
            ui.radio_value(&mut self.store, StoreKind::Grpc, "GRPC Server");

            ui.separator();

            match self.store {
                StoreKind::Memory => {}
                StoreKind::Sqlite => {
                    ui.horizontal(|ui| {
                        ui.label("database file:");
                        ui.label(&self.param);
                    });
                    if ui.button("Select").clicked() {
                        self.file_dialog.select_file();
                    }
                    if let Some(db_file_path) =
                        self.file_dialog.update(ctx).selected()
                    {
                        self.param = db_file_path.to_string_lossy().to_string();
                    }
                }
                StoreKind::Grpc => {
                    ui.horizontal(|ui| {
                        ui.label("server: ");
                        ui.text_edit_singleline(&mut self.param);
                    });
                }
            }

            ui.separator();

            if ui.button("Start").clicked() {
                let store = match self.store {
                    StoreKind::Memory => StoreType::Memory,
                    #[cfg(feature = "sqlite")]
                    StoreKind::Sqlite => StoreType::Sqlite(self.param.clone()),
                    StoreKind::Grpc => StoreType::Grpc(self.param.clone()),
                };
                ui_reqs.push(UIRequest::Start(store));
            }
            ui.separator();
            if let Some(e) = &self.error_text {
                ui.label("{e}");
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
            param: String::default(),
            file_dialog: FileDialog::new(),
            error_text: None,
        }
    }

    fn handle_ui_response(&self, resps: Vec<UIResponse>) {
        todo!();
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
