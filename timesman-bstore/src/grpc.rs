use super::{Arc, Mutex, PostStore, Store, TimesStore, TodoStore};
use async_trait::async_trait;

use timesman_grpc::grpc;
use timesman_grpc::grpc::times_man_client::TimesManClient;
use tonic;

use timesman_type::{File, Post, Tid, Times};

mod times;
use times::GrpcTimesStore;
mod post;
use post::GrpcPostStore;
mod todo;
use todo::GrpcTodoStore;

type GrpcClient =
    Arc<Mutex<TimesManClient<tonic::transport::channel::Channel>>>;

pub struct GrpcStore {
    client: GrpcClient,
}

impl GrpcStore {
    pub async fn new(server: String) -> Self {
        let tclient = TimesManClient::connect(server).await.unwrap();
        let client = Arc::new(Mutex::new(tclient));
        Self { client }
    }

    fn new_times_store(
        &self,
        times: Times,
    ) -> Arc<Mutex<dyn TimesStore + Send + Sync>> {
        let client = self.client.clone();

        Arc::new(Mutex::new(GrpcTimesStore::new(
            client.clone(),
            times.clone(),
        )))
    }
}

#[async_trait]
impl Store for GrpcStore {
    async fn check(&mut self) -> Result<(), String> {
        self.get().await?;
        Ok(())
    }

    async fn get(
        &mut self,
    ) -> Result<Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>>, String> {
        let client = self.client.clone();

        let stores: Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>> = {
            let mut c = self.client.lock().await;

            let gtimes = c.get_times(()).await.unwrap();

            let times: Vec<Times> = gtimes
                .into_inner()
                .timeses
                .iter()
                .map(|t| t.clone().into())
                .collect();

            times
                .iter()
                .map(|t| {
                    let s: Arc<Mutex<dyn TimesStore + Send + Sync>> =
                        Arc::new(Mutex::new(GrpcTimesStore::new(
                            client.clone(),
                            t.clone(),
                        )));
                    s
                })
                .collect()
        };

        Ok(stores)
    }

    async fn create(
        &mut self,
        title: String,
    ) -> Result<Arc<Mutex<dyn TimesStore + Send + Sync>>, String> {
        let client = self.client.clone();
        let title = grpc::TimesTitle { title };

        let mut c = client.lock().await;
        let times = c
            .create_times(tonic::Request::new(title))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(self.new_times_store(times.into_inner().into()))
    }

    async fn delete(&mut self, tid: Tid) -> Result<(), String> {
        todo!();
    }
}


impl std::fmt::Debug for GrpcStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GrpcStore")
    }
}
