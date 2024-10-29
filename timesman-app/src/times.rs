use crate::app::{Event, Pane};
use crate::req::{Post, Requester, Times};
use eframe::egui::ScrollArea;

pub struct TimesPane {
    times: Times,
    posts: Option<Vec<Post>>,
    post_text: String,
}

impl TimesPane {
    pub fn new(times: Times, req: &Requester) -> Self {
        Self {
            posts: req.get_posts(times.id),
            times,
            post_text: "".to_string(),
        }
    }
}

impl Pane for TimesPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Event {
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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height())
                .stick_to_bottom(true);
            scroll_area.show(ui, |ui| {
                if let Some(ps) = &self.posts {
                    for p in ps {
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
                }
            });
        });

        Event::Nothing
    }
}
