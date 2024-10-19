use eframe::egui::ScrollArea;

struct TimesManApp {
    input_text: String,
}

impl Default for TimesManApp {
    fn default() -> Self {
        Self {
            input_text: "".to_owned(),
        }
    }
}

impl eframe::App for TimesManApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TimesMan");
            ui.separator();
            let scroll_area = ScrollArea::vertical().max_height(200.0).auto_shrink(false);
            scroll_area.show(ui, |ui| {
                ui.vertical(|ui| {
                    for i in 1..30 {
                        ui.horizontal(|ui| {
                            ui.label("20:21");
                            ui.label(format!("test {i}"));
                        });
                    }
                });
            });

            ui.separator();

            egui::TextEdit::multiline(&mut self.input_text)
                .hint_text("Type something!")
                .show(ui);
            if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::Enter)) {
                if self.input_text.is_empty() {
                    return;
                }

                let _text = self.input_text.clone();

                //TODO: send to server
                self.input_text.clear();
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<TimesManApp>::default())),
    )
}
