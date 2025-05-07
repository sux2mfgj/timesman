use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

use super::start_ui::StartUI;
use super::start_ui::{UIRequest, UIResponse};
use super::{Model, State};

use super::{AppRequest, AppResponse, Runtime};

pub struct StartModel {
    ui: StartUI,
    uresp: Vec<UIResponse>,
    artx: Sender<AppRequest>,
    arrx: Receiver<AppRequest>,
}

impl StartModel {
    pub fn new() -> Self {
        let uresp = vec![];
        let (artx, arrx) = channel();
        Self {
            ui: StartUI::new(),
            uresp,
            artx,
            arrx,
        }
    }

    fn handle_ui_requests(
        &mut self,
        ureq: UIRequest,
        _uresp: &mut Vec<UIResponse>,
        rt: &Runtime,
    ) {
        match ureq {
            UIRequest::Start(stype, _server) => {
                //TODO: use the server parameter.

                let artx = self.artx.clone();
                rt.spawn(async move {
                    let store = match stype.to_store().await {
                        Ok(s) => s,
                        Err(e) => {
                            artx.send(AppRequest::Err(e)).unwrap();
                            return;
                        }
                    };

                    artx.send(AppRequest::ChangeState(State::ToSelect(store)))
                        .unwrap();
                });
            }
            UIRequest::Close => {
                self.artx.send(AppRequest::ChangeState(State::Back));
            }
        }
    }
}

impl Model for StartModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        rt: &Runtime,
        resp: Vec<AppResponse>,
    ) -> Result<Vec<AppRequest>, String> {
        let ureqs = self.ui.update(ctx, &self.uresp).unwrap();

        let mut uresp = vec![];
        for req in ureqs {
            self.handle_ui_requests(req, &mut uresp, rt);
        }

        self.uresp = uresp;

        let mut preq = vec![];
        loop {
            match self.arrx.try_recv() {
                Ok(r) => {
                    preq.push(r);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {}
            }
        }

        Ok(preq)
    }
}
