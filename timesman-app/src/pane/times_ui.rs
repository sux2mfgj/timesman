use timesman_type::Post;

use egui::{
    CentralPanel, Key, Modifiers, ScrollArea, TextEdit, TopBottomPanel,
};

#[derive(Clone)]
pub enum UIRequest {
    Post(String),
}

pub enum UIResponse {
    PostSuccess,
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
        self.main_panel(ctx, posts);

        let r = self.consume_keys(ctx);
        ui_reqs = vec![ui_reqs, r].concat();

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

            ui.label(post.post.clone());
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

    fn consume_keys(&self, ctx: &egui::Context) -> Vec<UIRequest> {
        let mut ui_reqs = vec![];

        let cmd_enter =
            ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter));
        if cmd_enter {
            ui_reqs.push(UIRequest::Post(self.post_text.clone()));
        }

        ui_reqs
    }

    fn handle_ui_resp(&mut self, resp: &UIResponse) {
        match resp {
            UIResponse::PostSuccess => {
                self.post_text.clear();
            }
        }
    }
}
