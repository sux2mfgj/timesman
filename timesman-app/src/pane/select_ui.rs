use chrono::Local;
use egui::{CentralPanel, ScrollArea, TopBottomPanel};
use timesman_type::{Tid, Times};

use super::ui;

#[derive(Clone)]
pub enum UIRequest {
    SelectTimes(Tid),
    CreateTimes(String),
    Close,
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

    fn is_duplicated(
        &self,
        title: &String,
        times: &Vec<Times>,
    ) -> Option<Times> {
        let Some(t) = times.iter().find(|&x| &x.title == title) else {
            return None;
        };

        Some(t.clone())
    }

    fn top_bar(
        &self,
        ctx: &egui::Context,
        times: &Vec<Times>,
    ) -> Vec<UIRequest> {
        let mut reqs = vec![];

        TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("today").clicked() {
                    let title = Local::now().format("%Y%m%d").to_string();
                    if let Some(t) = self.is_duplicated(&title, times) {
                        reqs.push(UIRequest::SelectTimes(t.id));
                    } else {
                        reqs.push(UIRequest::CreateTimes(title));
                    }
                }
            });
        });

        reqs
    }

    fn times_entry(
        &self,
        times: &Times,
        ui: &mut egui::Ui,
    ) -> Option<UIRequest> {
        let mut req = None;
        ui.horizontal(|ui| {
            ui.label(times.created_at.format("%Y-%m-%d %H:%M").to_string());

            ui.separator();

            if ui.button(times.title.clone()).clicked() {
                req = Some(UIRequest::SelectTimes(times.id));
            }

            // TODO: show the latest post
        });

        req
    }

    fn main_panel(
        &self,
        ctx: &egui::Context,
        times: &Vec<Times>,
    ) -> Vec<UIRequest> {
        let mut reqs = vec![];

        CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height());

            scroll_area.show(ui, |ui| {
                for t in times {
                    let r = self.times_entry(&t, ui);
                    if let Some(r) = r {
                        reqs.push(r);
                    }
                }
            });
        });

        reqs
    }

    fn consume_keys(&self, ctx: &egui::Context) -> Vec<UIRequest> {
        let mut reqs = vec![];

        if ui::consume_escape(ctx) {
            reqs.push(UIRequest::Close);
        }

        reqs
    }
}

impl SelectPaneTrait for SelectPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        msg: &Vec<UIResponse>,
        times: &Vec<Times>,
    ) -> Result<Vec<UIRequest>, String> {
        let mut ureqs = vec![];

        let r = self.top_bar(ctx, times);
        ureqs = [ureqs, r].concat();

        let r = self.main_panel(ctx, times);
        ureqs = vec![ureqs, r].concat();

        let r = self.consume_keys(ctx);
        ureqs = vec![ureqs, r].concat();

        Ok(ureqs)
    }
}
