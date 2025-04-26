mod app;
mod arbiter;
mod config;
mod fonts;
mod log;
mod pane;

use app::App;

fn main() -> Result<(), i64> {
    log::tmlog("Starting".to_string());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let config = config::Config::load().map_err(|e| {
        log::tmlog(e);
        1
    })?;

    let r = eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            fonts::load_fonts(&cc);
            let app = App::new(cc, config.clone());
            Ok(Box::new(app))
        }),
    );

    log::tmlog("Closing".to_string());
    r.map_err(|e| {
        log::tmlog(format!("err: {e}"));
        1
    })
}
