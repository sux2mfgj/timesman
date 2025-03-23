use super::{PaneModel, PaneRequest};
use crate::log::tmlog;
use crate::pane::select_ui::{SelectPaneTrait, UIRequest, UIResponse};
use timesman_type::Times;

use tokio::runtime::Runtime;

const PANE_NAME: &str = "SelectPane";

pub struct SelectPaneModel {
    pane: Box<dyn SelectPaneTrait>,
    ui_resps: Vec<UIResponse>,
    times_list: Vec<Times>,
}

fn log(text: String) {
    tmlog(format!("{} {}", PANE_NAME.to_string(), text));
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
            let (uresp, preq) = self.handle_ui_request(rt, r);

            if let Some(uresp) = uresp {
                todo!();
            }
            if let Some(preq) = preq {
                preqs.push(preq);
            }
        }

        Ok(preqs)
    }

    fn get_name(&self) -> String {
        PANE_NAME.to_string()
    }
}

impl SelectPaneModel {
    pub fn new(pane: Box<dyn SelectPaneTrait>) -> Self {
        Self {
            pane,
            ui_resps: vec![],
            times_list: vec![],
        }
    }

    fn handle_ui_request(
        &mut self,
        rt: &Runtime,
        req: UIRequest,
    ) -> (Option<UIResponse>, Option<PaneRequest>) {
        match req {
            UIRequest::Close => (None, Some(PaneRequest::Close)),
            UIRequest::SelectTimes(tid) => {
                log(format!("The times is selected {}", tid));

                (None, Some(PaneRequest::SelectTimes(tid)))
            }
            UIRequest::CreateTimes(title) => {
                let req = PaneRequest::CreateTimes(title);
                (None, Some(req))
            }
        }
    }
}
