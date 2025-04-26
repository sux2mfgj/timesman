use async_trait::async_trait;
use std::fmt;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::sync::Arc;
use tokio::runtime;
use tokio::sync::Mutex;

use timesman_bstore::{Store, StoreEvent};
use timesman_server::{GrpcServer, TimesManServer};
use timesman_type::{File, Post, Times};

pub struct ArbiterStore {
    store: Arc<Mutex<dyn Store + Send + Sync>>,
    rx: Arc<Mutex<Receiver<StoreEvent>>>,
}

impl ArbiterStore {
    pub fn new(
        rt: &runtime::Runtime,
        store: Arc<Mutex<dyn Store>>,
        listen: &str,
    ) -> Self {
        let (tx, rx) = channel();

        let s = store.clone();
        let l = listen.to_string();
        rt.spawn(async move {
            let server = GrpcServer {};
            server.run(&l, s, Some(tx)).await
        });
        Self {
            store,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}

#[async_trait]
impl Store for ArbiterStore {
    async fn check(&mut self) -> Result<(), String> {
        let mut store = self.store.lock().await;
        store.check().await?;

        let rx = self.rx.lock().await;
        loop {
            match rx.try_recv() {
                Ok(_) => {
                    todo!();
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(_e) => {}
            }
        }

        Ok(())
    }

    // for Times
    async fn get_times(&mut self) -> Result<Vec<Times>, String> {
        let mut store = self.store.lock().await;
        store.get_times().await
    }
    async fn create_times(&mut self, title: String) -> Result<Times, String> {
        let mut store = self.store.lock().await;
        store.create_times(title).await
    }
    async fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        let mut store = self.store.lock().await;
        store.delete_times(tid).await
    }
    async fn update_times(&mut self, times: Times) -> Result<Times, String> {
        let mut store = self.store.lock().await;
        store.update_times(times).await
    }

    // for Post
    async fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        let mut store = self.store.lock().await;
        store.get_posts(tid).await
    }
    async fn create_post(
        &mut self,
        tid: u64,
        post: String,
        file: Option<(String, File)>,
    ) -> Result<Post, String> {
        let mut store = self.store.lock().await;
        store.create_post(tid, post, file).await
    }
    async fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        let mut store = self.store.lock().await;
        store.delete_post(tid, pid).await
    }
    async fn update_post(
        &mut self,
        tid: u64,
        post: Post,
    ) -> Result<Post, String> {
        let mut store = self.store.lock().await;
        store.update_post(tid, post).await
    }

    async fn get_latest_post(
        &mut self,
        tid: u64,
    ) -> Result<Option<Post>, String> {
        let mut store = self.store.lock().await;
        store.get_latest_post(tid).await
    }
}

impl fmt::Debug for ArbiterStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " ArbiterStore")
    }
}
