use super::{PaneModel, PaneRequest, PaneResponse};
use crate::log::tmlog;
use crate::pane::select_ui::{SelectPaneTrait, UIRequest, UIResponse};
use timesman_type::Times;

use tokio::runtime::Runtime;

const PANE_NAME: &str = "SelectPane";

pub struct SelectPaneModel {
    pane: Box<dyn SelectPaneTrait>,
    uresps: Vec<UIResponse>,
    preqs: Vec<PaneRequest>,
    times_list: Vec<Times>,
}

fn log(text: String) {
    tmlog(format!("{} {}", PANE_NAME.to_string(), text));
}

impl PaneModel for SelectPaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        presps: &Vec<PaneResponse>,
    ) -> Result<Vec<PaneRequest>, String> {
        for resp in presps {
            match resp {
                PaneResponse::NewTimes(times, select) => {
                    self.times_list.push(times.clone());
                    if *select {
                        self.preqs.push(PaneRequest::SelectTimes(times.id));
                    }
                }
                PaneResponse::Err(e) => {
                    log(format!("{e}"));
                    todo!();
                }
                _ => {
                    todo!("unknown response found");
                }
            }
        }

        let reqs = self
            .pane
            .update(ctx, &self.uresps, &self.times_list)
            .unwrap();

        for r in reqs {
            let (uresp, preq) = self.handle_ui_request(r);

            if let Some(uresp) = uresp {
                todo!();
            }
            if let Some(preq) = preq {
                self.preqs.push(preq);
            }
        }

        let preqs = self.preqs.clone();
        self.preqs.clear();

        Ok(preqs)
    }

    fn get_name(&self) -> String {
        PANE_NAME.to_string()
    }
}

impl SelectPaneModel {
    pub fn new(pane: Box<dyn SelectPaneTrait>) -> Self {
        let preqs = vec![PaneRequest::GetTimes];
        Self {
            pane,
            uresps: vec![],
            preqs,
            times_list: vec![],
        }
    }

    fn handle_ui_request(
        &mut self,
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
