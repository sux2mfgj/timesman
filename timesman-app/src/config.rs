use std::fs;
use std::path::PathBuf;

use dirs;
use serde::{Deserialize, Serialize};
use toml;

use crate::app::{AppRequest, UIRequest};
use crate::log::tmlog;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Config {
    ui: UIConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UIConfig {
    pub scale: f32,
    pub window_size: WindowConfig,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            scale: 1.5,
            window_size: WindowConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    height: f32,
    width: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            height: 600f32,
            width: 400f32,
        }
    }
}

impl Config {
    fn load_config_file(path: &PathBuf) -> Result<Config, String> {
        if !path.exists() {
            fs::create_dir(path).map_err(|e| {
                format!("failed to create the config directory: {e}")
            })?;
        }

        if !(path.exists() && path.is_dir()) {
            return Err("Failed to open the config directory".to_string());
        }

        let config_file_path = path.join("config.toml");

        if !config_file_path.exists() {
            fs::write(
                &config_file_path,
                toml::to_string(&Config::default()).unwrap(),
            )
            .unwrap();
        }

        tmlog(format!("Load config from {:?}", config_file_path));
        let config_str = fs::read_to_string(config_file_path).unwrap();

        let config = toml::from_str(&config_str).unwrap();

        Ok(config)
    }

    pub fn load() -> Result<Self, String> {
        let Some(config_dir) = dirs::config_dir() else {
            return Err("Failed to open the config dir".to_string());
        };

        let tm_config_dir = config_dir.join("timesman");
        let config = Self::load_config_file(&tm_config_dir)?;

        Ok(config)
    }

    pub fn generate_pane_reqs(&self) -> Vec<AppRequest> {
        let mut reqs = vec![];

        reqs.push(AppRequest::UI(UIRequest::ChangeScale(self.ui.scale)));
        reqs.push(AppRequest::UI(UIRequest::ChangeWindowSize(
            self.ui.window_size.height,
            self.ui.window_size.width,
        )));

        reqs
    }
}
