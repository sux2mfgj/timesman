use std::cell::RefCell;
use std::rc::Rc;

use crate::app::Event;

use eframe::egui::ScrollArea;
use egui::{Key, Modifiers};
use store::{Store, Times};

use super::{pane_menu, Pane};

pub struct SelectPane {
    times: Vec<Times>,
    new_title: String,
    store: Rc<RefCell<dyn Store>>,
}

impl Pane for SelectPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Event> {
        let mut event = None;

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Times", |ui| {
                    if let Some(e) = pane_menu(ui) {
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
                let mut store_ref = self.store.borrow_mut();
                match store_ref.create_times(self.new_title.clone()) {
                    Ok(new_times) => {
                        event = Some(Event::Select(
                            self.store.clone(),
                            new_times.clone(),
                        ));
                        self.new_title.clear();
                    }
                    Err(e) => {
                        error!(e);
                    }
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
                        ui.label(
                            t.created_at.format("%Y-%m-%d %H:%M").to_string(),
                        );

                        ui.separator();
                        if ui.button(&t.title).clicked() {
                            event = Some(Event::Select(
                                self.store.clone(),
                                t.clone(),
                            ));
                        }
                    });
                }
            });
        });

        event
    }

    fn reload(&mut self) {
        let storeref = self.store.borrow();
        self.times = storeref.get_times().unwrap();
    }
}

impl SelectPane {
    pub fn new(store: Rc<RefCell<dyn Store>>) -> Self {
        let mut pane = Self {
            times: vec![],
            store: store.clone(),
            new_title: "".to_string(),
        };

        pane.reload();

        pane
    }
}
