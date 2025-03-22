use super::{ui, PaneModel, PaneRequest, PaneResponse};
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
    store: StoreType,
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
            ui.radio_value(&mut self.store, StoreType::Memory, "Temporay");

            ui.separator();
            if ui.button("Start").clicked() {
                ui_reqs.push(UIRequest::Start(self.store.clone()));
            }
        });

        let r = self.consume_keys(ctx);
        ui_reqs = vec![ui_reqs, r].concat();

        Ok(ui_reqs)
    }
}

impl StartPane {
    pub fn new() -> Self {
        let store = StoreType::default();
        Self { store }
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
