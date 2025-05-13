use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::times_ui::{TimesUI, UIRequest, UIResponse};
use super::{AppRequest, AppResponse, Model, State};
use serde::Serialize;

use timesman_bstore::{PostStore, TimesStore};
use timesman_type::{Post, Times};
use tokio::runtime::Runtime;

#[derive(Debug)]
enum AsyncEvent {
    AddPost(Post),
    Err(String),
}

pub struct TimesModel {
    ui: TimesUI,
    tstore: Arc<Mutex<dyn TimesStore>>,
    pstore: Arc<Mutex<dyn PostStore>>,
    posts: Vec<Post>,
    aetx: Sender<AsyncEvent>,
    aerx: Receiver<AsyncEvent>,
    urtx: Sender<UIResponse>,
    urrx: Receiver<UIResponse>,
}

async fn load_posts(
    pstore: Arc<Mutex<dyn PostStore>>,
    tx: &Sender<AsyncEvent>,
) {
    let mut pstore = pstore.lock().await;

    let posts = pstore.get_all().await.unwrap();

    for post in posts {
        tx.send(AsyncEvent::AddPost(post.clone())).unwrap();
    }
}

impl TimesModel {
    pub fn new(tstore: Arc<Mutex<dyn TimesStore>>, rt: &Runtime) -> Self {
        let posts = vec![];

        let (aetx, aerx) = channel();

        let pstore = {
            let tstore = tstore.clone();

            rt.block_on(async move {
                let mut tstore = tstore.lock().await;
                let pstore = tstore.pstore().await.unwrap();
                pstore
            })
        };

        {
            let pstore = pstore.clone();
            let tx = aetx.clone();

            rt.spawn(async move { load_posts(pstore, &tx).await });
        }

        let (urtx, urrx) = channel();

        let times = {
            let tstore = tstore.clone();
            rt.block_on(async move {
                let mut tstore = tstore.lock().await;
                tstore.get().await.unwrap()
            })
        };

        let ui = TimesUI::new(times.title);

        Self {
            ui,
            tstore,
            pstore,
            posts,
            aetx,
            aerx,
            urtx,
            urrx,
        }
    }

    fn handle_ureqs(
        &mut self,
        ureq: Vec<UIRequest>,
        areq: &mut Vec<AppRequest>,
        rt: &Runtime,
    ) {
        for req in ureq {
            println!("{:?}", req);
            match req {
                UIRequest::Post(content, file) => {
                    let pstore = self.pstore.clone();

                    let aetx = self.aetx.clone();
                    let urtx = self.urtx.clone();
                    rt.spawn(async move {
                        let mut pstore = pstore.lock().await;
                        let post = pstore
                            .post(content.clone(), file.clone())
                            .await
                            .unwrap();
                        aetx.send(AsyncEvent::AddPost(post)).unwrap();
                        urtx.send(UIResponse::ClearText).unwrap();
                    });
                }
                UIRequest::Dump(path) => {
                    let tstore = self.tstore.clone();
                    let posts = self.posts.clone();
                    let mut file = fs::File::create(path).unwrap();
                    rt.spawn(async move {
                        let mut tstore = tstore.lock().await;
                        let times = tstore.get().await.unwrap();

                        let dump = DumpData { times, posts };
                        serde_json::to_writer(&mut file, &dump).unwrap();
                    });
                }
                UIRequest::Close => {
                    areq.push(AppRequest::ChangeState(State::Back));
                }
            }
        }
    }

    fn handle_async_event(&mut self, areq: &mut Vec<AppRequest>) {
        loop {
            match self.aerx.try_recv() {
                Ok(event) => match event {
                    AsyncEvent::AddPost(post) => {
                        self.posts.push(post);
                        self.posts.sort_by(|a, b| a.id.cmp(&b.id));
                    }
                    AsyncEvent::Err(e) => {
                        areq.push(AppRequest::Err(e));
                    }
                },
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    areq.push(AppRequest::Err(format!("{e}")));
                }
            }
        }
    }

    fn gen_ures_vec(&self) -> Vec<UIResponse> {
        let mut ures = vec![];
        loop {
            match self.urrx.try_recv() {
                Ok(res) => {
                    ures.push(res);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    todo!();
                }
            }
        }

        ures
    }

    fn handle_app_resp(
        &self,
        resp: &Vec<AppResponse>,
        ures: &mut Vec<UIResponse>,
    ) {
        for r in resp {
            match r {
                AppResponse::FileDropped(path) => {
                    ures.push(UIResponse::FileDropped(path.clone()));
                }
            }
        }
    }
}

impl Model for TimesModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        rt: &Runtime,
        resp: Vec<AppResponse>,
    ) -> Result<Vec<AppRequest>, String> {
        let mut areqs = vec![];

        let mut ures = self.gen_ures_vec();
        self.handle_app_resp(&resp, &mut ures);

        let ureqs = self.ui.update(ctx, &self.posts, ures);

        self.handle_ureqs(ureqs, &mut areqs, rt);
        self.handle_async_event(&mut areqs);

        Ok(areqs)
    }
}

#[derive(Serialize)]
struct DumpData {
    times: Times,
    posts: Vec<Post>,
}
