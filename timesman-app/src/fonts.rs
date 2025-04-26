use egui::{FontData, FontDefinitions};
use std::fs::{create_dir, File};
use std::io::Read;
use std::path::PathBuf;

use crate::log::tmlog;

fn load_font(
    path: &PathBuf,
    fonts: &mut FontDefinitions,
) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("it is not file, {:?}", path));
    }

    let mut file = File::open(path.clone()).unwrap();

    let mut data = vec![];
    file.read_to_end(&mut data)
        .map_err(|e| format!("failed to read: {e}"))?;

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

    Ok(())
}

pub fn load_fonts(cc: &eframe::CreationContext<'_>) {
    let Some(config_dir) = dirs::config_dir() else {
        panic!("Failed to open the config dir");
    };

    let fonts_dir = config_dir.join("timesman").join("fonts");

    if !fonts_dir.exists() {
        create_dir(&fonts_dir).unwrap();
    }

    let mut fonts = FontDefinitions::default();

    for file in fonts_dir.read_dir().unwrap() {
        let path = file.unwrap().path();

        load_font(&path, &mut fonts).unwrap();
    }

    cc.egui_ctx.set_fonts(fonts);
}
