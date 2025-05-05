use super::start_ui::StartUI;
use super::start_ui::{UIRequest, UIResponse};
use super::{Model, State};

use super::{AppRequest, AppResponse, Runtime};

pub struct StartModel {
    ui: StartUI,
    uresp: Vec<UIResponse>,
}

impl StartModel {
    pub fn new() -> Self {
        let uresp = vec![];
        Self {
            ui: StartUI::new(),
            uresp,
        }
    }

    fn handle_ui_requests(
        &mut self,
        ureq: UIRequest,
        areq: &mut Vec<AppRequest>,
        _uresp: &mut Vec<UIResponse>,
    ) {
        match ureq {
            UIRequest::Start(stype, _server) => {
                //TODO: use the server parameter.
                let store = match stype.to_store() {
                    Ok(s) => s,
                    Err(e) => {
                        areq.push(AppRequest::Err(e));
                        return;
                    }
                };

                areq.push(AppRequest::ChangeState(State::ToSelect(store)));
            }
            UIRequest::Close => {
                areq.push(AppRequest::ChangeState(State::Back));
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
        let mut preq = vec![];
        for req in ureqs {
            self.handle_ui_requests(req, &mut preq, &mut uresp);
        }

        self.uresp = uresp;

        Ok(preq)
    }
}
