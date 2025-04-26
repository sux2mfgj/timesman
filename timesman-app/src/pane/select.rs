use super::{PaneModel, PaneRequest, PaneResponse, TimesInfo};
use crate::pane::select_ui::{SelectPaneTrait, UIRequest, UIResponse};

const PANE_NAME: &str = "SelectPane";

pub struct SelectPaneModel {
    pane: Box<dyn SelectPaneTrait>,
    uresps: Vec<UIResponse>,
    times_list: Vec<TimesInfo>,
}

impl PaneModel for SelectPaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        presps: &Vec<PaneResponse>,
    ) -> Result<Vec<PaneRequest>, String> {
        let mut preqs = vec![];
        for resp in presps {
            match resp {
                PaneResponse::NewTimes(tinfo, select) => {
                    self.times_list.push(tinfo.clone());
                    if *select {
                        preqs.push(PaneRequest::SelectTimes(tinfo.times.id));
                    }
                }
                PaneResponse::Err(e) => {
                    preqs.push(PaneRequest::Log(format!("{e}")));
                }
                _ => {
                    todo!("unknown response found");
                }
            }
        }

        let reqs = self
            .pane
            .update(
                ctx,
                &self.uresps,
                &self.times_list, // &self.times_list.iter().map(|t| t.times.clone()).collect(),
            )
            .unwrap();

        for r in reqs {
            let uresp = self.handle_ui_request(r, &mut preqs);

            if let Some(_uresp) = uresp {
                todo!();
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
            uresps: vec![],
            times_list: vec![],
        }
    }

    fn handle_ui_request(
        &mut self,
        req: UIRequest,
        preqs: &mut Vec<PaneRequest>,
    ) -> Option<UIResponse> {
        match req {
            UIRequest::Close => {
                preqs.push(PaneRequest::Close);
                None
            }
            UIRequest::SelectTimes(tid) => {
                preqs.push(PaneRequest::SelectTimes(tid));
                None
            }
            UIRequest::CreateTimes(title) => {
                preqs.push(PaneRequest::CreateTimes(title));
                None
            }
        }
    }
}
