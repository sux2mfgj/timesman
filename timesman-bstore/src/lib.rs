//#[cfg(feature = "grpc")]
//pub mod grpc;
//#[cfg(feature = "json")]
//pub mod json;
mod ram;
pub use ram::RamStore;
#[cfg(feature = "http")]
pub mod remote;
//#[cfg(feature = "sqlite")]
//mod sqlite;
//pub use sqlite::{SqliteStore, SqliteStoreBuilder};

use std::fmt::Debug;

use async_trait::async_trait;

use timesman_type::{Post, Times};

#[derive(PartialEq, Default, Debug, Clone)]
pub enum StoreType {
    #[default]
    Memory,
    //#[cfg(feature = "json")]
    //Json,
    //#[cfg(feature = "http")]
    //Remote,
    //#[cfg(feature = "sqlite")]
    //Sqlite(String),
}

#[async_trait]
pub trait Store: Send + Sync + 'static + Debug {
    async fn check(&mut self) -> Result<(), String>;

    // for Times
    async fn get_times(&mut self) -> Result<Vec<Times>, String>;
    async fn create_times(&mut self, title: String) -> Result<Times, String>;
    async fn delete_times(&mut self, tid: u64) -> Result<(), String>;
    async fn update_times(&mut self, times: Times) -> Result<Times, String>;

    // for Post
    async fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String>;
    async fn create_post(
        &mut self,
        tid: u64,
        post: String,
    ) -> Result<Post, String>;
    async fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String>;
    async fn update_post(
        &mut self,
        tid: u64,
        post: Post,
    ) -> Result<Post, String>;

    async fn get_latest_post(
        &mut self,
        tid: u64,
    ) -> Result<Option<Post>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
