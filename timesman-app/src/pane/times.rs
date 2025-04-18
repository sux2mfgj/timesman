use super::times_ui::{TimesPaneTrait, UIRequest, UIResponse};
use super::{PaneModel, PaneRequest, PaneResponse};

use timesman_type::{Post, Tid};
use tokio::runtime::Runtime;

pub struct TimesPaneModel {
    pane: Box<dyn TimesPaneTrait>,
    tid: Tid,
    ui_resps: Vec<UIResponse>,
    posts: Vec<Post>,
}

const PANE_NAME: &str = "TimesPane";

impl PaneModel for TimesPaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        p_resps: &Vec<PaneResponse>,
        rt: &Runtime,
    ) -> Result<Vec<PaneRequest>, String> {
        let mut p_reqs = vec![];
        self.ui_resps = vec![];

        for p in p_resps {
            match p {
                PaneResponse::PostCreated(p) => {
                    self.posts.push(p.clone());
                    self.ui_resps.push(UIResponse::PostSuccess);
                }
                PaneResponse::FileDropped(path) => {
                    self.ui_resps.push(UIResponse::FileDropped(path.clone()));
                }
                PaneResponse::Err(e) => {
                    todo!("{e}");
                }
                _ => {
                    todo!("unexpected pane response found");
                }
            }
        }

        let ui_reqs =
            self.pane.update(ctx, &self.ui_resps, &self.posts).unwrap();

        for req in ui_reqs {
            let (ui_resp, p_req) = self.handle_ui_request(req);
            if let Some(resp) = ui_resp {
                self.ui_resps.push(resp);
            }

            if let Some(req) = p_req {
                p_reqs.push(req);
            }
        }

        Ok(p_reqs)
    }

    fn get_name(&self) -> String {
        PANE_NAME.to_string()
    }
}

impl TimesPaneModel {
    pub fn new(pane: Box<dyn TimesPaneTrait>, tid: Tid) -> Self {
        Self {
            pane,
            tid,
            ui_resps: vec![],
            posts: vec![],
        }
    }

    fn handle_ui_request(
        &mut self,
        reqs: UIRequest,
    ) -> (Option<UIResponse>, Option<PaneRequest>) {
        match reqs {
            UIRequest::Post(text, file) => {
                let req = PaneRequest::CreatePost(self.tid, text, file);
                (None, Some(req))
            }
            UIRequest::Close => (None, Some(PaneRequest::Close)),
        }
    }
}
