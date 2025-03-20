use timesman_type::Post;

use egui::{CentralPanel, ScrollArea, TextEdit, TopBottomPanel};

use tokio::runtime::Runtime;

#[derive(Copy, Clone)]
pub enum UIRequest {}

#[derive(Copy, Clone)]
pub enum UIResponse {}

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
}

impl TimesPaneTrait for TimesPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg: &Vec<UIResponse>,
        posts: &Vec<Post>,
    ) -> Result<Vec<UIRequest>, String> {
        let mut ui_reqs = vec![];
        self.top_bar(ctx);
        self.bottom(ctx);
        self.main_panel(ctx, posts);

        Ok(ui_reqs)
    }
}

impl TimesPane {
    pub fn new() -> Self {
        Self {
            post_text: String::default(),
        }
    }

    fn top_bar(&self, ctx: &egui::Context) {
        TopBottomPanel::top("bar").show(ctx, |ui| {});
    }

    fn post_entry(&self, post: &Post, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(post.created_at.format("%Y-%m-%d %H:%M").to_string());

            ui.separator();

            if ui.button(post.post.clone()).clicked() {
                todo!();
            }

            // TODO: show the latest post
        });
    }

    fn main_panel(&self, ctx: &egui::Context, posts: &Vec<Post>) {
        CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height());

            scroll_area.show(ui, |ui| {
                for p in posts {
                    self.post_entry(&p, ui);
                }
            });
        });
    }

    fn bottom(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("input").show(ctx, |ui| {
            TextEdit::multiline(&mut self.post_text)
                .hint_text("write here")
                .desired_width(f32::INFINITY)
                .show(ui);
        });
    }
}
