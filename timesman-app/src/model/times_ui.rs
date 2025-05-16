use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use super::ui;
use timesman_type::Post;

use chrono::{DateTime, Local, Timelike};
use dirs;
use egui::{
    Align, CentralPanel, Key, Layout, Modifiers, TextEdit, TopBottomPanel,
};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use egui_file_dialog::FileDialog;
use linkify::LinkFinder;

#[derive(Debug)]
pub enum UIRequest {
    Post(String, Option<(String, timesman_type::File)>),
    Dump(PathBuf),
    Sort(bool),
    Close,
}

#[derive(Debug)]
pub enum UIResponse {
    ClearText,
    FileDropped(PathBuf),
}

pub struct TimesUI {
    title: String,
    post_text: String,
    dropped_file: Option<PathBuf>,
    preview: Option<(String, Vec<u8>)>,
    file_dialog: FileDialog,
}

fn show_text(text: &str, ui: &mut egui::Ui) {
    let finder = LinkFinder::new();
    let spans: Vec<_> = finder.spans(text).collect();

    for span in spans {
        if let Some(_) = span.kind() {
            ui.hyperlink(span.as_str());
        } else {
            ui.label(span.as_str().trim_end());
        }
    }
}

impl TimesUI {
    pub fn new(title: String) -> Self {
        Self {
            title: title.clone(),
            post_text: String::from(""),
            dropped_file: None,
            preview: None,
            file_dialog: FileDialog::new().default_file_name(
                dirs::download_dir()
                    .unwrap()
                    .join(format!("{title}.json"))
                    .to_str()
                    .unwrap(),
            ),
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        posts: &Vec<Post>,
        ures: Vec<UIResponse>,
    ) -> Vec<UIRequest> {
        let mut ureq = vec![];

        self.handle_ui_resp(ures);

        self.top_bar(ctx);
        self.bottom(ctx);
        self.main_panel_table(ctx, posts);

        self.consume_keys(ctx, &mut ureq);

        self.handle_file_dialog(ctx, &mut ureq);

        ureq
    }

    fn top_bar(&self, ctx: &egui::Context) {
        TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.label(&self.title);
        });
    }

    fn bottom(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("input").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(
                    TextEdit::multiline(&mut self.post_text)
                        .hint_text("write here")
                        .desired_width(f32::INFINITY),
                );

                if let Some(path) = self.dropped_file.clone() {
                    ui.horizontal(|ui| {
                        if let Some(name) = path.file_name() {
                            ui.label(format!("{}", name.to_str().unwrap()));

                            if ui.button("clear").clicked() {
                                self.dropped_file = None;
                            };
                        }
                    });
                }
            });
        });
    }

    fn insert_separater_row(
        &mut self,
        last_posted: &mut u32,
        p: &Post,
        body: &mut TableBody,
    ) {
        let posted_at = p.created_at.hour();
        if last_posted != &posted_at {
            *last_posted = posted_at;
            body.row(20f32, |mut row| {
                row.col(|_| {});
                row.col(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                        ui.label(p.created_at.format("%H:00").to_string());
                    });
                });
                row.col(|_| {});
            });
        }
    }

    fn insert_post_row(&mut self, p: &Post, body: &mut TableBody) {
        let hight = if let Some(a) = &p.file {
            match a.1 {
                timesman_type::File::Image(_) => 100f32,
                _ => 20f32,
            }
        } else {
            20f32
        };

        body.row(hight, |mut row| {
            self.post_row(&mut row, &p);
        })
    }

    fn main_panel_table(&mut self, ctx: &egui::Context, posts: &Vec<Post>) {
        CentralPanel::default().show(ctx, |ui| {
            let height_available = ui.available_height();
            let builder = TableBuilder::new(ui)
                .striped(true)
                .resizable(false)
                .stick_to_bottom(true)
                .auto_shrink(false)
                .max_scroll_height(height_available)
                .resizable(true)
                .column(Column::auto()) // for #
                .column(Column::auto().at_least(100f32)) // for created_at
                .column(Column::remainder()); // for post

            let mut last_posted = if posts.is_empty() {
                return;
            } else {
                posts[0].created_at.hour()
            };

            builder.body(|mut body| {
                for p in posts {
                    self.insert_separater_row(&mut last_posted, p, &mut body);
                    self.insert_post_row(p, &mut body);
                }
            });
        });
    }

    fn post_row(&mut self, row: &mut TableRow, post: &Post) {
        row.col(|ui| {
            ui.label(format!("{}", post.id));
        });

        row.col(|ui| {
            let localtime: DateTime<Local> =
                DateTime::from(post.created_at.and_utc());
            ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                ui.label(localtime.format("%Y-%m-%d %H:%M").to_string());
            });
        });
        row.col(|ui| {
            if !post.post.is_empty() {
                show_text(&post.post, ui);
            }

            if let Some(file) = &post.file {
                match &file.1 {
                    timesman_type::File::Image(data) => {
                        let img = egui::Image::from_bytes(
                            format!("bytes://{}", file.0),
                            data.clone(),
                        );
                        let img_ui = ui.add(
                            img.max_height(200.0).sense(egui::Sense::click()),
                        );

                        if img_ui.clicked() {
                            self.preview = Some((file.0.clone(), data.clone()));
                        }
                    }
                    timesman_type::File::Text(_txt) => {}
                    timesman_type::File::Other(_data) => {}
                }
            }
        });
    }

    fn post(&mut self, ureqs: &mut Vec<UIRequest>) {
        if self.post_text.is_empty() {
            return;
        }

        let txt = self.post_text.clone();
        let file = if let Some(file) = self.dropped_file.clone() {
            let name = file.file_name().unwrap().to_string_lossy().to_string();

            let ext = file.extension().unwrap().to_string_lossy().to_string();

            let mut data = vec![];
            let mut file = File::open(file).unwrap();
            file.read_to_end(&mut data).unwrap();

            let f = match &*ext {
                "png" | "jpg" | "jpeg" => timesman_type::File::Image(data),
                "txt" => {
                    timesman_type::File::Text(String::from_utf8(data).unwrap())
                }
                _ => timesman_type::File::Other(data),
            };

            Some((name, f))
        } else {
            None
        };

        ureqs.push(UIRequest::Post(txt, file));
    }

    fn consume_keys(
        &mut self,
        ctx: &egui::Context,
        ureqs: &mut Vec<UIRequest>,
    ) {
        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::Enter) {
            self.post(ureqs);
        }

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::D) {
            self.file_dialog.save_file();
        }


        if ui::consume_key_with_meta(ctx, Modifiers::SHIFT, Key::S) {
            ureqs.push(UIRequest::Sort(true));
        }

        if ui::consume_key(ctx, Key::S) {
            ureqs.push(UIRequest::Sort(false));
        }

        if ui::consume_escape(ctx) {
            if self.preview.is_some() {
                self.preview = None;
            } else {
                ureqs.push(UIRequest::Close);
            }
        }
    }

    fn handle_file_dialog(
        &mut self,
        ctx: &egui::Context,
        ureqs: &mut Vec<UIRequest>,
    ) {
        self.file_dialog.update(ctx);
        if let Some(dump_file) = self.file_dialog.take_selected() {
            ureqs.push(UIRequest::Dump(dump_file.to_path_buf()));
        }
    }

    fn handle_ui_resp(&mut self, resps: Vec<UIResponse>) {
        for r in resps {
            match r {
                UIResponse::ClearText => {
                    self.post_text.clear();
                    self.dropped_file = None;
                }
                UIResponse::FileDropped(path) => {
                    self.dropped_file = Some(path.clone());
                }
            }
        }
    }
}
