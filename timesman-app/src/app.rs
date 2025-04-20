use egui::Vec2;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

use timesman_bstore::{GrpcStore, RamStore, Store, StoreType};

#[cfg(feature = "sqlite")]
use timesman_bstore::SqliteStore;

use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use tokio::runtime;

use crate::arbiter::ArbiterStore;
use crate::config::Config;
use crate::log::tmlog;
use crate::pane::{
    create_select_pane, create_times_pane, init_pane, PaneModel, PaneRequest,
    PaneResponse, TimesInfo, UIRequest,
};

pub struct App {
    pane_stack: VecDeque<Box<dyn PaneModel>>,
    reqs: Vec<PaneRequest>,
    resp: Vec<PaneResponse>,
    rt: runtime::Runtime,
    store: Option<Arc<Mutex<dyn Store>>>,
    rx: Receiver<PaneResponse>,
    tx: Sender<PaneResponse>,
}

fn log(text: String) {
    tmlog(format!("app {}", text));
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        let mut pane_stack = VecDeque::new();
        pane_stack.push_front(init_pane());

        let resp = vec![];

        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let (tx, rx) = channel();

        let reqs = config.generate_pane_reqs();

        Self {
            pane_stack,
            resp,
            reqs,
            rt,
            store: None,
            rx,
            tx,
        }
    }

    fn create_store(
        &self,
        stype: StoreType,
    ) -> Result<Arc<Mutex<dyn Store>>, String> {
        let store: Arc<Mutex<dyn Store>> = match stype {
            StoreType::Memory => Arc::new(Mutex::new(RamStore::new())),
            #[cfg(feature = "sqlite")]
            StoreType::Sqlite(db_file_path, file_path) => Arc::new(Mutex::new(
                //TODO make user selectable to create or use exists database.
                self.rt.block_on(async {
                    SqliteStore::new(
                        &format!("//{db_file_path}?mode=rwc"),
                        file_path,
                    )
                    .await
                    .unwrap()
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
        ctx: &egui::Context,
        name: &String,
    ) -> Result<(), String> {
        log(format!("{:?}", req));

        match req {
            PaneRequest::Close => {
                self.pane_stack.pop_front();
            }
            PaneRequest::SelectTimes(tid) => {
                let Some(store) = self.store.clone() else {
                    todo!();
                };

                let pane = create_times_pane(tid);
                self.pane_stack.push_front(pane);

                let tx = self.tx.clone();
                self.rt.spawn(async move {
                    let mut store = store.lock().await;
                    let posts = store.get_posts(tid).await.unwrap();
                    for p in posts {
                        tx.send(PaneResponse::PostCreated(p.clone())).unwrap();
                    }
                });
            }
            PaneRequest::SelectStore(stype, server) => {
                let mut store = self.create_store(stype)?;
                if let Some(server) = server {
                    tmlog(format!("Start server: {server}"));
                    let s = ArbiterStore::new(&self.rt, store, &server);
                    store = Arc::new(Mutex::new(s));
                };
                self.store = Some(store);
                let pane = create_select_pane();
                self.pane_stack.push_front(pane);
            }
            PaneRequest::CreateTimes(title) => {
                let Some(store) = self.store.clone() else {
                    todo!();
                };

                let tx = self.tx.clone();

                self.rt.spawn(async move {
                    let mut store = store.lock().await;
                    let times = store.create_times(title).await.unwrap();
                    let nposts = store.get_posts(times.id).await.unwrap().len();
                    tx.send(PaneResponse::NewTimes(
                        TimesInfo { times, nposts },
                        true,
                    ))
                    .unwrap();
                });
            }
            PaneRequest::GetTimes => {
                let Some(store) = self.store.clone() else {
                    todo!();
                };

                let tx = self.tx.clone();

                self.rt.spawn(async move {
                    let mut store = store.lock().await;
                    let times = store.get_times().await.unwrap();
                    for t in times {
                        let nposts = store.get_posts(t.id).await.unwrap().len();
                        tx.send(PaneResponse::NewTimes(
                            TimesInfo {
                                times: t.clone(),
                                nposts,
                            },
                            false,
                        ))
                        .unwrap();
                    }
                });
            }
            PaneRequest::CreatePost(tid, text, file) => {
                let Some(store) = self.store.clone() else {
                    todo!();
                };

                let tx = self.tx.clone();

                self.rt.spawn(async move {
                    let mut store = store.lock().await;
                    let post =
                        store.create_post(tid, text, file).await.unwrap();
                    tx.send(PaneResponse::PostCreated(post)).unwrap();
                });
            }
            PaneRequest::UI(op) => match op {
                UIRequest::ChangeScale(scale) => ctx.set_zoom_factor(scale),
                UIRequest::ChangeWindowSize(h, w) => ctx.send_viewport_cmd(
                    egui::ViewportCommand::InnerSize(Vec2::new(h, w)),
                ),
            },
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

        let mut reqs = match pane.update(ctx, &self.resp) {
            Ok(reqs) => reqs,
            Err(e) => {
                todo!("{e}");
            }
        };

        self.resp = vec![];

        for r in [&reqs[..], &self.reqs[..]].concat() {
            match self.handle_pane_event(r, ctx, &name) {
                Ok(()) => {}
                Err(e) => {
                    self.resp.push(PaneResponse::Err(e));
                }
            }
        }
        self.reqs.clear();

        loop {
            match self.rx.try_recv() {
                Ok(resp) => self.resp.push(resp),
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    log(format!("{e}"));
                    todo!();
                }
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn raw_input_hook(
        &mut self,
        _ctx: &egui::Context,
        raw_input: &mut egui::RawInput,
    ) {
        for f in &raw_input.dropped_files {
            tmlog(format!("{:?}", f));
            let Some(path) = &f.path else {
                continue;
            };

            self.resp.push(PaneResponse::FileDropped(path.clone()));
        }
    }
}
