use crate::app::Event;
use crate::plugin::Plugin;
use crate::req::{Requester, Times};
use eframe::egui::ScrollArea;
use egui::{Key, Modifiers};

use super::{pane_menu, Pane};

pub struct SelectPane {
    times: Vec<Times>,
    new_title: String,
    req: Requester,
}

impl Pane for SelectPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        plugin: &mut Plugin,
    ) -> Option<Event> {
        let mut event = None;

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui, plugin) {
                        event = Some(e);
                    }
                });
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("new");
                ui.separator();
                ui.text_edit_singleline(&mut self.new_title);
            });
            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter)) {
                if let Some(newt) = self.req.create_times(&self.new_title) {
                    event = Some(Event::Select(self.req.clone(), newt.clone()));
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height());
            scroll_area.show(ui, |ui| {
                for t in &self.times {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", t.created_at));
                        ui.separator();
                        if ui.button(&t.title).clicked() {
                            event = Some(Event::Select(
                                self.req.clone(),
                                t.clone(),
                            ));
                        }
                    });
                }
            });
        });

        event
    }

    fn reload(&mut self) {}
}

impl SelectPane {
    pub fn new(req: Requester) -> Self {
        let list = req.get_list().unwrap();
        Self {
            times: list,
            req,
            new_title: "".to_string(),
        }
    }
}
