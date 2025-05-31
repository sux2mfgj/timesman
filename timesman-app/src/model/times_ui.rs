use std::io::Read;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

use super::ui;
use infer::Infer;
use timesman_type::{File, FileType, Post, Tag, TagId, Tdid, Todo};

use chrono::{DateTime, Local, Timelike};
use dirs;
use egui::{
    Align, CentralPanel, Key, Layout, Modifiers, TextEdit, TopBottomPanel,
};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use egui_file_dialog::FileDialog;
use linkify::LinkFinder;

mod side_panel;
use side_panel::SidePanel;

#[derive(Debug)]
pub enum UIRequest {
    Post(String, Option<File>),
    UpdatePost(Post),
    Dump(PathBuf),
    Sort(bool),
    Todo(String),
    Tag(String),
    TodoDone(Tdid, bool),
    Close,
}

fn load_dropped_file(file_path: PathBuf) -> Option<File> {
    let name = file_path.file_name().unwrap().to_string_lossy().to_string();

    let mut data = vec![];
    let mut file = fs::File::open(file_path).unwrap();
    file.read_to_end(&mut data).unwrap();

    let infer = Infer::new();

    let ftype = if let Some(ftype) = infer.get(&data) {
        println!("file type: {:?}", ftype.matcher_type());
        match ftype.matcher_type() {
            infer::MatcherType::Image => FileType::Image(data),
            infer::MatcherType::Text => {
                FileType::Text(String::from_utf8(data).unwrap())
            }
            _ => FileType::Other(data),
        }
    } else {
        println!("failed the infer.get");
        FileType::Other(data)
    };

    Some(File { name, ftype })
}

#[derive(Debug)]
pub enum UIResponse {
    ClearText,
    ClearTextSidePane,
    FileDropped(PathBuf),
}

#[derive(PartialEq)]
enum UIState {
    Normal,
    TagAssign,
}

pub struct TimesUI {
    title: String,
    post_text: String,
    dropped_file: Option<File>,
    preview: Option<File>,
    file_dialog: FileDialog,
    side_panel: SidePanel,
    state: UIState,
}

