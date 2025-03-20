use std::{rc::Rc, sync::Mutex};

use timesman_bstore::{Store, StoreType};
use timesman_type::Tid;

mod select;
mod select_ui;
mod start;
mod start_ui;
mod times;
mod times_ui;

pub use select::SelectPaneModel;
pub use start::StartPaneModel;
pub use times::TimesPaneModel;

use tokio::runtime::Runtime;

pub enum PaneRequest {
    Close,
    SelectStore(StoreType),
    SelectTimes(Rc<Mutex<dyn Store>>, Tid),
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
    Box::new(StartPaneModel::new(pane))
}

pub fn create_select_pane(
    store: Rc<Mutex<dyn Store>>,
    rt: &Runtime,
) -> Box<dyn PaneModel> {
    let pane = Box::new(select_ui::SelectPane::new());
    Box::new(SelectPaneModel::new(store, pane, rt))
}

pub fn create_times_pane(
    store: Rc<Mutex<dyn Store>>,
    tid: Tid,
    rt: &Runtime,
) -> Box<dyn PaneModel> {
    let pane = Box::new(times_ui::TimesPane::new());
    Box::new(TimesPaneModel::new(pane, store, tid, rt))
}
