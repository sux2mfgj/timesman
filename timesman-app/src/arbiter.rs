use timesman_bstore::Store;
use async_trait::async_trait;
use timesman_type::{Post, Times};

use crate::app::AsyncStore;

pub struct StoreArbiter {
    store: AsyncStore,
}

impl StoreArbiter {
    pub fn new(store: AsyncStore) -> Self {
        Self {
            store
        }
    }
}

#[async_trait]
impl Store for StoreArbiter {
    async fn check(&mut self) -> Result<(), String> {
        //TODO: Implement
        self.store.lock().await.check().await   
    }

    async fn get_times(&mut self) -> Result<Vec<Times>, String> {
        self.store.lock().await.get_times().await
    }

    async fn create_times(
        &mut self,
        title: String,
    ) -> Result<Times, String> {
        self.store.lock().await.create_times(title).await
    }

    async fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        self.store.lock().await.delete_times(tid).await
    }       

    async fn update_times(&mut self, times: Times) -> Result<Times, String> {
        self.store.lock().await.update_times(times).await
    }

    async fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        self.store.lock().await.get_posts(tid).await
    }

    async fn create_post(
        &mut self,
        tid: u64,
        post: String,
    ) -> Result<Post, String> {
        self.store.lock().await.create_post(tid, post).await
    }

    async fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        self.store.lock().await.delete_post(tid, pid).await
    }

    async fn update_post(
        &mut self,
        tid: u64,
        post: Post,
    ) -> Result<Post, String> {
        self.store.lock().await.update_post(tid, post).await
    }

    async fn get_latest_post(
        &mut self,
        tid: u64,
    ) -> Result<Option<Post>, String> {
        self.store.lock().await.get_latest_post(tid).await
    }
}