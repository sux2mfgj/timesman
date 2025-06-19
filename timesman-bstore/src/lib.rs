mod ram;
use ram::RamStore;

#[cfg(feature = "local")]
mod local;
#[cfg(feature = "local")]
use local::LocalStore;

#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "grpc")]
use grpc::GrpcStore;


use serde::{Deserialize, Serialize};


use std::{fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

use async_trait::async_trait;

use timesman_type::{File, Pid, Post, Tag, Tdid, Tid, Times, Todo};

#[derive(Debug, PartialEq)]
pub enum StoreError {
    NotFound(String),
    InvalidId(String),
    SerializationError(String),
    StorageError(String),
    NetworkError(String),
    PermissionDenied(String),
    NotSupported(String),
    Corrupted(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::NotFound(msg) => write!(f, "Not found: {}", msg),
            StoreError::InvalidId(msg) => write!(f, "Invalid ID: {}", msg),
            StoreError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            StoreError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            StoreError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            StoreError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            StoreError::NotSupported(msg) => write!(f, "Not supported: {}", msg),
            StoreError::Corrupted(msg) => write!(f, "Data corrupted: {}", msg),
        }
    }
}

impl std::error::Error for StoreError {}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub enum StoreType {
    #[default]
    Memory,
    #[cfg(feature = "local")]
    Local(String),
    #[cfg(feature = "grpc")]
    Grpc(String),
}

impl StoreType {
    pub async fn to_store(&self) -> Result<Arc<Mutex<dyn Store>>, StoreError> {
        let store: Arc<Mutex<dyn Store>> = match self {
            Self::Memory => Arc::new(Mutex::new(RamStore::new())),
            #[cfg(feature = "local")]
            Self::Local(path) => {
                Arc::new(Mutex::new(LocalStore::new(&path).await))
            }
            #[cfg(feature = "grpc")]
            Self::Grpc(server_url) => {
                let grpc_store = GrpcStore::new(server_url.clone()).await
                    .map_err(|e| StoreError::NetworkError(e))?;
                Arc::new(Mutex::new(grpc_store))
            }
        };

        Ok(store)
    }
}

#[async_trait]
pub trait Store: Send + Sync + 'static {
    async fn check(&mut self) -> Result<(), StoreError>;
    async fn get(
        &mut self,
    ) -> Result<Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>>, StoreError>;
    async fn create(
        &mut self,
        title: String,
    ) -> Result<Arc<Mutex<dyn TimesStore + Send + Sync>>, StoreError>;
    async fn delete(&mut self, tid: Tid) -> Result<(), StoreError>;
}

#[async_trait]
pub trait TimesStore: Send + Sync + 'static {
    async fn get(&mut self) -> Result<Times, StoreError>;
    async fn update(&mut self, times: Times) -> Result<Times, StoreError>;
    async fn pstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn PostStore + Send + Sync>>, StoreError>;
    async fn tdstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn TodoStore + Send + Sync>>, StoreError>;
}

#[async_trait]
pub trait PostStore: Send + Sync + 'static {
    async fn get(&mut self, pid: Pid) -> Result<Post, StoreError>;
    async fn get_all(&mut self) -> Result<Vec<Post>, StoreError>;
    async fn get_tags(&mut self) -> Result<Vec<Tag>, StoreError>;
    async fn create_tag(&mut self, name: String) -> Result<Tag, StoreError>;
    async fn post(
        &mut self,
        post: String,
        file: Option<File>,
    ) -> Result<Post, StoreError>;
    async fn delete(&mut self, pid: Pid) -> Result<(), StoreError>;
    async fn update(&mut self, post: Post) -> Result<Post, StoreError>;
}

#[async_trait]
pub trait TodoStore: Send + Sync + 'static {
    async fn get(&mut self) -> Result<Vec<Todo>, StoreError>;
    async fn new(&mut self, content: String) -> Result<Todo, StoreError>;
    async fn done(&mut self, tdid: Tdid, done: bool) -> Result<Todo, StoreError>;
    async fn update(&mut self, todo: Todo) -> Result<Todo, StoreError>;
    async fn delete(&mut self, tdid: Tdid) -> Result<(), StoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    async fn test_store(mut store: Box<dyn Store>) {
        // Test creating a TimesStore
        let title = "Test Times".to_string();
        let _times_store = store.create(title.clone()).await.unwrap();

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
    }

    #[test]
    fn test_times_ram_store() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let store = Box::new(RamStore::new());
            test_store(store).await;
        });
    }

    #[cfg(feature = "local")]
    #[test]
    fn test_times_local_store() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let store = Box::new(LocalStore::new(":mem:").await);
            test_store(store).await;
        });
    }

    async fn test_posts(mut store: Box<dyn Store>) {
        let tstore = store.create("post test".to_string()).await.unwrap();

        let mut tstore = tstore.lock().await;
        let pstore = tstore.pstore().await.unwrap();
        let mut pstore = pstore.lock().await;

        let posts = pstore.get_all().await.unwrap();
        assert!(posts.len() == 0);

        let text = "hello".to_string();
        let post = pstore.post(text.clone(), None).await.unwrap();
        assert_eq!(post.post, text);

        let posts = pstore.get_all().await.unwrap();
        assert!(posts.len() == 1);
    }

    #[test]
    fn test_post_ram_store() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let store = Box::new(LocalStore::new(":mem:").await);
            test_posts(store).await;
        });
    }

    #[cfg(feature = "local")]
    #[test]
    fn test_post_local_store() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let store = Box::new(LocalStore::new(":mem:").await);
            test_posts(store).await;
        });
    }

    async fn test_recreate_pstore(mut store: Box<dyn Store>) {
        let tstore = store.create("post test".to_string()).await.unwrap();

        let mut tstore = tstore.lock().await;
        {
            let pstore = tstore.pstore().await.unwrap();
            let mut pstore = pstore.lock().await;

            let posts = pstore.get_all().await.unwrap();
            assert!(posts.len() == 0);

            let text = "hello".to_string();
            let post = pstore.post(text.clone(), None).await.unwrap();
            assert_eq!(post.post, text);
        }
        {
            let pstore = tstore.pstore().await.unwrap();
            let mut pstore = pstore.lock().await;

            let posts = pstore.get_all().await.unwrap();
            assert!(posts.len() == 1);
        }
    }

    #[test]
    fn test_recreate_ram_pstore() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let store = Box::new(LocalStore::new(":mem:").await);
            test_recreate_pstore(store).await;
        });
    }

    #[test]
    fn test_recreate_local_pstore() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let store = Box::new(LocalStore::new(":mem:").await);
            test_recreate_pstore(store).await;
        });
    }
}
