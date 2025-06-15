use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{default::Default, fs::File, path::PathBuf};
use timesman_bstore::StoreType;

use toml;

#[derive(Deserialize, Serialize, Clone)]
pub enum FrontType {
    Http,
    Grpc,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct StoreConfig {
    #[serde(rename = "type")]
    pub store_type: String,
    pub path: Option<String>,
    pub create: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub listen: String,
    pub front_type: FrontType,
    pub store: StoreConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen: "localhost:8080".to_string(),
            front_type: FrontType::Grpc,
            store: StoreConfig {
                store_type: "Memory".to_string(),
                path: None,
                create: None,
            },
        }
    }
}

impl StoreConfig {
    fn expand_path(path: &str) -> String {
        if path.starts_with("~/") {
            if let Ok(home) = std::env::var("HOME") {
                path.replacen("~", &home, 1)
            } else {
                path.to_string()
            }
        } else {
            path.to_string()
        }
    }

    pub fn to_store_type(&self) -> Result<StoreType, String> {
        match self.store_type.as_str() {
            "Memory" => Ok(StoreType::Memory),
            #[cfg(feature = "local")]
            "Local" => {
                let path = self.path.as_ref()
                    .ok_or_else(|| "Local store requires path parameter".to_string())?;
                let expanded_path = Self::expand_path(path);
                Ok(StoreType::Local(expanded_path))
            }
            #[cfg(feature = "json")]
            "Json" => {
                let path = self.path.as_ref()
                    .ok_or_else(|| "Json store requires path parameter".to_string())?;
                let expanded_path = Self::expand_path(path);
                let create = self.create.unwrap_or(false);
                Ok(StoreType::Json(expanded_path.into(), create))
            }
            _ => Err(format!("Unknown store type: {}", self.store_type))
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
