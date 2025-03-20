use timesman_bstore::{RamStore, Store, StoreType};
use tokio::runtime;

use crate::log::tmlog;
use crate::pane::{
    create_select_pane, init_pane, PaneModel, PaneRequest, PaneResponse,
};

use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Mutex;

pub struct App {
    pane_stack: VecDeque<Box<dyn PaneModel>>,
    msg_resp: Vec<PaneResponse>,
    rt: runtime::Runtime,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut pane_stack = VecDeque::new();
        pane_stack.push_front(init_pane());

        let msg_resp = vec![];

        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Self {
            pane_stack,
            msg_resp,
            rt,
        }
    }

    fn create_store(
        &self,
        stype: StoreType,
    ) -> Result<Rc<Mutex<dyn Store>>, String> {
        let store = match stype {
            StoreType::Memory => RamStore::new(),
        };

        Ok(Rc::new(Mutex::new((store))))
    }

    fn handle_pane_event(
        &mut self,
        req: PaneRequest,
        name: &String,
    ) -> Result<PaneResponse, String> {
        match req {
            PaneRequest::Close => {
                self.pane_stack.pop_front();
            }
            PaneRequest::SelectTimes(tid) => {
                todo!();
            }
            PaneRequest::SelectStore(stype) => {
                let store = self.create_store(stype)?;
                let pane = create_select_pane(store, &self.rt);
                self.pane_stack.push_front(pane);
            }
            PaneRequest::Log(text) => {
                tmlog(format!("{name} {text}"));
            }
        }

        Ok(PaneResponse::Ok)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let pane: &mut Box<dyn PaneModel> = match self.pane_stack.front_mut() {
            Some(pane) => pane,
            None => {
                todo!("Shutdown app");
            }
        };

        let name = pane.get_name().clone();

        let reqs = match pane.update(ctx, &self.msg_resp, &self.rt) {
            Ok(reqs) => reqs,
            Err(e) => {
                todo!("{e}");
            }
        };

        self.msg_resp = vec![];

        for r in reqs {
            match self.handle_pane_event(r, &name) {
                Ok(resp) => {
                    self.msg_resp.push(resp);
                }
                Err(e) => {
                    todo!("{e}");
                }
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
}
