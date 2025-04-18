use std::path::PathBuf;

use timesman_bstore::{Store, StoreType};
use timesman_type::{File, Pid, Post, Tid, Times};

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

pub enum PaneRequest {
    Close,
    SelectStore(StoreType, Option<String>),
    SelectTimes(Tid),
    CreateTimes(String),
    CreatePost(Pid, String, Option<(String, File)>), //filename and file path
    Log(String),
}

impl std::fmt::Debug for PaneRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaneRequest::Close => {
                write!(f, "Close")
            }
            PaneRequest::SelectStore(stype, opt) => {
                write!(f, "SelectStore {:?}", stype)
            }
            PaneRequest::SelectTimes(tid) => {
                write!(f, "SelectTimes {tid}")
            }
            PaneRequest::CreateTimes(name) => {
                write!(f, "CreateTimes {name}")
            }
            PaneRequest::CreatePost(pid, text, file) => {
                let fname = if let Some(file) = file {
                    file.0.clone()
                } else {
                    "".to_string()
                };
                write!(f, "CreatePost {pid} {text} {fname}")
            }
            PaneRequest::Log(log) => {
                write!(f, "Log {log}")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PaneResponse {
    TimesCreated(Times),
    PostCreated(Post),
    Err(String),
    FileDropped(PathBuf),
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
