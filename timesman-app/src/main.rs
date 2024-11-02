#[macro_use]
mod log;
mod app;
mod pane;
mod req;

use std::sync::Arc;
use std::sync::Mutex;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let logs = Arc::new(Mutex::new(vec![]));
    log::register(logs.clone());
    info!("Starting");

    let r = eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| Ok(Box::<app::App>::new(app::App::new(cc, logs)))),
    );

    info!("Closing");
    r
}
