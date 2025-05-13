use super::{Store, TimesStore};
use async_trait::async_trait;
use timesman_type::Tid;

use std::{fs, path::PathBuf};

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct JsonStore {
    path: PathBuf,
    file: fs::File,
}

impl JsonStore {
    pub fn new(path: PathBuf, is_create: bool) -> Result<Self, String> {
        let file = if is_create {
            if path.exists() {
                return Err(format!("The file {:?} already exists", path));
            }

            fs::File::open(&path).map_err(|e| format!("{e}"))?
        } else {
            if !path.exists() {
                return Err(format!("The file {:?} is not found", path));
            }

            fs::File::create(&path).map_err(|e| format!("{e}"))?
        };

        let store = Self { path, file };

        Ok(store)
    }
}

#[async_trait]
impl Store for JsonStore {
    async fn check(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn get(
        &mut self,
    ) -> Result<Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>>, String> {
        todo!();
    }

    async fn create(
        &mut self,
        title: String,
    ) -> Result<Arc<Mutex<dyn TimesStore + Send + Sync>>, String> {
        todo!();
    }

    async fn delete(&mut self, tid: Tid) -> Result<(), String> {
        todo!();
    }
}
