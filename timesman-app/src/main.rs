use eframe::egui;

struct TimesManApp {
    name: String,
}

impl Default for TimesManApp {
    fn default() -> Self {
        Self {
            name: "TimesMan".to_owned(),
        }
    }
}

impl eframe::App for TimesManApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.heading("TimesMan"));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::<TimesManApp>::default())),
    )
}
