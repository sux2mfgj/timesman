use crate::app::{Event, Pane};
use crate::pane::pane_menu;
use crate::req::{Requester, Times};
use eframe::egui::ScrollArea;
use egui::{Key, Modifiers};

pub struct StartPane {
    times: Vec<Times>,
    title: String,
}

impl Pane for StartPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Event {
        let mut event = Event::Nothing;

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui) {
                        event = e;
                    }
                });
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("new");
                ui.separator();
                ui.text_edit_singleline(&mut self.title);
            });
            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter)) {
                if let Some(newt) = req.create_times(&self.title) {
                    event = Event::OpenTimes(newt);
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
                        if ui.button(&t.title).clicked() {
                            event = Event::OpenTimes(t.clone());
                        }
                        ui.label(format!("{}", t.created_at));
                        if ui.button("delete").clicked() {
                            match req.delete_times(t.id) {
                                Err(e) => {
                                    error!(e)
                                }
                                Ok(()) => {}
                            }
                        }
                    });
                }
            });
        });

        event
    }
}

impl StartPane {
    pub fn new(req: &Requester) -> Self {
        let list = req.get_list().unwrap();
        Self {
            times: list,
            title: "".to_string(),
        }
    }
}
