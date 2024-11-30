use std::fs;
use std::fs::File;
use std::io::Read;
use xdg;

use egui::{FontData, FontDefinitions, FontFamily};

#[derive(Clone)]
struct FontFile {
    name: String,
    data: Vec<u8>,
}

#[derive(Clone)]
pub struct Fonts {
    fonts: Vec<FontFile>,
}

impl Fonts {
    pub fn new(base: xdg::BaseDirectories) -> Result<Self, String> {
        let dir = base.get_config_home().join("fonts");

        if !dir.exists() {
            fs::create_dir(dir).map_err(|e| format!("{e}"))?;

            return Ok(Self { fonts: vec![] });
        }

        let mut fonts = vec![];

        for file in dir.read_dir().map_err(|e| format!("{e}"))?.into_iter() {
            let path = file.map_err(|e| format!("{e}"))?.path();

            if !path.is_file() {
                continue;
            }

            let mut file = File::open(path.clone()).unwrap();

            let mut font_data = vec![];
            let _ = file.read_to_end(&mut font_data);

            let fname: String =
                path.file_stem().unwrap().to_string_lossy().into_owned();

            info!(format!("find font file: {}", &fname));

            fonts.push(FontFile {
                data: font_data,
                name: fname,
            });
        }

        Ok(Self { fonts })
    }

    pub fn load_fonts(&self, cc: &eframe::CreationContext<'_>) {
        let mut fonts = FontDefinitions::default();

        for font in &self.fonts {
            let name = font.name.clone();
            info!(format!("Loading font ({})", &name));

            fonts.font_data.insert(
                name.clone().to_owned(),
                FontData::from_owned(font.data.clone()),
            );

            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, name.to_owned());
        }

        cc.egui_ctx.set_fonts(fonts);
    }
}
