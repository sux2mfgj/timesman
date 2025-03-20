use egui::{CentralPanel, ScrollArea};
use timesman_type::{Tid, Times};

pub enum UIRequest {
    SelectTimes(Tid),
}

pub enum UIResponse {}

pub trait SelectPaneTrait {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg: &Vec<UIResponse>,
        times: &Vec<Times>,
    ) -> Result<Vec<UIRequest>, String>;
}

pub struct SelectPane {}

impl SelectPane {
    pub fn new() -> Self {
        Self {}
    }

    fn times_entry(&self, times: &Times, ui: &mut egui::Ui) -> Vec<UIRequest> {
        let mut reqs = vec![];

        ui.horizontal(|ui| {
            ui.label(times.created_at.format("%Y-%m-%d %H:%M").to_string());

            ui.separator();

            if ui.button(times.title.clone()).clicked() {
                reqs.push(UIRequest::SelectTimes(times.id));
            }

            // TODO: show the latest post
        });

        reqs
    }

    fn main_panel(&self, ctx: &egui::Context, times: &Vec<Times>) {
        CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height());

            scroll_area.show(ui, |ui| {
                for t in times {
                    self.times_entry(&t, ui);
                }
            });
        });
    }
}

impl SelectPaneTrait for SelectPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg: &Vec<UIResponse>,
        times: &Vec<Times>,
    ) -> Result<Vec<UIRequest>, String> {
        self.main_panel(ctx, times);

        Ok(vec![])
    }
}
