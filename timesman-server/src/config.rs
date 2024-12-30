use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{default::Default, fs::File, path::PathBuf};

use toml;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub listen: String,
    pub front_type: String,
    pub store_type: String,
    pub store_param: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen: "localhost:8080".to_string(),
            front_type: "http".to_string(),
            store_type: "sqlite".to_string(),
            store_param: "./database.db".to_string(),
        }
    }
}

impl Config {
    pub fn load(path: PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Err("config file is not found".to_string());
        }

        let mut buf = String::new();
        let mut file = File::open(path).map_err(|e| format!("{e}"))?;
        file.read_to_string(&mut buf).map_err(|e| format!("{e}"))?;

        let config: Config =
            toml::from_str(&buf).map_err(|e| format!("{e}"))?;
        Ok(config)
    }
}
