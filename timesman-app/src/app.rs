use egui::Vec2;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::model::{
    create_select_model, create_start_model, create_times_model, Model,
};
use std::path::PathBuf;
use timesman_bstore::{Store, TimesStore};
//use timesman_bstore::GrpcStore;

//#[cfg(feature = "sqlite")]
//use timesman_bstore::SqliteStore;

use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use tokio::runtime::Builder;
pub use tokio::runtime::Runtime;

//use crate::arbiter::ArbiterStore;
use crate::config::Config;
use crate::log::tmlog;

#[derive(Clone)]
pub enum AppRequest {
    ChangeState(State),
    UI(UIRequest),
    Err(String),
}

#[derive(Clone)]
pub enum State {
    ToSelect(Arc<Mutex<dyn Store>>),
    ToTimes(Arc<Mutex<dyn TimesStore>>),
    Back,
}

#[derive(Debug, Clone)]
pub enum UIRequest {
    ChangeScale(f32),
    ChangeWindowSize(f32, f32),
}

pub enum AppResponse {
    FileDropped(PathBuf),
}

pub struct App {
    model_stack: VecDeque<Box<dyn Model>>,
    rt: Runtime,
    pub req_rx: Receiver<AppRequest>,
    pub req_tx: Sender<AppRequest>,
    pub resp_rx: Receiver<AppResponse>,
    pub resp_tx: Sender<AppResponse>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        let mut model_stack = VecDeque::new();
        model_stack.push_front(create_start_model());

        let rt = Builder::new_multi_thread().enable_all().build().unwrap();

        let (req_tx, req_rx) = channel();
        let (resp_tx, resp_rx) = channel();

        for r in config.generate_pane_reqs() {
            req_tx.send(r).unwrap();
        }

        Self {
            model_stack,
            rt,
            req_rx,
            req_tx,
            resp_rx,
            resp_tx,
        }
    }

    fn handle_app_request(
        &mut self,
        req: AppRequest,
        ctx: &egui::Context,
    ) -> Result<(), String> {
        match req {
            AppRequest::UI(ui) => {
                match ui {
                    UIRequest::ChangeScale(scale) => ctx.set_zoom_factor(scale),
                    UIRequest::ChangeWindowSize(h, w) => ctx.send_viewport_cmd(
                        egui::ViewportCommand::InnerSize(Vec2::new(h, w)),
                    ),
                }
                Ok(())
            }
            AppRequest::Err(e) => {
                tmlog(format!("{e}"));
                Ok(())
            }
            AppRequest::ChangeState(s) => match s {
                State::Back => {
                    let Some(_) = self.model_stack.pop_front() else {
                        todo!();
                    };

                    Ok(())
                }
                State::ToSelect(store) => {
                    let model = create_select_model(store, &self.rt);
                    self.model_stack.push_front(model);
                    Ok(())
                }
                State::ToTimes(tstore) => {
                    let model = create_times_model(tstore, &self.rt);
                    self.model_stack.push_front(model);
                    Ok(())
                }
            },
        }
    }
    /*
    fn create_store(
        &self,
        stype: StoreType,
    ) -> Result<Arc<Mutex<dyn TimesStore>>, String> {
        let store: Arc<Mutex<dyn TimesStore>> = match stype {
            StoreType::Memory => Arc::new(Mutex::new(RamStore::new())),
            //#[cfg(feature = "sqlite")]
            //StoreType::Sqlite(db_file_path, file_path) => Arc::new(Mutex::new(
            //    //TODO make user selectable to create or use exists database.
            //    self.rt.block_on(async {
            //        SqliteStore::new(
            //            &format!("//{db_file_path}?mode=rwc"),
            //            file_path,
            //        )
            //        .await
            //        .unwrap()
            //    }),
            //)),
            //#[cfg(feature = "grpc")]
            //StoreType::Grpc(server) => self.rt.block_on(async {
            //    Arc::new(Mutex::new(GrpcStore::new(server).await))
            //}),
        };

        Ok(store)
    }

    fn handle_select_store(
        &mut self,
        stype: StoreType,
        server: Option<String>,
    ) -> Result<(), String> {
        let mut store = self.create_store(stype)?;
        if let Some(server) = server {
            todo!();
            //tmlog(format!("Start server: {server}"));
            //let s = ArbiterStore::new(&self.rt, store, &server);
            //store = Arc::new(Mutex::new(s));
        };
        self.store = Some(store.clone());
        let pane = create_select_pane();
        self.pane_stack.push_front(pane);

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

        Ok(())
    }

    fn handle_pane_event(
        &mut self,
        req: PaneRequest,
        ctx: &egui::Context,
        name: &String,
    ) -> Result<(), String> {

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
                    let tinfo = store.get(tid).await.unwrap();

                    //let posts = store.get_posts(tid).await.unwrap();
                    // let posts = tinfo.pstore.get_all().await.unwrap();
                    let pstore = tinfo.pstore.lock().unwrap();
                    let posts = pstore.get_all().await.unwrap();
                    for p in posts {
                        tx.send(PaneResponse::PostCreated(p.clone())).unwrap();
                    }
                });
            }
            PaneRequest::SelectStore(stype, server) => {
                todo!();
                //self.handle_select_store(stype, server)?;
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
        */
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let model: &mut Box<dyn Model> = match self.model_stack.front_mut() {
            Some(model) => model,
            None => {
                todo!("Shutdown app");
            }
        };

        let mut resps = vec![];
        loop {
            match self.resp_rx.try_recv() {
                Ok(resp) => resps.push(resp),
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    tmlog(format!("{e}"));
                }
            }
        }

        let reqs = match model.update(ctx, &self.rt, resps) {
            Ok(reqs) => reqs,
            Err(e) => {
                vec![AppRequest::Err(e)]
            }
        };

        for r in reqs {
            self.req_tx.send(r).unwrap();
        }

        loop {
            match self.req_rx.try_recv() {
                Ok(req) => {
                    self.handle_app_request(req, ctx).unwrap();
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    tmlog(format!("{e}"));
                }
            }
        }

        /*

        let name = pane.get_name().clone();

        let reqs = match pane.update(ctx, &self.resp) {
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
                    todo!();
                }
            }
        }
        */
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

            self.resp_tx
                .send(AppResponse::FileDropped(path.clone()))
                .unwrap();
        }
    }
}
