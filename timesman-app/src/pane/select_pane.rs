use std::sync::Arc;
// use std::sync::Mutex;

use crate::app::Event;

use eframe::egui::ScrollArea;
use egui::{Key, Modifiers};
use store::{Store, Times};
use tokio;
use tokio::sync::Mutex;

use super::{pane_menu, Pane};
use tokio::runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct SelectPane {
    times: Vec<Times>,
    new_title: String,
    store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

enum Message {
    Create(Times),
    Refresh(Vec<Times>),
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
                for t in &self.times {
                    ui.horizontal(|ui| {
                        ui.label(
                            t.created_at.format("%Y-%m-%d %H:%M").to_string(),
                        );

                        ui.separator();
                        if ui.button(&t.title).clicked() {
                            event = Some(Event::Select(
                                self.store.clone(),
                                t.clone(),
                            ));
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
                match tx.send(Message::Refresh(times)).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!(format!("failed to sent message: {}", e));
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
            times: vec![],
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
                Message::Refresh(timeses) => {
                    debug!("found message which referesh");
                    self.times = timeses;
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
