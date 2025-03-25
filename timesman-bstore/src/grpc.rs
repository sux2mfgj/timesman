use super::Store;
use async_trait::async_trait;

use timesman_grpc::grpc;
use timesman_grpc::grpc::times_man_client::TimesManClient;
use tonic;

use timesman_type::{File, Post, Times};

pub struct GrpcStore {
    client: TimesManClient<tonic::transport::channel::Channel>,
}

impl GrpcStore {
    pub async fn new(server: String) -> Self {
        let client = TimesManClient::connect(server).await.unwrap();
        Self { client }
    }
}

#[async_trait]
impl Store for GrpcStore {
    async fn check(&mut self) -> Result<(), String> {
        self.get_times().await?;
        Ok(())
    }

    // for Times
    async fn get_times(&mut self) -> Result<Vec<timesman_type::Times>, String> {
        let gtimes = self
            .client
            .get_times(())
            .await
            .map_err(|e| format!("{e}"))?;

        let times = gtimes
            .into_inner()
            .timeses
            .iter()
            .map(|t| t.clone().into())
            .collect();
        Ok(times)
    }

    async fn create_times(&mut self, title: String) -> Result<Times, String> {
        let title = grpc::TimesTitle { title };
        let times = self
            .client
            .create_times(tonic::Request::new(title))
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(times.into_inner().into())
    }

    async fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        let id = grpc::TimesId { id: tid };
        self.client
            .delete_times(tonic::Request::new(id))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(())
    }

    async fn update_times(&mut self, times: Times) -> Result<Times, String> {
        let times = self
            .client
            .update_times(tonic::Request::new(times.into()))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(times.into_inner().into())
    }

    // for Post
    async fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        let tid = grpc::TimesId { id: tid };
        let posts = self
            .client
            .get_posts(tonic::Request::new(tid))
            .await
            .map_err(|e| format!("{e}"))?;

        let posts = posts
            .into_inner()
            .posts
            .iter()
            .map(|t| t.clone().into())
            .collect();
        Ok(posts)
    }

    async fn create_post(
        &mut self,
        tid: u64,
        post: String,
        file: Option<(String, File)>,
    ) -> Result<Post, String> {
        let param = grpc::CreatePostPrams {
            id: tid,
            text: post,
        };
        let post = self
            .client
            .create_post(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(post.into_inner().into())
    }

    async fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        let param = grpc::DeletePostParam { tid: tid, pid: pid };

        self.client
            .delete_post(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    async fn update_post(
        &mut self,
        tid: u64,
        post: Post,
    ) -> Result<Post, String> {
        let param = grpc::UpdatePostParam {
            tid,
            post: Some(post.into()),
        };

        let post = self
            .client
            .update_post(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(post.into_inner().into())
    }

    async fn get_latest_post(
        &mut self,
        tid: u64,
    ) -> Result<Option<Post>, String> {
        Err("unimplemented".to_string())
    }
}

impl std::fmt::Debug for GrpcStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GrpcStore")
    }
}
