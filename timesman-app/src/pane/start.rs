use super::{PaneModel, PaneRequest, PaneResponse};
use crate::pane::start_ui::{StartPaneTrait, UIRequest, UIResponse};

pub struct StartPaneModel {
    pane: Box<dyn StartPaneTrait>,
    ui_resps: Vec<UIResponse>,
}

impl PaneModel for StartPaneModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg_resp: &Vec<PaneResponse>,
    ) -> Result<Vec<PaneRequest>, String> {
        let reqs = self.pane.update(ctx, &self.ui_resps).unwrap();

        let mut ui_resps = vec![];
        let mut pane_resps = vec![];
        for req in reqs {
            let (ui_resp, pane_resp) = self.handle_ui_requests(req).unwrap();

            if let Some(resp) = ui_resp {
                ui_resps.push(resp);
            }

            if let Some(resp) = pane_resp {
                pane_resps.push(resp);
            }
        }
        self.ui_resps = ui_resps;

        Ok(pane_resps)
    }
}

impl StartPaneModel {
    pub fn new(pane: Box<dyn StartPaneTrait>) -> Self {
        Self {
            pane,
            ui_resps: vec![],
        }
    }

    fn handle_ui_requests(
        &self,
        req: UIRequest,
    ) -> Result<(Option<UIResponse>, Option<PaneRequest>), String> {
        match req {
            UIRequest::Close => Ok((None, Some(PaneRequest::Close))),
            UIRequest::Start => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::sync::Mutex;

    struct DummyStartPane {
        test_ui_event_queue: Rc<Mutex<VecDeque<UIRequest>>>,
    }

    impl StartPaneTrait for DummyStartPane {
        fn update(
            &mut self,
            ctx: &egui::Context,
            msg: &Vec<UIResponse>,
        ) -> Result<Vec<UIRequest>, String> {
            let mut ui_resp = vec![];
            let queue = self.test_ui_event_queue.lock().unwrap();

            for event in queue.iter() {
                ui_resp.push(*event);
            }

            Ok(ui_resp)
        }
    }

    impl DummyStartPane {
        fn new(queue: Rc<Mutex<VecDeque<UIRequest>>>) -> Self {
            Self {
                test_ui_event_queue: queue,
            }
        }
    }

    #[test]
    fn test_close() {
        let ui_event_queue = Rc::new(Mutex::new(VecDeque::new()));
        let pane = Box::new(DummyStartPane::new(ui_event_queue.clone()));
        let mut model = StartPaneModel::new(pane);

        {
            let mut q = ui_event_queue.lock().unwrap();
            q.push_back(UIRequest::Close);
        }
        let ctx = egui::Context::default();
        let reqs = model.update(&ctx, &vec![]).unwrap();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0], PaneRequest::Close);
    }
}
