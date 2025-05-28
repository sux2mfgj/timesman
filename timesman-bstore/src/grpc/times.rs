use super::{async_trait, Arc, Mutex};
use super::{GrpcClient, PostStore, TimesStore, TodoStore};
use super::{GrpcPostStore, GrpcTodoStore};

use timesman_type::{File, Post, Tid, Times};

pub(crate) struct GrpcTimesStore {
    client: GrpcClient,
    times: Times,
}

impl GrpcTimesStore {
    pub fn new(client: GrpcClient, times: Times) -> Self {
        Self { client, times }
    }
}

#[async_trait]
impl TimesStore for GrpcTimesStore {
    async fn get(&mut self) -> Result<Times, String> {
        Ok(self.times.clone())
    }

    async fn update(&mut self, times: Times) -> Result<Times, String> {
        todo!();
    }

    async fn pstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn PostStore + Send + Sync>>, String> {
        Ok(Arc::new(Mutex::new(GrpcPostStore::new(
            self.client.clone(),
            self.times.id,
        ))))
    }

    async fn tdstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn TodoStore + Send + Sync>>, String> {
        Ok(Arc::new(Mutex::new(GrpcTodoStore::new(
            self.client.clone(),
            self.times.id,
        ))))
    }
}
