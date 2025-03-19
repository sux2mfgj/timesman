mod app;
mod pane;

use app::App;

fn main() -> Result<(), i64> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let r = eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| {
            let app = App::new(cc);
            Ok(Box::new(app))
        }),
    );

    r.map_err(|_| 1)
}
