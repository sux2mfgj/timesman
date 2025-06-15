use super::{async_trait, Arc, Mutex};
use super::{GrpcClient, PostStore, TimesStore, TodoStore};
use timesman_type::{File, Pid, Post, Tag, Tid, Times};
use tonic;
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
        // Get all posts and find the one with matching pid
        let posts = self.get_all().await?;
        posts.into_iter()
            .find(|p| p.id == pid)
            .ok_or_else(|| format!("Post with id {} not found", pid))
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
        _file: Option<File>,
    ) -> Result<Post, String> {
        let param = grpc::CreatePostPrams {
            id: self.tid,
            text: post,
        };
        let mut c = self.client.lock().await;
        let created_post = c
            .create_post(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(created_post.into_inner().into())
    }

    async fn delete(&mut self, pid: Pid) -> Result<(), String> {
        let param = grpc::DeletePostParam {
            tid: self.tid,
            pid,
        };
        let mut c = self.client.lock().await;
        c.delete_post(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    async fn update(&mut self, post: Post) -> Result<Post, String> {
        let param = grpc::UpdatePostParam {
            tid: self.tid,
            post: Some(post.into()),
        };
        let mut c = self.client.lock().await;
        let updated_post = c
            .update_post(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(updated_post.into_inner().into())
    }
}
