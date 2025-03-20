use super::{PaneModel, PaneRequest, PaneResponse};
use timesman_bstore::StoreType;

#[derive(Copy, Clone)]
pub enum UIRequest {
    Close,
    Start(StoreType),
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

        let mut reqs = vec![];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.radio_value(&mut self.store, StoreType::Memory, "Temporay");

            ui.separator();
            if ui.button("Start").clicked() {
                reqs.push(UIRequest::Start(self.store));
            }
        });

        Ok(reqs)
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {}
}
