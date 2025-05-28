use super::{async_trait, Arc, Mutex};
use super::{GrpcClient, PostStore, TimesStore, TodoStore};
use timesman_type::{File, Pid, Post, Tag, Tid, Times};

use timesman_grpc::grpc;

pub(crate) struct GrpcPostStore {
    client: GrpcClient,
    tid: Tid,
}

impl GrpcPostStore {
    pub fn new(client: GrpcClient, tid: Tid) -> Self {
        Self { client, tid }
    }
}

#[async_trait]
impl PostStore for GrpcPostStore {
    async fn get(&mut self, pid: Pid) -> Result<Post, String> {
        todo!();
    }

    async fn get_all(&mut self) -> Result<Vec<Post>, String> {
        let tid = grpc::TimesId { id: self.tid };
        let mut c = self.client.lock().await;
        let posts = c
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

    async fn get_tags(&mut self) -> Result<Vec<Tag>, String> {
        todo!();
    }

    async fn create_tag(&mut self, name: String) -> Result<Tag, String> {
        todo!();
    }

    async fn post(
        &mut self,
        post: String,
        file: Option<File>,
    ) -> Result<Post, String> {
        todo!();
    }

    async fn delete(&mut self, pid: Pid) -> Result<(), String> {
        todo!();
    }

    async fn update(&mut self, post: Post) -> Result<Post, String> {
        todo!();
    }
}
