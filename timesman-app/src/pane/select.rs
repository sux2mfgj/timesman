use std::{rc::Rc, sync::Mutex};

use super::{PaneModel, PaneRequest};
use crate::log::tmlog;
use crate::pane::select_ui::{SelectPaneTrait, UIRequest, UIResponse};
use timesman_bstore::Store;
use timesman_type::Times;

use tokio::runtime::Runtime;

const PANE_NAME: &str = "SelectPane";

pub struct SelectPaneModel {
    store: Rc<Mutex<dyn Store>>,
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
    pub fn new(
        store: Rc<Mutex<dyn Store>>,
        pane: Box<dyn SelectPaneTrait>,
        rt: &Runtime,
    ) -> Self {
        let times = {
            let store = store.clone();
            let times = rt
                .block_on(async move {
                    let mut store = store.lock().unwrap();
                    store.get_times().await
                })
                .unwrap();
            times
        };

        Self {
            store,
            pane,
            ui_resps: vec![],
            times_list: times,
        }
    }

    fn handle_ui_request(
        &mut self,
        rt: &Runtime,
        req: UIRequest,
    ) -> (Option<UIResponse>, Option<PaneRequest>) {
        match req {
            UIRequest::SelectTimes(tid) => {
                tmlog(format!(
                    "{} The times is selected {}",
                    PANE_NAME.to_string(),
                    tid
                ));

                (None, Some(PaneRequest::SelectTimes(tid)))
            }
            UIRequest::CreateTimes(title) => {
                let times = {
                    let store = self.store.clone();
                    let title = title.clone();
                    rt.block_on(async move {
                        let mut store = store.lock().unwrap();
                        store.create_times(title).await
                    })
                    .unwrap()
                };

                tmlog(format!(
                    "{} A new times is created named {}",
                    PANE_NAME.to_string(),
                    title
                ));

                self.times_list.push(times);

                (None, None)
            }
        }
    }
}
