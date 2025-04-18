use super::ui;
use std::fs;
use std::io::Read;
use std::{fs::File, path::PathBuf};

use egui_extras::{Column, TableBuilder, TableRow};
use timesman_type::{self, Post};

use egui::{CentralPanel, Key, Modifiers, TextEdit, TopBottomPanel};

#[derive(Clone)]
pub enum UIRequest {
    Post(String, Option<(String, timesman_type::File)>),
    Close,
}

pub enum UIResponse {
    PostSuccess,
    FileDropped(PathBuf),
}

pub trait TimesPaneTrait {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg: &Vec<UIResponse>,
        posts: &Vec<Post>,
    ) -> Result<Vec<UIRequest>, String>;
}

pub struct TimesPane {
    post_text: String,
    dropped_file: Option<PathBuf>,
    preview: Option<(String, Vec<u8>)>,
}

impl TimesPaneTrait for TimesPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        ui_resps: &Vec<UIResponse>,
        posts: &Vec<Post>,
    ) -> Result<Vec<UIRequest>, String> {
        let mut ui_reqs = vec![];

        for resp in ui_resps {
            self.handle_ui_resp(resp);
        }

        self.top_bar(ctx);
        self.bottom(ctx);
        self.main_panel_table(ctx, posts);

        if let Some((name, preview_img)) = &self.preview {
            let img = egui::Image::from_bytes(
                format!("bytes://{}", name),
                preview_img.clone(),
            );

            egui::Window::new(format!("{}", name))
                .title_bar(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.add(img);
                });
        }

        let r = self.consume_keys(ctx);
        ui_reqs = vec![ui_reqs, r].concat();

        Ok(ui_reqs)
    }
}

impl TimesPane {
    pub fn new() -> Self {
        Self {
            post_text: String::default(),
            dropped_file: None,
            preview: None,
        }
    }

    fn top_bar(&self, ctx: &egui::Context) {
        TopBottomPanel::top("bar").show(ctx, |_ui| {});
    }

    fn post_row(&mut self, row: &mut TableRow, post: &Post) {
        row.col(|ui| {
            ui.label(post.created_at.format("%Y-%m-%d %H:%M").to_string());
        });
        row.col(|ui| {
            ui.label(post.post.clone());

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
                .column(Column::auto().at_least(100f32))
                .column(Column::remainder());

            builder.body(|mut body| {
                for p in posts {
                    // TODO: function
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
            });
        });
    }

    fn bottom(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("input").show(ctx, |ui| {
            ui.vertical(|ui| {
                TextEdit::multiline(&mut self.post_text)
                    .hint_text("write here")
                    .desired_width(f32::INFINITY)
                    .show(ui);

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

    fn consume_keys(&mut self, ctx: &egui::Context) -> Vec<UIRequest> {
        let mut ui_reqs = vec![];

        let cmd_enter =
            ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter));
        if cmd_enter
            && (!self.post_text.is_empty() || self.dropped_file.is_some())
        {
            let txt = self.post_text.clone();
            let file = if let Some(file) = self.dropped_file.clone() {
                let name =
                    file.file_name().unwrap().to_string_lossy().to_string();

                let ext =
                    file.extension().unwrap().to_string_lossy().to_string();

                let mut data = vec![];
                let mut file = File::open(file).unwrap();
                file.read_to_end(&mut data).unwrap();

                let f = match &*ext {
                    "png" | "jpg" | "jpeg" => timesman_type::File::Image(data),
                    "txt" => timesman_type::File::Text(
                        String::from_utf8(data).unwrap(),
                    ),
                    _ => timesman_type::File::Other(data),
                };

                Some((name, f))
            } else {
                None
            };

            ui_reqs.push(UIRequest::Post(txt, file));
        }

        if ui::consume_escape(ctx) {
            if self.preview.is_some() {
                self.preview = None;
            } else {
                ui_reqs.push(UIRequest::Close);
            }
        }

        ui_reqs
    }

    fn handle_ui_resp(&mut self, resp: &UIResponse) {
        match resp {
            UIResponse::PostSuccess => {
                self.post_text.clear();
                self.dropped_file = None;
            }
            UIResponse::FileDropped(path) => {
                self.dropped_file = Some(path.clone());
            }
        }
    }
}
