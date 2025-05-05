mod ui;

mod start_model;
mod start_ui;
use start_model::StartModel;

mod select_model;
mod select_ui;
use select_model::SelectModel;

mod times_model;
mod times_ui;
use times_model::TimesModel;

use tokio::runtime;

use crate::app::{AppRequest, AppResponse, Runtime, State};

use std::sync::Arc;
use tokio::sync::Mutex;

use timesman_bstore::{Store, TimesStore};

pub trait Model {
    fn update(
        &mut self,
        ctx: &egui::Context,
        rt: &Runtime,
        resp: Vec<AppResponse>,
    ) -> Result<Vec<AppRequest>, String>;
}

pub fn create_start_model() -> Box<dyn Model> {
    Box::new(StartModel::new())
}

pub fn create_select_model(
    store: Arc<Mutex<dyn Store + Send + Sync>>,
    rt: &runtime::Runtime,
) -> Box<dyn Model> {
    Box::new(SelectModel::new(store, rt))
}

pub fn create_times_model(
    tstore: Arc<Mutex<dyn TimesStore>>,
    rt: &runtime::Runtime,
) -> Box<dyn Model> {
    Box::new(TimesModel::new(tstore, rt))
}
