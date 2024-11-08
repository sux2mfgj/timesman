use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    fonts: Option<String>,
    server: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fonts: None,
            server: "http://localhost:8080/".to_string(),
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Self, String> {
        let config_dir = "~/.config/timesman/";
        let dir_path = PathBuf::from(config_dir);

        if !dir_path.exists() {
            match fs::create_dir(config_dir) {
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

        let config = Self::from_reader(file);

        Ok(config)
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
