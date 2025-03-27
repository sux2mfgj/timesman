mod app;
mod arbiter;
mod log;
mod pane;

use app::App;
use egui_extras::install_image_loaders;

fn main() -> Result<(), i64> {
    log::tmlog("Starting".to_string());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let r = eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = App::new(cc);
            Ok(Box::new(app))
        }),
    );

    log::tmlog("Closing".to_string());
    r.map_err(|_| 1)
}
