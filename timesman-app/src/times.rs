use crate::app::{Event, Pane};
use crate::log::Logger;
use crate::req::{Post, Requester, Times};
use eframe::egui::ScrollArea;
use egui::{Key, Modifiers, Ui};

pub struct TimesPane {
    times: Times,
    posts: Vec<Post>,
    post_text: String,
}

impl TimesPane {
    pub fn new(times: Times, req: &Requester) -> Self {
        Self {
            posts: req.get_posts(times.id).unwrap(),
            times,
            post_text: "".to_string(),
        }
    }

    fn show_times(&self, scroll_area: ScrollArea, ui: &mut Ui) {
        scroll_area.show(ui, |ui| {
            for p in &self.posts {
                ui.horizontal(|ui| {
                    ui.label(p.created_at.format("%Y-%m-%d %H:%M").to_string());
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
            ui.horizontal(|ui| {
                ui.label("times");
                ui.separator();
                ui.label(&self.times.title);
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
                    Err(e) => {
                        Logger::error(e);
                    }
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
