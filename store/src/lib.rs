pub mod ram;
pub mod remote;
pub mod sqlite3;

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

pub trait Store {
    fn check(&self) -> Result<(), String>;

    // for Times
    fn get_times(&self) -> Result<Vec<Times>, String>;
    fn create_times(&mut self, title: String) -> Result<Times, String>;
    fn delete_times(&mut self, tid: i64) -> Result<(), String>;
    fn update_times(&mut self, times: Times) -> Result<(), String>;

    // for Post
    fn get_posts(&self, tid: i64) -> Result<Vec<Post>, String>;
    fn create_post(&mut self, tid: i64, post: String) -> Result<Post, String>;
    fn delete_post(&mut self, tid: i64, pid: i64) -> Result<(), String>;
    fn update_post(&mut self, tid: i64, post: Post) -> Result<Post, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
