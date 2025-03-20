use super::{PaneModel, PaneRequest};
use crate::pane::select_ui::{SelectPaneTrait, UIRequest, UIResponse};
use timesman_bstore::Store;
use timesman_type::Times;

use tokio::runtime::Runtime;

pub struct SelectPaneModel {
    store: Box<dyn Store>,
    pane: Box<dyn SelectPaneTrait>,
    ui_resps: Vec<UIResponse>,
    times_list: Vec<Times>,
}

impl PaneModel for SelectPaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg_resp: &Vec<super::PaneResponse>,
        rt: &Runtime,
    ) -> Result<Vec<PaneRequest>, String> {
        let reqs = self
            .pane
            .update(ctx, &self.ui_resps, &self.times_list)
            .unwrap();

        let mut preqs = vec![];
        for r in reqs {
            if let Some(preq) = self.handle_ui_request(r) {
                preqs.push(preq);
            }
        }

        Ok(preqs)
    }
}

impl SelectPaneModel {
    pub fn new(
        mut store: Box<dyn Store>,
        pane: Box<dyn SelectPaneTrait>,
        rt: &Runtime,
    ) -> Self {
        let times = rt.block_on(async { store.get_times().await }).unwrap();

        Self {
            store,
            pane,
            ui_resps: vec![],
            times_list: times,
        }
    }

    fn handle_ui_request(&self, req: UIRequest) -> Option<PaneRequest> {
        match req {
            UIRequest::SelectTimes(tid) => Some(PaneRequest::SelectTimes(tid)),
        }
    }
}