fn show_text(text: &str, ui: &mut egui::Ui) {
    let finder = LinkFinder::new();
    let spans: Vec<_> = finder.spans(text).collect();

    for span in spans {
        if let Some(_) = span.kind() {
            ui.hyperlink(span.as_str());
        } else {
            let text = span.as_str().trim_end();
            if !text.is_empty() {
                ui.label(text);
            }
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
            side_panel: SidePanel::new(),
            state: UIState::Normal,
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        posts: &Vec<Post>,
        todos: &Vec<Todo>,
        tags: &HashMap<TagId, Tag>,
        ures: Vec<UIResponse>,
    ) -> Vec<UIRequest> {
        let mut ureq = vec![];

        self.handle_ui_resp(ures);

        self.top_bar(ctx);
        self.bottom(ctx);
        self.main_panel_table(ctx, posts, tags, &mut ureq);
        self.right_side_panel(ctx, todos, tags, &mut ureq);

        self.consume_keys(ctx, &mut ureq);

        self.handle_file_dialog(ctx, &mut ureq);

        self.show_preview(ctx);

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

                if let Some(dfile) = self.dropped_file.clone() {
                    ui.horizontal(|ui| {
                        ui.label(format!("File: {}", dfile.name));
                        let resp = ui.button("clear");
                        if resp.clicked() {
                            self.dropped_file = None;
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
        let local_created_at: DateTime<Local> =
            DateTime::from(p.created_at.and_utc());

        let posted_at = p.created_at.hour();

        if last_posted != &posted_at {
            *last_posted = posted_at;
            body.row(20f32, |mut row| {
                row.col(|_| {});
                row.col(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                        ui.label(local_created_at.format("%H:00").to_string());
                    });
                });
                row.col(|_| {});
            });
        }
    }

    fn insert_post_row(
        &mut self,
        p: &Post,
        tags: &HashMap<TagId, Tag>,
        ureq: &mut Vec<UIRequest>,
        body: &mut TableBody,
    ) {
        let hight = if let Some(file) = &p.file {
            match file.ftype {
                FileType::Image(_) => 100f32,
                _ => 20f32,
            }
        } else {
            20f32
        };

        body.row(hight, |mut row| {
            self.post_row(&mut row, &p, tags, ureq);
        })
    }

    fn main_panel_table(
        &mut self,
        ctx: &egui::Context,
        posts: &Vec<Post>,
        tags: &HashMap<TagId, Tag>,
        ureq: &mut Vec<UIRequest>,
    ) {
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
                .column(Column::auto()) // tag
                .column(Column::remainder()); // for post

            let mut last_posted = if posts.is_empty() {
                return;
            } else {
                posts[0].created_at.hour()
            };

            builder.body(|mut body| {
                for p in posts {
                    self.insert_separater_row(&mut last_posted, p, &mut body);
                    self.insert_post_row(p, tags, ureq, &mut body);
                }
            });
        });
    }

    fn right_side_panel(
        &mut self,
        ctx: &egui::Context,
        todo: &Vec<Todo>,
        tag: &HashMap<TagId, Tag>,
        ureq: &mut Vec<UIRequest>,
    ) {
        self.side_panel.update(ctx, todo, tag, ureq);
    }

    fn show_file_row(
        &mut self,
        file: &File,
        ui: &mut egui::Ui,
        ureq: &mut Vec<UIRequest>,
    ) {
        match &file.ftype {
            FileType::Image(image) => {
                let img = egui::Image::from_bytes(
                    format!("bytes://{}", file.name),
                    image.clone(),
                );
                let img_ui =
                    ui.add(img.max_height(200.0).sense(egui::Sense::click()));

                if img_ui.clicked() {
                    self.preview = Some(file.clone());
                }

                ui.label(format!("File(image) {}", file.name));
            }
            FileType::Text(_) => {
                let resp = ui.label(format!("File(text): {}", file.name));

                if resp.clicked() {
                    self.preview = Some(file.clone());
                }
            }
            FileType::Other(_) => {
                ui.label(format!("File: {}", file.name));
            }
        }
    }

    fn post_row(
        &mut self,
        row: &mut TableRow,
        post: &Post,
        tags: &HashMap<TagId, Tag>,
        ureq: &mut Vec<UIRequest>,
    ) {
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

        // tag
        row.col(|ui| {
            ui.horizontal(|ui| {
                if self.state == UIState::TagAssign {
                    if ui.button("x").clicked() {
                        if let Some(tag) = &self.side_panel.selected_tag {
                            let mut npost = post.clone();
                            npost.tag = Some(tag.id);

                            ureq.push(UIRequest::UpdatePost(npost));
                        }
                    };
                }

                if let Some(tagid) = post.tag {
                    if let Some(tag) = tags.get(&tagid) {
                        ui.label(&tag.name);
                    } else {
                        ui.label(format!("Error"));
                    }
                }
            });
        });

        row.col(|ui| {
            if !post.post.is_empty() {
                show_text(&post.post, ui);
            }

            if let Some(file) = &post.file {
                self.show_file_row(file, ui, ureq);
            }
        });
    }

    fn post(&mut self, ureqs: &mut Vec<UIRequest>) {
        if self.post_text.is_empty() && self.dropped_file.is_none() {
            return;
        }

        let txt = self.post_text.clone();

        ureqs.push(UIRequest::Post(txt, self.dropped_file.clone()));
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

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::O) {
            self.side_panel
                .select_side_panel(Some(side_panel::SidePanelType::Todo));
        }

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::A) {
            self.side_panel
                .select_side_panel(Some(side_panel::SidePanelType::Tag));
        }

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::S) {
            self.side_panel
                .select_side_panel(Some(side_panel::SidePanelType::TagAssigne));
            self.state = UIState::TagAssign;
        }

        if ui::consume_escape(ctx) {
            if self.preview.is_some() {
                self.preview = None;
            }
            if self.state == UIState::TagAssign {
                self.state = UIState::Normal;
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

    fn show_preview(&self, ctx: &egui::Context) {
        let Some(file) = &self.preview else {
            return;
        };

        match &file.ftype {
            FileType::Image(data) => {
                let img = egui::Image::from_bytes(
                    format!("bytes://{}", &file.name),
                    data.clone(),
                );
                egui::Window::new(&file.name)
                    .title_bar(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.add(img);
                    });
            }
            FileType::Text(text) => {
                //egui::Window::new(&file.name)
                //    .title_bar(false)
                //    .collapsible(false)
                //    .show(ctx, |ui| {});
            }
            FileType::Other(_) => {
                return;
            }
        }
    }

    fn handle_ui_resp(&mut self, resps: Vec<UIResponse>) {
        for r in resps {
            match r {
                UIResponse::ClearText => {
                    self.post_text.clear();
                    self.dropped_file = None;
                }
                UIResponse::ClearTextSidePane => {
                    self.side_panel.clear_text();
                }
                UIResponse::FileDropped(path) => {
                    self.dropped_file = load_dropped_file(path);
                }
            }
        }
    }
}
