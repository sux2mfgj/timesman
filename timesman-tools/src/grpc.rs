use timesman_bstore::{Post, Times};
use timesman_grpc::grpc::times_man_client::TimesManClient;

pub struct GrpcClient {
    client: TimesManClient<tonic::transport::channel::Channel>,
    rt: tokio::runtime::Runtime,
}

impl super::Client for GrpcClient {
    fn get_times(&mut self) -> Result<Vec<Times>, String> {
        let tary = self
            .rt
            .block_on(async { self.client.get_times(()).await.unwrap() })
            .into_inner();

        let r = tary
            .timeses
            .iter()
            .map(|t| t.clone().into())
            .collect::<Vec<Times>>();
        Ok(r)
    }
}

impl GrpcClient {
    pub fn new(server: &String) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();

        let server: String = server.parse().unwrap();
        let client = rt
            .block_on(async { TimesManClient::connect(server).await.unwrap() });
        Self { client, rt }
    }
}
