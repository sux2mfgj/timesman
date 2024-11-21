use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::rc::Rc;

use crate::app::Event;
use crate::store::{Post, Store, Times};
use chrono::{DateTime, Local, TimeZone, Utc};
use eframe::egui::ScrollArea;
use egui::{Key, Modifiers, Ui};
use egui_file_dialog::FileDialog;

use super::{pane_menu, Pane};

pub struct TimesPane {
    times: Times,
    posts: Vec<Post>,
    post_text: String,
    file_dialog: FileDialog,
    store: Rc<RefCell<dyn Store>>,
    edit_title: bool,
    edit_post: Option<i64>,
}

impl TimesPane {
    pub fn new(store: Rc<RefCell<dyn Store>>, times: Times) -> Self {
        let store_ref = store.borrow();
        let posts = store_ref.get_posts(times.id).unwrap();

        Self {
            posts,
            times,
            post_text: "".to_string(),
            file_dialog: FileDialog::new(),
            store: store.clone(),
            edit_title: false,
            edit_post: None,
        }
    }

    fn show_times(&mut self, scroll_area: ScrollArea, ui: &mut Ui) {
        scroll_area.show(ui, |ui| {
            for p in &mut self.posts {
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
                                let mut store = self.store.borrow_mut();
                                self.edit_post = None;
                                match store
                                    .update_post(self.times.id, p.clone())
                                {
                                    Ok(_) => {
                                        //TODO: should update the post.updated_at.
                                    }
                                    Err(e) => {
                                        error!(e);
                                    }
                                };
                            }
                        } else {
                            ui.label(&p.post);
                        }
                    } else {
                        ui.label(&p.post).on_hover_ui(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("id: {}", p.id));
                                if ui.button("delete").clicked() {
                                    let mut store = self.store.borrow_mut();
                                    match store.delete_post(self.times.id, p.id)
                                    {
                                        Ok(_) => {}
                                        Err(e) => error!(e),
                                    };
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

    fn save_file(&self, path: &PathBuf) {
        let file = File::create(path).unwrap();

        let mut bw = BufWriter::new(file);

        for post in &self.posts {
            writeln!(
                bw,
                "{} {} {}",
                post.id,
                post.created_at.format("%Y-%m-%d %H:%M").to_string(),
                post.post
            )
            .unwrap();
        }

        bw.flush().unwrap();
    }
}

impl Pane for TimesPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Event> {
        let mut event = None;
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui) {
                        event = Some(e);
                    }
                });
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
                    self.file_dialog.save_file();
                }

                if ui.button("delete").clicked() {
                    let mut store_ref = self.store.borrow_mut();
                    match store_ref.delete_times(self.times.id) {
                        Err(e) => {
                            error!(e)
                        }
                        Ok(()) => {
                            event = Some(Event::Pop);
                        }
                    }
                }

                if self.edit_title {
                    if ui.button("done").clicked() {
                        let mut store = self.store.borrow_mut();
                        match store.update_times(self.times.clone()) {
                            Ok(_) => {}
                            Err(e) => {
                                error!(e);
                            }
                        }
                        self.edit_title = false;
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
            egui::TextEdit::multiline(&mut self.post_text)
                .hint_text("write here")
                .desired_width(f32::INFINITY)
                .show(ui);

            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter)) {
                if self.post_text.is_empty() {
                    return;
                }

                let text = self.post_text.trim_end();

                let mut store_ref = self.store.borrow_mut();
                match store_ref.create_post(self.times.id, text.to_string()) {
                    Err(_e) => {}
                    Ok(p) => {
                        self.posts.push(p);
                        self.post_text.clear();
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height())
                .stick_to_bottom(true);
            self.show_times(scroll_area, ui);
        });

        event
    }

    fn reload(&mut self) {}
}
