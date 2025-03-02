use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::default::Default;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use toml;
use xdg;

use crate::app::{Event, UIOperation};
use crate::fonts::Fonts;

#[derive(Clone)]
pub struct Config {
    pub base: xdg::BaseDirectories,
    pub params: ConfigParam,
    pub fonts: Fonts,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigParam {
    pub store: String,
    pub sqlite: SqliteConfig,
    pub remote: RemoteConfig,
    pub ui: UIConfig,
    pub grpc: GrpcConfig,
}

impl Default for ConfigParam {
    fn default() -> Self {
        Self {
            store: "sqlite".to_string(),
            sqlite: SqliteConfig::default(),
            remote: RemoteConfig::default(),
            ui: UIConfig::default(),
            grpc: GrpcConfig::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SqliteConfig {
    pub db: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RemoteConfig {
    pub server: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GrpcConfig {
    pub server: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WindowConfig {
    height: f32,
    width: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UIConfig {
    pub scale: f32,
    pub window_size: WindowConfig,
}

impl Default for SqliteConfig {
    fn default() -> Self {
        let base = xdg::BaseDirectories::with_prefix("timesman").unwrap();
        let dbname = "database.db";
        let db = if let Some(db) = base.find_data_file(dbname) {
            db
        } else {
            base.place_data_file(dbname).unwrap()
        };

        Self {
            db: db.to_string_lossy().to_string(),
        }
    }
}

impl Default for RemoteConfig {
    fn default() -> Self {
        Self {
            server: "http://localhost:8080".to_string(),
        }
    }
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            server: "http://localhost:8080".to_string(),
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            scale: 1.0,
            window_size: WindowConfig {
                height: 600.0,
                width: 400.0,
            },
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Self, String> {
        let base = xdg::BaseDirectories::with_prefix("timesman").unwrap();

        let config_file_name = "config.toml";
        let config = base.get_config_file(config_file_name);

        let params = if !config.exists() {
            let path = base
                .place_config_file(config_file_name)
                .map_err(|e| format!("{e}"))?;

            let param = ConfigParam::default();
            let param_str =
                toml::to_string(&param).map_err(|e| format!("{e}"))?;
            let mut file = File::create(path.clone()).map_err(|e| {
                format!(
                    "failed to open the config file: {} {}",
                    path.to_string_lossy(),
                    e
                )
            })?;
            write!(file, "{}", param_str).unwrap();

            param
        } else {
            let path = base
                .find_config_file(config_file_name)
                .ok_or("can't found config file")?;

            let mut buf = String::new();

            let mut file = File::open(path).map_err(|e| format!("{e}"))?;
            file.read_to_string(&mut buf).map_err(|e| format!("{e}"))?;

            toml::from_str(&buf).map_err(|e| format!("{e}"))?
        };

        let fonts = Fonts::new(base.clone())?;

        Ok(Self {
            base,
            params,
            fonts,
        })
    }

    pub fn store_config(&self) -> Result<(), String> {
        let config_file_name = "config.toml";
        let path = self
            .base
            .find_config_file(config_file_name)
            .ok_or("Can't found config file")?;
        let file =
            File::open(&path).map_err(|e| format!("{e}: {:?}", path))?;

        let mut bw = BufWriter::new(file);
        let param_str =
            toml::to_string(&self.params).map_err(|e| format!("{e}"))?;

        write!(bw, "{}", param_str)
            .map_err(|e| format!("Error on try writing config. {e}"))?;

        Ok(())
    }

    pub fn append_init_events(&self, queue: &mut VecDeque<Event>) {
        queue.push_back(Event::ChangeUI(UIOperation::ChangeWindowSize(
            self.params.ui.window_size.width,
            self.params.ui.window_size.height,
        )));
        queue.push_back(Event::ChangeUI(UIOperation::ChangeScale(
            self.params.ui.scale,
        )));
    }
}
