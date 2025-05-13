use egui::{CentralPanel, Key, ScrollArea, TopBottomPanel};
use timesman_type::{Tid, Times};

use super::ui;
use chrono::{DateTime, Local};

use egui_extras::{Column, TableBuilder, TableRow};

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
        row: &mut TableRow,
    ) -> Option<UIRequest> {
        let mut req = None;

        row.col(|ui| {
            let created_at: DateTime<Local> =
                DateTime::from(times.created_at.and_utc());
            ui.label(created_at.format("%Y-%m-%d %H:%M").to_string());
        });
        row.col(|ui| {
            if ui.button(times.title.clone()).clicked() {
                req = Some(UIRequest::SelectTimes(times.id));
            }
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
            let height_available = ui.available_height();
            let builder = TableBuilder::new(ui)
                .striped(true)
                .resizable(false)
                .stick_to_bottom(true)
                .auto_shrink(false)
                .max_scroll_height(height_available)
                .resizable(true)
                .column(Column::auto().at_least(100f32))
                .column(Column::remainder());

            builder.body(|mut body| {
                for t in times {
                    body.row(20f32, |mut row| {
                        let r = self.times_entry(&t, &mut row);
                        if let Some(r) = r {
                            ureq.push(r);
                        }
                    });
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
