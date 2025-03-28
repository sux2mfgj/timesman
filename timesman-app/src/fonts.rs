use egui::{FontData, FontDefinitions};
use std::fs::File;
use std::io::Read;

use crate::log::tmlog;

pub fn load_fonts(cc: &eframe::CreationContext<'_>) {
    let Some(config_dir) = dirs::config_dir() else {
        panic!("Failed to open the config dir");
    };

    let fonts_dir = config_dir.join("timesman").join("fonts");

    let mut fonts = FontDefinitions::default();

    for file in fonts_dir.read_dir().unwrap() {
        let path = file.unwrap().path();
        if !path.is_file() {
            continue;
        }

        let mut file = File::open(path.clone()).unwrap();

        let mut data = vec![];
        file.read_to_end(&mut data);

        let fname = path.file_name().unwrap().to_string_lossy().to_string();

        fonts
            .font_data
            .insert(fname.clone(), FontData::from_owned(data));

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, fname.clone());

        tmlog(format!("Load fonts, {}", fname));
    }

    cc.egui_ctx.set_fonts(fonts);
}
