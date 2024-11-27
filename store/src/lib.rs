pub mod ram;
// pub mod remote;
// pub mod sqlite3;

use async_trait::async_trait;
use chrono;

#[derive(Clone)]
pub struct Times {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone)]
pub struct Post {
    pub id: i64,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[async_trait]
pub trait Store: Send + Sync + 'static {
    fn check(&self) -> Result<(), String>;

    // for Times
    async fn get_times(&self) -> Result<Vec<Times>, String>;
    async fn create_times(&mut self, title: String) -> Result<Times, String>;
    async fn delete_times(&mut self, tid: i64) -> Result<(), String>;
    async fn update_times(&mut self, times: Times) -> Result<(), String>;

    // for Post
    async fn get_posts(&self, tid: i64) -> Result<Vec<Post>, String>;
    async fn create_post(
        &mut self,
        tid: i64,
        post: String,
    ) -> Result<Post, String>;
    async fn delete_post(&mut self, tid: i64, pid: i64) -> Result<(), String>;
    async fn update_post(
        &mut self,
        tid: i64,
        post: Post,
    ) -> Result<Post, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
