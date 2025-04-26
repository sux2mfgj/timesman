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
        _msg_resp: &Vec<PaneResponse>,
    ) -> Result<Vec<PaneRequest>, String> {
        let ureqs = self.pane.update(ctx, &self.ui_resps).unwrap();

        let mut uresps = vec![];
        let mut preqs = vec![];
        for req in ureqs {
            self.handle_ui_requests(req, &mut uresps, &mut preqs)
                .unwrap();
        }
        self.ui_resps = uresps;

        Ok(preqs)
    }

    fn get_name(&self) -> String {
        "StartPane".to_string()
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
        _uresps: &mut Vec<UIResponse>,
        preqs: &mut Vec<PaneRequest>,
    ) -> Result<(), String> {
        match req {
            UIRequest::Close => {
                preqs.push(PaneRequest::Close);
                Ok(())
            }
            UIRequest::Start(stype, server) => {
                preqs.push(PaneRequest::SelectStore(stype, server));
                Ok(())
            }
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
            _ctx: &egui::Context,
            _msg: &Vec<UIResponse>,
        ) -> Result<Vec<UIRequest>, String> {
            let mut ui_resp = vec![];
            let queue = self.test_ui_event_queue.lock().unwrap();

            for event in queue.iter() {
                ui_resp.push(event.clone());
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
    }
}
