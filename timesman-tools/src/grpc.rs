use timesman_grpc::grpc::times_man_client::TimesManClient;
use timesman_grpc::grpc::{TimesTitle, TimesId, CreatePostPrams, DeletePostParam, UpdatePostParam};
use timesman_type::{Post, Times};

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
        let request = TimesTitle { title };
        let response = self
            .rt
            .block_on(async { self.client.create_times(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        let request = TimesId { id: tid };
        self.rt
            .block_on(async { self.client.delete_times(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(())
    }

    fn update_times(&mut self, times: Times) -> Result<Times, String> {
        let request: timesman_grpc::grpc::Times = times.into();
        let response = self
            .rt
            .block_on(async { self.client.update_times(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        let request = TimesId { id: tid };
        let response = self
            .rt
            .block_on(async { self.client.get_posts(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        let posts = response
            .into_inner()
            .posts
            .iter()
            .map(|p| p.clone().into())
            .collect::<Vec<Post>>();
        
        Ok(posts)
    }
    fn create_post(&mut self, tid: u64, text: String) -> Result<Post, String> {
        let request = CreatePostPrams { id: tid, text };
        let response = self
            .rt
            .block_on(async { self.client.create_post(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        let request = DeletePostParam { tid, pid };
        self.rt
            .block_on(async { self.client.delete_post(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(())
    }

    fn update_post(&mut self, tid: u64, post: Post) -> Result<Post, String> {
        let request = UpdatePostParam { 
            tid, 
            post: Some(post.into()) 
        };
        let response = self
            .rt
            .block_on(async { self.client.update_post(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
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
