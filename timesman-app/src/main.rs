#[macro_use]
mod log;
mod app;
mod config;
mod pane;
mod req;

use std::sync::Arc;
use std::sync::Mutex;

use crate::config::Config;

fn main() -> Result<(), i64> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let logs = Arc::new(Mutex::new(vec![]));
    log::register(logs.clone());
    info!("Starting");

    let config = Config::load_config().map_err(|e| {
        error!(format!("{}", e));
        1
    });

    let r = eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| {
            Ok(Box::<app::App>::new(app::App::new(cc, config, logs)))
        }),
    );

    info!("Closing");
    r.map_err(|e| {
        error!(format!("{}", e));
        1
    })
}
