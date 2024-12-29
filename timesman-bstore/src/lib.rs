#[cfg(feature = "json")]
pub mod json;
pub mod ram;
#[cfg(feature = "http")]
pub mod remote;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use async_trait::async_trait;
use chrono;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default)]
pub enum StoreType {
    #[default]
    Memory,
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "http")]
    Remote,
    #[cfg(feature = "sqlite")]
    Sqlite,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Times {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[async_trait]
pub trait Store: Send + Sync + 'static {
    async fn check(&self) -> Result<(), String>;

    // for Times
    async fn get_times(&self) -> Result<Vec<Times>, String>;
    async fn create_times(&mut self, title: String) -> Result<Times, String>;
    async fn delete_times(&mut self, tid: i64) -> Result<(), String>;
    async fn update_times(&mut self, times: Times) -> Result<Times, String>;

    // for Post
    async fn get_posts(&self, tid: i64) -> Result<Vec<Post>, String>;
    async fn create_post(
        &mut self,
        tid: i64,
        post: String,
    ) -> Result<Post, String>;
    async fn delete_post(&mut self, tid: i64, pid: i64) -> Result<(), String>;
    async fn update_post(
        &mut self,
        tid: i64,
        post: Post,
    ) -> Result<Post, String>;

    async fn get_latest_post(&self, tid: i64) -> Result<Option<Post>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
