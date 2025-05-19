use super::async_trait;
use super::todo::LocalTodoStore;
use super::{Arc, Mutex, Times, UnQLite};
use super::{PostStore, TimesStore, TodoStore};

use super::LocalPostStore;

pub struct LocalTimesStore {
    times: Times,
    store: Arc<Mutex<UnQLite>>,
}

impl LocalTimesStore {
    pub fn new(times: Times, store: Arc<Mutex<UnQLite>>) -> Self {
        Self { times, store }
    }
}

// /{tid}/mata.data
#[async_trait]
impl TimesStore for LocalTimesStore {
    async fn get(&mut self) -> Result<Times, String> {
        Ok(self.times.clone())
    }

    async fn update(&mut self, times: Times) -> Result<Times, String> {
        self.times = times.clone();
        todo!("update /times/{}/meta.data", times.id);
    }

    async fn pstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn PostStore + Send + Sync>>, String> {
        let pstore: Arc<Mutex<dyn PostStore + Send + Sync>> =
            Arc::new(Mutex::new(
                LocalPostStore::new(self.times.id, self.store.clone()).await,
            ));

        Ok(pstore)
    }

    async fn tdstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn TodoStore + Send + Sync>>, String> {
        let tdstore: Arc<Mutex<dyn TodoStore + Send + Sync>> =
            Arc::new(Mutex::new(
                LocalTodoStore::new(self.times.id, self.store.clone()).await?,
            ));

        Ok(tdstore)
    }
}
