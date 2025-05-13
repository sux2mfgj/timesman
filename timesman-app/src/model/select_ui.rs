use egui::{CentralPanel, Key, ScrollArea, TextEdit, TopBottomPanel};
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
pub enum UIResponse {
    SelectErr(String),
    SelectOk,
}

pub struct SelectUI {
    new: bool,
    new_title: String,
    open: bool,
    open_id: String,
    open_err_msg: Option<String>,
}

// TODO: maybe this function can return the reference of Times in Vec<times>.
fn get_times(title: &String, times: &Vec<Times>) -> Option<Times> {
    let Some(t) = times.iter().find(|t| &t.title == title) else {
        return None;
    };

    Some(t.clone())
}

impl SelectUI {
    pub fn new() -> Self {
        Self {
            new: false,
            new_title: "".to_string(),
            open: false,
            open_id: "".to_string(),
            open_err_msg: None,
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        times: &Vec<Times>,
        resp: &Vec<UIResponse>,
    ) -> Result<Vec<UIRequest>, String> {
        let mut ureq = vec![];

        self.top_bar(ctx, &times, &mut ureq)?;
        self.main_panel(ctx, &times, &mut ureq)?;
        self.consume_keys(ctx, &times, &mut ureq)?;

        if self.new {
            self.show_title_input_window(ctx, &mut ureq);
        }

        if self.open {
            self.show_open_input_window(ctx, &mut ureq);
        }

        self.handle_ui_resp(resp);

        Ok(ureq)
    }

    fn times_entry(
        &self,
        times: &Times,
        row: &mut TableRow,
    ) -> Option<UIRequest> {
        let mut req = None;

        row.col(|ui| {
            ui.label(format!("{}", times.id));
        });

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
                .column(Column::auto().at_least(20f32)) // for #
                .column(Column::auto().at_least(100f32)) // for created_at
                .column(Column::remainder()); // for title

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
        // on window
        if self.new {
            if ui::consume_escape(ctx) {
                self.new = false;
            }

            return Ok(());
        }
        if self.open {
            if ui::consume_escape(ctx) {
                self.open = false;
            }
            return Ok(());
        }

        if ui::consume_escape(ctx) {
            ureq.push(UIRequest::Close);
        }

        if ui::consume_key(ctx, Key::T) {
            ureq.push(self.select_today(times));
        }

        if ui::consume_key(ctx, Key::N) {
            self.new = true;
        }

        if ui::consume_key(ctx, Key::O) {
            self.open = true;
        }

        Ok(())
    }

    fn show_title_input_window(
        &mut self,
        ctx: &egui::Context,
        ureq: &mut Vec<UIRequest>,
    ) {
        egui::Window::new("new").title_bar(false).show(ctx, |ui| {
            ui.label("new title: ");
            let resp = ui.add(TextEdit::singleline(&mut self.new_title));
            resp.request_focus();

            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.new = false;
                ureq.push(UIRequest::CreateTimes(self.new_title.clone()));
                self.new_title.clear();
            }
        });
    }

    fn show_open_input_window(
        &mut self,
        ctx: &egui::Context,
        ureq: &mut Vec<UIRequest>,
    ) {
        egui::Window::new("open").title_bar(false).show(ctx, |ui| {
            ui.label("input the times id to open:");
            let resp = ui.add(TextEdit::singleline(&mut self.open_id));
            resp.request_focus();

            if let Some(err) = &self.open_err_msg {
                ui.label(err);
            }

            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let Ok(tid) = self.open_id.parse() else {
                    self.open_err_msg =
                        Some("Invalid input. please check it".to_string());
                    return;
                };

                ureq.push(UIRequest::SelectTimes(tid));
            }
        });
    }

    fn handle_ui_resp(&mut self, resp: &Vec<UIResponse>) {
        for r in resp {
            match r {
                UIResponse::SelectErr(err) => {
                    self.open_err_msg = Some(err.to_string());
                }
                UIResponse::SelectOk => {
                    self.open = false;
                    self.open_err_msg = None;
                    self.open_id.clear();
                }
            }
        }
    }
}
