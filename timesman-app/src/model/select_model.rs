use super::select_ui::{SelectUI, Sort, UIRequest, UIResponse};
use super::{AppRequest, Model, State};

use std::fmt::Debug;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;

use tokio::sync::Mutex;

use timesman_bstore::{Store, TimesStore};
use timesman_type::{Tid, Times};
use tokio::runtime::Runtime;

enum AsyncEvent {
    AddTimes((Times, Arc<Mutex<dyn TimesStore>>)),
    SelectTimes(Tid),
    Close,
    Err(String),
}

impl Debug for AsyncEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncEvent::AddTimes((times, _)) => {
                write!(f, "AddTimes {:?}", times)
            }
            AsyncEvent::SelectTimes(tid) => {
                write!(f, "SelectTimes {}", tid)
            }
            AsyncEvent::Close => {
                write!(f, "Cloes")
            }
            AsyncEvent::Err(e) => {
                write!(f, "Err {e}")
            }
        }
    }
}

struct TimesPack {
    times: Times,
    tstore: Arc<Mutex<dyn TimesStore>>,
}

pub struct SelectModel {
    store: Arc<Mutex<dyn Store>>,
    ui: SelectUI,

    times: Vec<TimesPack>,
    tx: Sender<AsyncEvent>,
    rx: Receiver<AsyncEvent>,

    uresp: Vec<UIResponse>,
}

async fn load_times(
    store: Arc<Mutex<dyn Store + Send + Sync>>,
    tx: &Sender<AsyncEvent>,
) -> Result<(), String> {
    let mut store = store.lock().await; //.map_err(|e| format!("{e}"))?;

    store.check().await?;

    let tstores = store.get().await?;

    for tstore in tstores {
        let t = tstore.clone();
        let mut ts = tstore.lock().await; //.map_err(|e| format!("{e}"))?;
        let times = ts.get().await?;
        tx.send(AsyncEvent::AddTimes((times, t))).unwrap();
    }

    Ok(())
}

impl SelectModel {
    pub fn new(
        store: Arc<Mutex<dyn Store + Send + Sync>>,
        rt: &Runtime,
    ) -> Self {
        let ui = SelectUI::new();
        let times = vec![];
        let (tx, rx) = channel();

        {
            let store = store.clone();
            let tx = tx.clone();
            rt.spawn(async move {
                match load_times(store, &tx).await {
                    Ok(()) => {}
                    Err(e) => {
                        tx.send(AsyncEvent::Err(e)).unwrap();
                    }
                }
            });
        }

        let uresp = vec![];

        Self {
            store,
            ui,
            times,
            tx,
            rx,
            uresp,
        }
    }

    fn handle_ureqs(&mut self, rt: &Runtime, ureqs: Vec<UIRequest>) {
        for r in ureqs {
            println!("{:?}", r);
            match r {
                UIRequest::CreateTimes(title) => {
                    let store = self.store.clone();
                    let tx = self.tx.clone();

                    rt.spawn(async move {
                        let mut store = store.lock().await;

                        let tstore =
                            store.create(title.to_string()).await.unwrap();

                        let times = {
                            let mut tstore = tstore.lock().await;
                            tstore.get().await.unwrap()
                        };

                        let tid = times.id;

                        tx.send(AsyncEvent::AddTimes((times, tstore))).unwrap();
                        tx.send(AsyncEvent::SelectTimes(tid)).unwrap();
                    });
                }
                UIRequest::SelectTimes(tid) => {
                    let tx = self.tx.clone();
                    tx.send(AsyncEvent::SelectTimes(tid)).unwrap();
                }
                UIRequest::Close => {
                    let tx = self.tx.clone();
                    tx.send(AsyncEvent::Close).unwrap();
                }
                UIRequest::Sort(key, reverse) => match key {
                    Sort::ID => {
                        if reverse {
                            self.times.sort_by(|a, b| {
                                a.times.id.cmp(&b.times.id).reverse()
                            })
                        } else {
                            self.times
                                .sort_by(|a, b| a.times.id.cmp(&b.times.id));
                        }
                    }
                    Sort::Name => {
                        todo!();
                    }
                },
            }
        }
    }

    fn handle_async_events(&mut self, areq: &mut Vec<AppRequest>) {
        loop {
            match self.rx.try_recv() {
                Ok(resp) => {
                    println!("{:?}", resp);
                    match resp {
                        AsyncEvent::Err(e) => {
                            todo!();
                        }
                        AsyncEvent::AddTimes((times, tstore)) => {
                            self.times.push(TimesPack { times, tstore });
                        }
                        AsyncEvent::SelectTimes(tid) => {
                            let Some(tp) =
                                self.times.iter().find(|tp| tp.times.id == tid)
                            else {
                                self.uresp.push(UIResponse::SelectErr(
                                    format!("id({}) is not found", tid),
                                ));
                                continue;
                            };

                            self.uresp.push(UIResponse::SelectOk);

                            areq.push(AppRequest::ChangeState(State::ToTimes(
                                tp.tstore.clone(),
                            )));
                        }
                        AsyncEvent::Close => {
                            areq.push(AppRequest::ChangeState(State::Back));
                        }
                    }
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(e) => {
                    todo!();
                }
            }
        }
    }
}

impl Model for SelectModel {
    fn update(
        &mut self,
        ctx: &egui::Context,
        rt: &tokio::runtime::Runtime,
        resp: Vec<crate::app::AppResponse>,
    ) -> Result<Vec<crate::app::AppRequest>, String> {
        let mut req = vec![];

        let times = self.times.iter().map(|t| t.times.clone()).collect();
        let ureqs = self.ui.update(ctx, &times, &self.uresp).unwrap();

        self.uresp.clear();

        self.handle_ureqs(rt, ureqs);
        self.handle_async_events(&mut req);

        Ok(req)
    }
}
