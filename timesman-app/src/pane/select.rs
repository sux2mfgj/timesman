use std::sync::Arc;
// use std::sync::Mutex;

use crate::app::Event;

use eframe::egui::ScrollArea;
use egui::{Key, Modifiers};
use std::collections::HashMap;
use store::{Post, Store, Times};
use tokio;
use tokio::sync::Mutex;

use super::{pane_menu, Pane};
use tokio::runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

struct TimesData {
    times: Times,
    latest: Option<Post>,
}

pub struct SelectPane {
    times: HashMap<i64, TimesData>,
    new_title: String,
    store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

enum Message {
    Create(Times),
    Refresh(HashMap<i64, TimesData>),
    UpdateLatest(i64, Post),
    Error(String),
}

impl Pane for SelectPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        rt: &runtime::Runtime,
    ) -> Option<Event> {
        let mut event = None;

        if let Some(event) = self.handle_message() {
            return Some(event);
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui) {
                        event = Some(e);
                    }
                });
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("new");
                ui.separator();
                ui.text_edit_singleline(&mut self.new_title);
            });
            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter)) {
                let store = self.store.clone();
                let title = self.new_title.clone();
                let tx = self.tx.clone();
                rt.spawn(async move {
                    let mut store = store.lock().await;

                    match store.create_times(title.clone()).await {
                        Ok(new_times) => {
                            tx.send(Message::Create(new_times)).await.unwrap();
                        }
                        Err(e) => {
                            tx.send(Message::Error(format!("{}", e)))
                                .await
                                .unwrap();
                        }
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height());
            scroll_area.show(ui, |ui| {
                for (_tid, tdata) in &self.times {
                    ui.horizontal(|ui| {
                        ui.label(
                            tdata
                                .times
                                .created_at
                                .format("%Y-%m-%d %H:%M")
                                .to_string(),
                        );

                        ui.separator();
                        if ui.button(&tdata.times.title).clicked() {
                            event = Some(Event::Select(
                                self.store.clone(),
                                tdata.times.clone(),
                            ));
                        }

                        if let Some(latest) = &tdata.latest {
                            ui.separator();
                            ui.label(format!("{}", latest.post));
                            ui.label(
                                latest
                                    .created_at
                                    .format("%Y-%m-%d %H:%M")
                                    .to_string(),
                            );
                        }
                    });
                }
            });
        });

        event
    }

    fn reload(&mut self, rt: &runtime::Runtime) {
        let store = self.store.clone();
        let tx = self.tx.clone();
        rt.spawn(async move {
            {
                let store = store.lock().await;
                let times = store.get_times().await.unwrap();

                let mut map: HashMap<i64, TimesData> = HashMap::new();
                for t in &times {
                    map.insert(
                        t.id,
                        TimesData {
                            times: t.clone(),
                            latest: None,
                        },
                    );
                }

                match tx.send(Message::Refresh(map)).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!(format!("failed to sent message: {}", e));
                    }
                }

                for t in &times {
                    if let Some(latest) =
                        store.get_latest_post(t.id).await.unwrap()
                    {
                        match tx.send(Message::UpdateLatest(t.id, latest)).await
                        {
                            Ok(_) => {}
                            Err(e) => {
                                error!(format!("failed to send message: {e}"));
                            }
                        }
                    }
                }
            }
        });
    }
}

impl SelectPane {
    pub fn new(
        store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
        rt: &runtime::Runtime,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<Message>(32);

        let mut pane = Self {
            times: HashMap::new(),
            store: store.clone(),
            new_title: "".to_string(),
            tx,
            rx,
        };

        pane.reload(rt);

        pane
    }

    fn handle_message(&mut self) -> Option<Event> {
        match self.rx.try_recv() {
            Ok(msg) => match msg {
                Message::Create(times) => {
                    debug!("found message which create times");
                    self.new_title.clear();
                    return Some(Event::Select(self.store.clone(), times));
                }
                Message::Refresh(map) => {
                    debug!("found message which referesh");
                    self.times = map;
                }
                Message::UpdateLatest(tid, post) => {
                    debug!("found message which update latest");
                    if let Some(tdata) = self.times.get_mut(&tid) {
                        tdata.latest = Some(post);
                    }
                }
                Message::Error(err) => {
                    error!(err);
                }
            },
            Err(_e) => {}
        }

        None
    }
}
