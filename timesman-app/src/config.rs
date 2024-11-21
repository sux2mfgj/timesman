use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use toml;
// use xdg;

pub struct FontFile {
    pub data: Vec<u8>,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip)]
    pub fonts: Vec<FontFile>,
    pub store: String,
    //pub plugins: Option<Plugin>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fonts: vec![],
            //store: "http://localhost:8080".to_string(),
            store: "sqlite3:database.db".to_string(),
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Self, String> {
        let home = env::var("HOME").unwrap();
        let config_dir = home + "/.config/timesman/";
        let dir_path = PathBuf::from(config_dir.clone());

        if !dir_path.exists() {
            match fs::create_dir(config_dir.clone()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!(
                        "failed to create {} {}",
                        config_dir, e
                    ));
                }
            }
        }

        let config_file_path = dir_path.join("config.toml");
        let file = if !config_file_path.exists() {
            let config_str = toml::to_string(&Config::default()).unwrap();
            let mut file = File::create(config_file_path).unwrap();
            write!(file, "{}", config_str).unwrap();
            file
        } else {
            File::open(config_file_path).unwrap()
        };

        let mut config = Self::from_reader(file);

        config.load_font_files(dir_path).unwrap();

        Ok(config)
    }

    fn load_font_files(&mut self, mut dir: PathBuf) -> Result<(), String> {
        dir.push("fonts");

        let entries = dir.read_dir().unwrap(); //map_err(|e| Error(format!("{}", e)))?;

        for entry in entries.into_iter() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            let mut file = File::open(path.clone()).unwrap();

            let mut font_data = vec![];
            let _ = file.read_to_end(&mut font_data);

            let fname: String =
                path.file_stem().unwrap().to_string_lossy().into_owned();

            info!(format!("find font file: {}", &fname));

            self.fonts.push(FontFile {
                data: font_data,
                name: fname,
            });
        }

        Ok(())
    }

    fn from_reader(mut reader: impl Read) -> Self {
        let mut buf = String::new();

        reader.read_to_string(&mut buf).unwrap();

        toml::from_str(&buf).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
