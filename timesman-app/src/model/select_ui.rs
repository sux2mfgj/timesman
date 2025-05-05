use egui::{CentralPanel, Key, ScrollArea, TopBottomPanel};
use timesman_type::{Tid, Times};

use super::ui;
use chrono::Local;

pub enum UIRequest {
    SelectTimes(Tid),
    CreateTimes(String),
}
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
        // self.main_panel(ctx, &times, &mut ureq)?;
        // self.consume_keys(ctx, &times, &mut ureq)?;

        Ok(ureq)
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
}
