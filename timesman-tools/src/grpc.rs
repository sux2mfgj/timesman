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

    fn create_times(&mut self, title: String) -> Result<Times, String> {
        Err("not yet implemented".to_string())
    }

    fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        Err("not yet implemented".to_string())
    }

    fn update_times(&mut self, times: Times) -> Result<Times, String> {
        Err("not yet implemented".to_string())
    }

    fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        Err("not yet implemented".to_string())
    }
    fn create_post(&mut self, tid: u64, text: String) -> Result<Post, String> {
        Err("not yet implemented".to_string())
    }

    fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        Err("not yet implemented".to_string())
    }

    fn update_post(&mut self, tid: u64, post: Post) -> Result<Post, String> {
        Err("not yet implemented".to_string())
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
