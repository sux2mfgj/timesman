use timesman_bstore::{Store, StoreType};
use timesman_type::Tid;

pub mod select;
pub use select::SelectPaneModel;
mod select_ui;
pub mod start;
pub use start::StartPaneModel;
mod start_ui;

use tokio::runtime::Runtime;

#[derive(Debug, PartialEq)]
pub enum PaneRequest {
    Close,
    SelectStore(StoreType),
    SelectTimes(Tid),
    Log(String),
}

#[derive(Debug, PartialEq)]
pub enum PaneResponse {
    Err(String),
    Ok,
}

pub trait PaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg_resp: &Vec<PaneResponse>,
        rt: &Runtime,
    ) -> Result<Vec<PaneRequest>, String>;

    fn get_name(&self) -> String;
}

pub fn init_pane() -> Box<dyn PaneModel> {
    let pane = Box::new(start_ui::StartPane::new());
    Box::new(start::StartPaneModel::new(pane))
}

pub fn create_select_pane(
    store: Box<dyn Store>,
    rt: &Runtime,
) -> Box<dyn PaneModel> {
    let pane = Box::new(select_ui::SelectPane::new());
    Box::new(select::SelectPaneModel::new(store, pane, rt))
}
