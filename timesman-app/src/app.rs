use timesman_bstore::{Store, StoreType};

use crate::pane::{init_pane, PaneModel, PaneRequest, PaneResponse};

use std::{collections::VecDeque, rc::Rc, sync::Mutex};

pub struct App {
    pane_stack: VecDeque<Box<dyn PaneModel>>,
    msg_resp: Vec<PaneResponse>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut pane_stack = VecDeque::new();
        pane_stack.push_front(init_pane());

        let msg_resp = vec![];

        Self {
            pane_stack,
            msg_resp,
        }
    }

    fn create_store(
        &self,
        stype: StoreType,
    ) -> Result<Rc<Mutex<dyn Store>>, String> {
        match stype {
            StoreType::Memory => {}
        }

        todo!();
    }

    fn handle_pane_event(
        &mut self,
        req: PaneRequest,
    ) -> Result<PaneResponse, String> {
        match req {
            PaneRequest::Close => {
                self.pane_stack.pop_front();
            }
            PaneRequest::SelectTimes(stype) => {
                let store = self.create_store(stype)?;
                todo!();
                //let pane = Box::new(SelectPaneModel::new())
            }
            PaneRequest::SelectStore => {}
        }

        todo!()
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

        let reqs = match pane.update(ctx, &self.msg_resp) {
            Ok(reqs) => reqs,
            Err(e) => {
                todo!("{e}");
            }
        };

        self.msg_resp = vec![];

        for r in reqs {
            match self.handle_pane_event(r) {
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
