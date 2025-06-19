use std::io::Read;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

use super::ui;
use infer::Infer;
use timesman_type::{File, FileType, Post, Tag, TagId, Tdid, Todo};
use serde_json;

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
    TodoWithDetail(String, String),
    UpdateTodoDetail(Tdid, String),
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
    ClearTextSidePaneDetail,
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
            match &file.ftype {
                FileType::Image(_) => 100f32,
                FileType::Text(text) => {
                    // Calculate height based on preview lines and UI elements
                    let preview_lines = text.lines().take(3).count();
                    40f32 + (preview_lines as f32 * 15f32) // Base height + line height
                }
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
                // Create a more unique URI using hash of image data  
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                
                let mut hasher = DefaultHasher::new();
                image.hash(&mut hasher);
                let hash = hasher.finish();
                
                let uri = format!("bytes://image_{}_{:x}", 
                    file.name.replace(" ", "_").replace("/", "_").replace("\\", "_"), 
                    hash);
                
                let img = egui::Image::from_bytes(uri, image.clone())
                    .max_height(200.0)
                    .max_width(300.0)
                    .rounding(egui::Rounding::same(5.0));
                
                let img_ui = ui.add(img.sense(egui::Sense::click()));

                if img_ui.clicked() {
                    self.preview = Some(file.clone());
                }

                ui.label(format!("File(image): {}", file.name));
            }
            FileType::Text(text) => {
                ui.vertical(|ui| {
                    // Show file info
                    ui.horizontal(|ui| {
                        ui.label(format!("📄 File(text): {}", file.name));
                        ui.label(format!("({} chars)", text.len()));
                    });
                    
                    // Show text preview (first few lines)
                    let preview_lines: Vec<&str> = text.lines().take(3).collect();
                    let preview_text = if preview_lines.len() < text.lines().count() {
                        format!("{}\n...", preview_lines.join("\n"))
                    } else {
                        preview_lines.join("\n")
                    };
                    
                    egui::Frame::none()
                        .fill(ui.style().visuals.faint_bg_color)
                        .inner_margin(egui::Margin::same(8.0))
                        .rounding(egui::Rounding::same(4.0))
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(preview_text)
                                    .wrap()
                                    .sense(egui::Sense::click())
                            );
                        });
                    
                    // Click to open full preview
                    if ui.small_button("🔍 View Full Text").clicked() {
                        self.preview = Some(file.clone());
                    }
                });
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

        if ui::consume_key_with_meta(ctx, Modifiers::COMMAND, Key::T) {
            self.side_panel
                .select_side_panel(Some(side_panel::SidePanelType::TodoDetail));
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
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                let hash = hasher.finish();
                
                let uri = format!("bytes://preview_{}_{:x}", 
                    file.name.replace(" ", "_").replace("/", "_").replace("\\", "_"), 
                    hash);
                    
                let img = egui::Image::from_bytes(uri, data.clone()).shrink_to_fit();
                
                egui::Window::new(&file.name)
                    .title_bar(true)
                    .collapsible(false)
                    .resizable(true)
                    .default_size([600.0, 600.0])
                    .show(ctx, |ui| {
                        ui.add(img);
                    });
            }
            FileType::Text(text) => {
                egui::Window::new(&format!("📄 {}", file.name))
                    .title_bar(true)
                    .collapsible(false)
                    .resizable(true)
                    .default_size([700.0, 500.0])
                    .min_size([400.0, 300.0])
                    .show(ctx, |ui| {
                        // File info header
                        ui.horizontal(|ui| {
                            ui.label(format!("Lines: {}", text.lines().count()));
                            ui.separator();
                            ui.label(format!("Characters: {}", text.len()));
                            ui.separator();
                            ui.label(format!("Bytes: {}", text.as_bytes().len()));
                        });
                        
                        ui.separator();
                        
                        // Text content with better formatting
                        egui::ScrollArea::both()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                // Use monospace font for better text file viewing
                                ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));
                                
                                // Add some padding around the text
                                egui::Frame::none()
                                    .inner_margin(egui::Margin::same(10.0))
                                    .show(ui, |ui| {
                                        // Detect and handle different text formats
                                        if file.name.ends_with(".json") {
                                            // Try to format JSON
                                            match serde_json::from_str::<serde_json::Value>(text) {
                                                Ok(json_value) => {
                                                    let formatted = serde_json::to_string_pretty(&json_value)
                                                        .unwrap_or_else(|_| text.to_string());
                                                    ui.label(formatted);
                                                }
                                                Err(_) => {
                                                    ui.label(text);
                                                }
                                            }
                                        } else if file.name.ends_with(".md") || file.name.ends_with(".markdown") {
                                            // Basic markdown-like formatting
                                            for line in text.lines() {
                                                if line.starts_with("# ") {
                                                    ui.heading(&line[2..]);
                                                } else if line.starts_with("## ") {
                                                    ui.strong(&line[3..]);
                                                } else if line.starts_with("- ") || line.starts_with("* ") {
                                                    ui.label(format!("  • {}", &line[2..]));
                                                } else if line.trim().is_empty() {
                                                    ui.add_space(ui.spacing().item_spacing.y);
                                                } else {
                                                    ui.label(line);
                                                }
                                            }
                                        } else {
                                            // Regular text with line numbers for code files
                                            if file.name.ends_with(".rs") || file.name.ends_with(".py") || 
                                               file.name.ends_with(".js") || file.name.ends_with(".ts") ||
                                               file.name.ends_with(".c") || file.name.ends_with(".cpp") {
                                                for (i, line) in text.lines().enumerate() {
                                                    ui.horizontal(|ui| {
                                                        ui.weak(format!("{:3}: ", i + 1));
                                                        ui.label(line);
                                                    });
                                                }
                                            } else {
                                                ui.label(text);
                                            }
                                        }
                                    });
                            });
                        
                        ui.separator();
                        
                        // Footer with actions
                        ui.horizontal(|ui| {
                            if ui.button("📋 Copy to Clipboard").clicked() {
                                ui.output_mut(|o| o.copied_text = text.clone());
                            }
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("✖ Close").clicked() {
                                    // The preview will be closed by the escape key handler
                                }
                            });
                        });
                    });
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
                UIResponse::ClearTextSidePaneDetail => {
                    self.side_panel.clear_detail_text();
                }
                UIResponse::FileDropped(path) => {
                    self.dropped_file = load_dropped_file(path);
                }
            }
        }
    }
}
