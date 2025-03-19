use timesman_bstore::StoreType;

pub mod start;
mod start_ui;

#[derive(Debug, PartialEq)]
pub enum PaneRequest {
    Close,
    SelectStore,
    SelectTimes(StoreType),
}

#[derive(Debug, PartialEq)]
pub enum PaneResponse {}

pub trait PaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg_resp: &Vec<PaneResponse>,
    ) -> Result<Vec<PaneRequest>, String>;
}

pub fn init_pane() -> Box<dyn PaneModel + 'static> {
    let pane = Box::new(start_ui::StartPane::new());
    Box::new(start::StartPaneModel::new(pane))
}
