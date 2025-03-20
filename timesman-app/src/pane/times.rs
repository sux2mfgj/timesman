use super::times_ui::{TimesPaneTrait, UIRequest, UIResponse};
use super::{PaneModel, PaneRequest, PaneResponse};
use std::rc::Rc;
use std::sync::Mutex;

use timesman_bstore::Store;
use timesman_type::{Post, Tid};
use tokio::runtime::Runtime;

pub struct TimesPaneModel {
    pane: Box<dyn TimesPaneTrait>,
    store: Rc<Mutex<dyn Store>>,
    tid: Tid,
    ui_resps: Vec<UIResponse>,
    posts: Vec<Post>,
}

const PANE_NAME: &str = "TimesPane";

impl PaneModel for TimesPaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg_resp: &Vec<PaneResponse>,
        rt: &Runtime,
    ) -> Result<Vec<PaneRequest>, String> {
        let mut p_reqs = vec![];

        let ui_reqs =
            self.pane.update(ctx, &self.ui_resps, &self.posts).unwrap();

        self.ui_resps = vec![];

        for req in ui_reqs {
            let (ui_resp, p_req) = self.handle_ui_request(req, rt);
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
    pub fn new(
        pane: Box<dyn TimesPaneTrait>,
        store: Rc<Mutex<dyn Store>>,
        tid: Tid,
        rt: &Runtime,
    ) -> Self {
        let posts = {
            let mut store = store.lock().unwrap();
            rt.block_on(async move { store.get_posts(tid).await })
                .unwrap()
        };
        Self {
            pane,
            store,
            tid,
            ui_resps: vec![],
            posts,
        }
    }

    fn handle_ui_request(
        &mut self,
        reqs: UIRequest,
        rt: &Runtime,
    ) -> (Option<UIResponse>, Option<PaneRequest>) {
        match reqs {
            UIRequest::Post(text) => {
                let store = self.store.clone();
                let tid = self.tid.clone();
                let post = rt
                    .block_on(async move {
                        let mut store = store.lock().unwrap();
                        store.create_post(tid, text).await
                    })
                    .unwrap();

                self.posts.push(post);

                (Some(UIResponse::PostSuccess), None)
            }
        }
    }
}
