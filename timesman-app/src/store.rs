// pub mod remote;
// pub mod sqlite3;
pub mod ram;

#[derive(Clone)]
pub struct Times {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

pub struct Post {
    pub id: i64,
    pub times_id: i64,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

pub trait Store {
    // for Times
    fn get_times(&self) -> Result<Vec<Times>, String>;
    fn create_times(&self, title: String) -> Result<Times, String>;
    fn delete_times(&self, tid: i64) -> Result<(), String>;
    fn update_times(&self, times: Times) -> Result<(), String>;

    // for Post
    fn get_posts(&self, tid: i64) -> Result<Vec<Post>, String>;
    fn create_post(&self, tid: i64, post: String) -> Result<Post, String>;
    fn delete_post(&self, tid: i64, pid: i64) -> Result<(), String>;
    fn update_post(&self, tid: i64, post: Post) -> Result<Post, String>;
}
