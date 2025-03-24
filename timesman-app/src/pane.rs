use std::{rc::Rc, sync::Mutex};

use timesman_bstore::{Store, StoreType};
use timesman_type::{Pid, Post, Tid, Times};

mod ui;

mod select;
mod select_ui;
mod start;
mod start_ui;
mod times;
mod times_ui;

use select::SelectPaneModel;
use start::StartPaneModel;
use times::TimesPaneModel;

use tokio::runtime::Runtime;

#[derive(Debug)]
pub enum PaneRequest {
    Close,
    SelectStore(StoreType, Option<String>),
    SelectTimes(Tid),
    CreateTimes(String),
    CreatePost(Pid, String),
    Log(String),
}

#[derive(Debug, PartialEq)]
pub enum PaneResponse {
    TimesCreated(Times),
    PostCreated(Post),
    Err(String),
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

pub fn create_select_pane() -> Box<dyn PaneModel> {
    let pane = Box::new(select_ui::SelectPane::new());
    Box::new(SelectPaneModel::new(pane))
}

pub fn create_times_pane(tid: Tid) -> Box<dyn PaneModel> {
    let pane = Box::new(times_ui::TimesPane::new());
    Box::new(TimesPaneModel::new(pane, tid))
}
