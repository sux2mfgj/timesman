mod ram;
use ram::RamStore;

#[cfg(feature = "local")]
mod local;
#[cfg(feature = "local")]
use local::LocalStore;

#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "sqlite")]
use sqlite::SqliteStore;
#[cfg(feature = "sqlite")]
use std::path::PathBuf;

//#[cfg(feature = "grpc")]
//mod grpc;
//#[cfg(feature = "grpc")]
//use grpc::GrpcStore;

//#[cfg(feature = "json")]
//mod json;
//#[cfg(feature = "json")]
//use json::JsonStore;

//#[cfg(feature = "http")]
//mod remote;
//#[cfg(feature = "http")]
//use remote::RemoteStore;

use std::{fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

use async_trait::async_trait;

use timesman_type::{File, Pid, Post, Tdid, Tid, Times, Todo};

#[derive(PartialEq, Default, Debug, Clone)]
pub enum StoreType {
    #[default]
    Memory,
    //#[cfg(feature = "json")]
    //Json,
    //#[cfg(feature = "http")]
    //Remote,
    #[cfg(feature = "sqlite")]
    Sqlite(String, PathBuf),
    //#[cfg(feature = "grpc")]
    //Grpc(String),
    #[cfg(feature = "local")]
    Local(String),
}

impl StoreType {
    pub async fn to_store(&self) -> Result<Arc<Mutex<dyn Store>>, String> {
        let store: Arc<Mutex<dyn Store>> = match self {
            Self::Memory => Arc::new(Mutex::new(RamStore::new())),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(db_path, file_path) => Arc::new(Mutex::new(
                SqliteStore::new(db_path, file_path).await.unwrap(),
            )),
            #[cfg(feature = "local")]
            Self::Local(path) => {
                Arc::new(Mutex::new(LocalStore::new(&path).await))
            }
        };

        Ok(store)
    }
}

#[async_trait]
pub trait Store: Send + Sync + 'static {
    async fn check(&mut self) -> Result<(), String>;
    async fn get(
        &mut self,
    ) -> Result<Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>>, String>;
    async fn create(
        &mut self,
        title: String,
    ) -> Result<Arc<Mutex<dyn TimesStore + Send + Sync>>, String>;
    async fn delete(&mut self, tid: Tid) -> Result<(), String>;
}

#[async_trait]
pub trait TimesStore: Send + Sync + 'static {
    async fn get(&mut self) -> Result<Times, String>;
    async fn update(&mut self, times: Times) -> Result<Times, String>;
    async fn pstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn PostStore + Send + Sync>>, String>;
    async fn tdstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn TodoStore + Send + Sync>>, String>;
}

#[async_trait]
pub trait PostStore: Send + Sync + 'static {
    async fn get(&mut self, pid: Pid) -> Result<Post, String>;
    async fn get_all(&mut self) -> Result<Vec<Post>, String>;
    async fn post(
        &mut self,
        post: String,
        file: Option<(String, File)>,
    ) -> Result<Post, String>;
    async fn delete(&mut self, pid: Pid) -> Result<(), String>;
    async fn update(&mut self, post: Post) -> Result<Post, String>;
}

#[async_trait]
pub trait TodoStore: Send + Sync + 'static {
    async fn get(&mut self) -> Result<Vec<Todo>, String>;
    async fn new(&mut self, content: String) -> Result<Todo, String>;
    async fn update(&mut self, todo: Todo) -> Result<Todo, String>;
    async fn delete(&mut self, tdid: Tdid) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
