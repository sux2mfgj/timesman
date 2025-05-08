use egui::{CentralPanel, Key, ScrollArea, TopBottomPanel};
use timesman_type::{Tid, Times};

use super::ui;
use chrono::{DateTime, Local};

#[derive(Debug)]
pub enum UIRequest {
    SelectTimes(Tid),
    CreateTimes(String),
    Close,
}
#[derive(Debug)]
pub enum UIResponse {}

pub struct SelectUI {}

// TODO: maybe this function can return the reference of Times in Vec<times>.
fn get_times(title: &String, times: &Vec<Times>) -> Option<Times> {
    let Some(t) = times.iter().find(|t| &t.title == title) else {
        return None;
    };

    Some(t.clone())
}

impl SelectUI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        times: &Vec<Times>,
    ) -> Result<Vec<UIRequest>, String> {
        let mut ureq = vec![];

        self.top_bar(ctx, &times, &mut ureq)?;
        self.main_panel(ctx, &times, &mut ureq)?;
        self.consume_keys(ctx, &times, &mut ureq)?;

        Ok(ureq)
    }

    fn times_entry(
        &self,
        times: &Times,
        ui: &mut egui::Ui,
    ) -> Option<UIRequest> {
        let mut req = None;
        ui.horizontal(|ui| {
            let created_at: DateTime<Local> =
                DateTime::from(times.created_at.and_utc());
            ui.label(created_at.format("%Y-%m-%d %H:%M").to_string());

            // ui.separator();
            // ui.label(format!("{:3}", nposts));
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
        ureq: &mut Vec<UIRequest>,
    ) -> Result<(), String> {
        CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height());

            scroll_area.show(ui, |ui| {
                for t in times {
                    let r = self.times_entry(&t, ui);
                    if let Some(r) = r {
                        ureq.push(r);
                    }
                }
            });
        });

        Ok(())
    }

    fn top_bar(
        &mut self,
        ctx: &egui::Context,
        times: &Vec<Times>,
        ureq: &mut Vec<UIRequest>,
    ) -> Result<(), String> {
        TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("today").clicked() {
                    ureq.push(self.select_today(times));
                }
            });
        });

        Ok(())
    }

    fn select_today(&mut self, times: &Vec<Times>) -> UIRequest {
        let title = Local::now().format("%Y%m%d").to_string();

        if let Some(t) = get_times(&title, times) {
            UIRequest::SelectTimes(t.id)
        } else {
            UIRequest::CreateTimes(title)
        }
    }

    fn consume_keys(
        &mut self,
        ctx: &egui::Context,
        times: &Vec<Times>,
        ureq: &mut Vec<UIRequest>,
    ) -> Result<(), String> {
        if ui::consume_escape(ctx) {
            ureq.push(UIRequest::Close);
        }

        if ui::consume_key(ctx, Key::T) {
            ureq.push(self.select_today(times));
        }

        Ok(())
    }
}
