use crate::app::Event;
use crate::log::LogRecord;
use eframe::egui::ScrollArea;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::runtime;

use super::Pane;

pub struct LogPane {
    logs: Arc<Mutex<Vec<LogRecord>>>,
}

impl LogPane {
    pub fn new(logs: Arc<Mutex<Vec<LogRecord>>>) -> Self {
        Self { logs: logs.clone() }
    }
}

impl Pane for LogPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        _rt: &runtime::Runtime,
    ) -> Option<Event> {
        let mut event = None;

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.times_menu(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(ui.available_height())
                .stick_to_bottom(true);

            scroll_area.show(ui, |ui| {
                let records = self.logs.lock().unwrap();
                for r in &*records {
                    ui.horizontal(|ui| {
                        r.show(ui);
                    });
                }
            });
        });

        event
    }
    fn reload(&mut self, _rt: &runtime::Runtime) {}
}
