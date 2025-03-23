use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

use timesman_bstore::{GrpcStore, RamStore, Store, StoreType};

#[cfg(feature = "sqlite")]
use timesman_bstore::SqliteStore;

use tokio::runtime;

use crate::log::tmlog;
use crate::pane::{
    create_select_pane, create_times_pane, init_pane, PaneModel, PaneRequest,
    PaneResponse,
};

pub struct App {
    pane_stack: VecDeque<Box<dyn PaneModel>>,
    msg_resp: Vec<PaneResponse>,
    rt: runtime::Runtime,
    store: Option<Arc<Mutex<dyn Store>>>,
}

fn log(text: String) {
    tmlog(format!("app {}", text));
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
            store: None,
        }
    }

    fn create_store(
        &self,
        stype: StoreType,
    ) -> Result<Arc<Mutex<dyn Store>>, String> {
        let store: Arc<Mutex<dyn Store>> = match stype {
            StoreType::Memory => Arc::new(Mutex::new(RamStore::new())),
            #[cfg(feature = "sqlite")]
            StoreType::Sqlite(db_file_path) => Arc::new(Mutex::new(
                //TODO make user selectable to create or use exists database.
                self.rt.block_on(async {
                    SqliteStore::new(&db_file_path, false).await
                }),
            )),
            #[cfg(feature = "grpc")]
            StoreType::Grpc(server) => self.rt.block_on(async {
                Arc::new(Mutex::new(GrpcStore::new(server).await))
            }),
        };

        Ok(store)
    }

    fn handle_pane_event(
        &mut self,
        req: PaneRequest,
        name: &String,
    ) -> Result<(), String> {
        log(format!("{:?}", req));

        match req {
            PaneRequest::Close => {
                self.pane_stack.pop_front();
            }
            PaneRequest::SelectTimes(tid) => {
                if let Some(_) = &self.store {
                    let pane = create_times_pane(tid);
                    self.pane_stack.push_front(pane);
                } else {
                    todo!();
                }
            }
            PaneRequest::SelectStore(stype) => {
                self.store = Some(self.create_store(stype)?);
                let pane = create_select_pane();
                self.pane_stack.push_front(pane);
            }
            PaneRequest::CreateTimes(title) => {
                todo!();
            }
            PaneRequest::CreatePost(text) => {
                todo!();
            }
            PaneRequest::Log(text) => {
                tmlog(format!("{name} {text}"));
            }
        }

        Ok(())
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
                Ok(()) => {}
                Err(e) => {
                    self.msg_resp.push(PaneResponse::Err(e));
                }
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
}
