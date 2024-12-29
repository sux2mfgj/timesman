use std::path::PathBuf;
use std::sync::Arc;
use url::Url;

use crate::app::Event;

use chrono::{DateTime, Local, TimeZone, Utc};
use eframe::egui::ScrollArea;
use egui::{Key, Modifiers, Ui};
use egui_file_dialog::FileDialog;
use timesman_bstore::json::JsonStore;
use timesman_bstore::{Post, Store, Times};
use tokio::runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use super::Pane;

pub struct TimesPane {
    times: Times,
    posts: Vec<Post>,
    post_text: String,
    file_dialog: FileDialog,
    store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
    edit_title: bool,
    edit_post: Option<i64>,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

enum Message {
    Refresh(Vec<Post>),
    Create(Post),
    UpdateTimes(Times),
    UpdatePost(Post),
    Delete(Post),
    Pop,
}

impl TimesPane {
    pub fn new(
        store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
        times: Times,
        rt: &runtime::Runtime,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<Message>(32);

        {
            let tid = times.id;
            let store2 = store.clone();
            let msg_tx = tx.clone();
            rt.spawn(async move {
                let store = store2.lock().await;
                match store.get_posts(tid).await {
                    Ok(posts) => {
                        msg_tx.send(Message::Refresh(posts)).await.unwrap();
                    }
                    Err(_e) => {}
                }
            });
        }

        //TODO
        // let store_ref = store.borrow();
        // let posts = store_ref.get_posts(times.id).unwrap();

        Self {
            posts: vec![],
            times,
            post_text: "".to_string(),
            file_dialog: FileDialog::new(),
            store: store.clone(),
            edit_title: false,
            edit_post: None,
            tx,
            rx,
        }
    }

    fn is_same_hour<T: chrono::Datelike + chrono::Timelike>(
        a: &T,
        b: &T,
    ) -> bool {
        if a.year() != b.year() {
            return false;
        }

        if a.month() != b.month() {
            return false;
        }

        if a.day() != b.day() {
            return false;
        }

        if a.hour() != b.hour() {
            return false;
        }

        return true;
    }

    fn show_times(
        &mut self,
        rt: &runtime::Runtime,
        scroll_area: ScrollArea,
        ui: &mut Ui,
    ) {
        scroll_area.show(ui, |ui| {
            let mut prev: Option<chrono::NaiveDateTime> = None;

            for p in &mut self.posts {
                if let Some(ptime) = prev {
                    if !Self::is_same_hour(&ptime, &p.created_at) {
                        ui.separator();
                    }
                }
                prev = Some(p.created_at);

                ui.horizontal(|ui| {
                    let utc_time = Utc.from_utc_datetime(&p.created_at);
                    let local_time: DateTime<Local> = DateTime::from(utc_time);
                    ui.label(
                        local_time
                            .naive_local()
                            .format("%Y-%m-%d %H:%M")
                            .to_string(),
                    );
                    ui.separator();

                    if let Some(edit_pid) = self.edit_post {
                        if p.id == edit_pid {
                            ui.text_edit_singleline(&mut p.post);
                            if ui.button("done").clicked() {
                                let store = self.store.clone();
                                let tid = self.times.id;
                                let tx = self.tx.clone();
                                let post = p.clone();

                                rt.spawn(async move {
                                    let mut store = store.lock().await;
                                    match store.update_post(tid, post).await {
                                        Ok(p) => {
                                            tx.send(Message::UpdatePost(p))
                                                .await
                                                .unwrap();
                                        }
                                        Err(e) => {
                                            error!(e);
                                        }
                                    }
                                });
                            }
                        } else {
                            ui.label(&p.post);
                        }
                    } else {
                        let line = match Url::parse(&p.post) {
                            Ok(_url) => ui.hyperlink(&p.post),
                            Err(_) => ui.label(&p.post),
                        };

                        line.on_hover_ui(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("id: {}", p.id));
                                if ui.button("delete").clicked() {
                                    let store = self.store.clone();
                                    let tid = self.times.id;
                                    let pid = p.id;
                                    let tx = self.tx.clone();
                                    let post = p.clone();
                                    rt.spawn(async move {
                                        let mut store = store.lock().await;
                                        match store.delete_post(tid, pid).await
                                        {
                                            Ok(_) => {
                                                tx.send(Message::Delete(post))
                                                    .await
                                                    .unwrap();
                                            }
                                            Err(_e) => {}
                                        }
                                    });
                                }
                                if ui.button("edit").clicked() {
                                    self.edit_post = Some(p.id);
                                }
                            });
                        });
                    }
                });
            }
        });
    }

    fn save_file(&self, path: &PathBuf) -> Result<(), String> {
        let json_store = JsonStore::new(self.times.clone(), self.posts.clone());

        json_store.save_to_file(path)?;

        Ok(())
    }

    fn handle_message(&mut self) -> Option<Event> {
        if self.rx.is_empty() {
            return None;
        }

        match self.rx.try_recv() {
            Ok(msg) => match msg {
                Message::Refresh(posts) => {
                    self.posts = posts;
                }
                Message::Create(post) => {
                    self.posts.push(post);
                    self.post_text.clear();
                }
                Message::UpdateTimes(times) => {
                    self.times = times;
                    self.edit_title = false;
                }
                Message::UpdatePost(post) => {
                    if let Some(p) =
                        self.posts.iter_mut().find(|p| p.id == post.id)
                    {
                        *p = post;
                    } else {
                        error!("found invalid post id: {}", post.id);
                    }
                    self.edit_post = None;
                }
                Message::Delete(post) => {
                    debug!("Handling delete post: {}", post.id);
                    let prev_len = self.posts.len();
                    self.posts.retain(|x| x.id != post.id);
                }
                Message::Pop => {
                    return Some(Event::Pop);
                }
            },
            Err(e) => {
                error!(e);
            }
        }

        None
    }
}

