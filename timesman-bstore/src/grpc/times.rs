use super::{async_trait, Arc, Mutex};
use super::{GrpcClient, PostStore, TimesStore, TodoStore};
use super::{GrpcPostStore, GrpcTodoStore};

use timesman_type::{File, Post, Tid, Times};
use tonic;

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
        let mut c = self.client.lock().await;
        let updated_times = c
            .update_times(tonic::Request::new(times.into()))
            .await
            .map_err(|e| format!("{e}"))?;
        
        let result: Times = updated_times.into_inner().into();
        self.times = result.clone();
        Ok(result)
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
