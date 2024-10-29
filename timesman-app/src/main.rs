mod app;
mod req;
mod start;
mod times;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| Ok(Box::<app::App>::new(app::App::new(cc)))),
    )
}
