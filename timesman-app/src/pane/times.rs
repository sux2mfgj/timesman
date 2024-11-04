use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::app::{Event, Pane};
use crate::req::{Post, Requester, Times};
use chrono::{DateTime, Local, TimeZone, Utc};
use eframe::egui::ScrollArea;
use egui::{Key, Modifiers, Ui};
use egui_file_dialog::FileDialog;

use super::pane_menu;

pub struct TimesPane {
    times: Times,
    posts: Vec<Post>,
    post_text: String,
    file_dialog: FileDialog,
}

impl TimesPane {
    pub fn new(times: Times, req: &Requester) -> Self {
        Self {
            posts: req.get_posts(times.id).unwrap(),
            times,
            post_text: "".to_string(),
            file_dialog: FileDialog::new(),
        }
    }

    fn show_times(&self, scroll_area: ScrollArea, ui: &mut Ui) {
        scroll_area.show(ui, |ui| {
            for p in &self.posts {
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
                    ui.label(&p.post).on_hover_ui(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("id: {}", p.id));
                            if ui.button("delete").clicked() {
                                println!("TODO: do delete the post!");
                            }
                        });
                    });
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
        req: &Requester,
    ) -> Event {
        let mut event = Event::Nothing;
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui) {
                        event = e;
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("times");
                ui.separator();
                ui.label(&self.times.title);
                ui.spacing_mut();
                if ui.button("back").clicked() {
                    event = Event::ToStart;
                }

                if ui.button("save").clicked() {
                    self.file_dialog.save_file();
                }

                if ui.button("delete").clicked() {
                    match req.delete_times(self.times.id) {
                        Err(e) => {
                            error!(e)
                        }
                        Ok(()) => {
                            event = Event::ToStart;
                        }
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

                match req.post_post(self.times.id, &self.post_text) {
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
}
