use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufWriter, Write};
use std::{fs::File, path::PathBuf};

use super::{Post, Store, Times};
use async_trait::async_trait;

pub struct JsonStore {
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
    times: Times,
    posts: Vec<Post>,
}

#[async_trait]
impl Store for JsonStore {
    async fn check(&self) -> Result<(), String> {
        Ok(())
    }

    async fn get_times(&self) -> Result<Vec<super::Times>, String> {
        Ok(vec![self.data.times.clone()])
    }

    async fn create_times(
        &mut self,
        _title: String,
    ) -> Result<super::Times, String> {
        Err("not supported to create times".to_string())
    }

    async fn delete_times(&mut self, _tid: i64) -> Result<(), String> {
        Err("not supported to delete times".to_string())
    }

    async fn update_times(
        &mut self,
        _times: super::Times,
    ) -> Result<Times, String> {
        Err("not supported to update times".to_string())
    }

    async fn get_posts(&self, tid: i64) -> Result<Vec<super::Post>, String> {
        if self.data.times.id != tid {
            return Err("unknown tid found".to_string());
        }

        Ok(self.data.posts.clone())
    }

    async fn create_post(
        &mut self,
        tid: i64,
        _post: String,
    ) -> Result<super::Post, String> {
        if self.data.times.id != tid {
            return Err("unknown tid found".to_string());
        }

        Err("not supported to create post".to_string())
    }

    async fn update_post(
        &mut self,
        tid: i64,
        mut _post: super::Post,
    ) -> Result<super::Post, String> {
        if self.data.times.id != tid {
            return Err("unknown tid found".to_string());
        }

        Err("not supported to update post".to_string())
    }

    async fn delete_post(&mut self, tid: i64, _pid: i64) -> Result<(), String> {
        if self.data.times.id != tid {
            return Err("unknown tid found".to_string());
        }

        Err("not supported to delete post".to_string())
    }

    async fn get_latest_post(&self, tid: i64) -> Option<Post> {
        None
    }
}

impl JsonStore {
    pub fn new(times: Times, posts: Vec<Post>) -> Self {
        Self {
            data: Data { times, posts },
        }
    }

    pub fn build(path: PathBuf) -> Result<Self, String> {
        let data = Self::load_from_file(path)?;

        Ok(Self { data })
    }

    pub fn save_to_file(&self, filepath: &PathBuf) -> Result<(), String> {
        let serialized =
            serde_json::to_string(&self.data).map_err(|e| format!("{e}"))?;

        println!("{serialized}");
        let file = File::create(filepath).map_err(|e| format!("{e}"))?;

        let mut bw = BufWriter::new(file);

        writeln!(bw, "{serialized}").map_err(|e| format!("{e}"))?;

        Ok(())
    }

    fn load_from_file(filepath: PathBuf) -> Result<Data, String> {
        let content =
            fs::read_to_string(filepath).map_err(|e| format!("{e}"))?;

        println!("{content}");

        serde_json::from_str(&content).map_err(|e| format!("{e}"))
    }
}
