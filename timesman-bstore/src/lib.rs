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
    use std::path::PathBuf;
    use tokio::runtime::Runtime;

    #[test]
    fn test_ram_store() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut store = RamStore::new();

            // Test creating a TimesStore
            let title = "Test Times".to_string();
            let times_store = store.create(title.clone()).await.unwrap();

            // Test retrieving TimesStore
            let times_list = store.get().await.unwrap();
            assert_eq!(times_list.len(), 1);

            let times = times_list[0].lock().await.get().await.unwrap();
            assert_eq!(times.title, title);

            // Test deleting TimesStore
            let tid = times.id;
            store.delete(tid).await.unwrap();
            let times_list = store.get().await.unwrap();
            assert!(times_list.is_empty());
        });
    }

    #[test]
    fn test_local_store() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut store = LocalStore::new(":mem:").await;

            // Test creating a TimesStore
            let title = "Test Local Times".to_string();
            let times_store = store.create(title.clone()).await.unwrap();

            // Test retrieving TimesStore
            let times_list = store.get().await.unwrap();
            assert_eq!(times_list.len(), 1);

            let times = times_list[0].lock().await.get().await.unwrap();
            assert_eq!(times.title, title);

            // Test deleting TimesStore
            let tid = times.id;
            store.delete(tid).await.unwrap();
            let times_list = store.get().await.unwrap();
            assert!(times_list.is_empty());
        });
    }

    #[test]
    fn test_local_store_with_times_post_todo() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut store = LocalStore::new(":mem:").await;

            // Test creating a TimesStore
            let title = "Test Times".to_string();
            let times_store = store.create(title.clone()).await.unwrap();

            // Test TimesStore operations
            let times = times_store.lock().await.get().await.unwrap();
            assert_eq!(times.title, title);

            /*
                        let updated_title = "Updated Times".to_string();
                        let mut updated_times = times.clone();
                        updated_times.title = updated_title.clone();
                        let updated = times_store
                            .lock()
                            .await
                            .update(updated_times)
                            .await
                            .unwrap();
                        assert_eq!(updated.title, updated_title);
            */

            // Test PostStore operations
            let post_store = times_store.lock().await.pstore().await.unwrap();
            let post_content = "Test Post".to_string();
            let post = post_store
                .lock()
                .await
                .post(post_content.clone(), None)
                .await
                .unwrap();
            assert_eq!(post.post, post_content);

            let posts = post_store.lock().await.get_all().await.unwrap();
            assert_eq!(posts.len(), 1);

            // Test TodoStore operations
            //let todo_store = times_store.lock().await.tdstore().await.unwrap();
            //let todo_content = "Test Todo".to_string();
            //let todo = todo_store
            //    .lock()
            //    .await
            //    .new(todo_content.clone())
            //    .await
            //    .unwrap();
            //assert_eq!(todo.content, todo_content);

            //let todos = todo_store.lock().await.get().await.unwrap();
            //assert_eq!(todos.len(), 1);
        });
    }
}