impl Pane for TimesPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        rt: &runtime::Runtime,
    ) -> Option<Event> {
        let mut event = None;

        event = self.handle_message();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                event = self.times_menu(ui);
            });

            ui.horizontal(|ui| {
                ui.label("times");
                ui.separator();
                if self.edit_title {
                    ui.text_edit_singleline(&mut self.times.title);
                } else {
                    ui.label(&self.times.title);
                }
                ui.spacing_mut();
                if ui.button("back").clicked() {
                    event = Some(Event::Pop);
                }

                if ui.button("save").clicked() {
                    //TODO: use the result
                    self.file_dialog.save_file();
                }

                if ui.button("delete").clicked() {
                    let store = self.store.clone();
                    let tid = self.times.id;
                    let tx = self.tx.clone();
                    rt.spawn(async move {
                        let mut store = store.lock().await;
                        match store.delete_times(tid).await {
                            Ok(()) => {
                                tx.send(Message::Pop).await.unwrap();
                            }
                            Err(e) => {
                                error!(e);
                            }
                        }
                    });
                }

                if self.edit_title {
                    if ui.button("done").clicked() {
                        let store = self.store.clone();
                        let times = self.times.clone();
                        let tx = self.tx.clone();

                        rt.spawn(async move {
                            let mut store = store.lock().await;
                            match store.update_times(times).await {
                                Ok(times) => {
                                    tx.send(Message::UpdateTimes(times))
                                        .await
                                        .unwrap();
                                }
                                Err(e) => {
                                    error!(e);
                                }
                            }
                        });
                    }
                } else {
                    if ui.button("edit").clicked() {
                        self.edit_title = true;
                    }
                }

                self.file_dialog.update(ctx);

                if let Some(path) = self.file_dialog.take_selected() {
                    self.save_file(&path);
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            let text = self.post_text.clone();
            egui::TextEdit::multiline(&mut self.post_text)
                .hint_text("write here")
                .desired_width(f32::INFINITY)
                .show(ui);

            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter)) {
                if self.post_text.is_empty() {
                    return;
                }

                let text = text.trim_end().to_string();
                let store = self.store.clone();
                let tid = self.times.id;
                let tx = self.tx.clone();

                rt.spawn(async move {
                    let mut store = store.lock().await;
                    match store.create_post(tid, text).await {
                        Ok(post) => {
                            tx.send(Message::Create(post)).await.unwrap();
                        }
                        Err(e) => {
                            error!(e);
                        }
                    }
                });

                self.show_latest_log(ui);
            }

            egui::TopBottomPanel::bottom("bottom log").show(ctx, |ui| {
                ui.label("bottom of bottom");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height())
                .stick_to_bottom(true);
            self.show_times(rt, scroll_area, ui);
        });

        event
    }

    fn reload(&mut self, _rt: &runtime::Runtime) {}
}
