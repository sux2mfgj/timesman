use super::{Post, Store, Times};
use chrono::Local;
use std::collections::HashMap;

struct LocalTimes {
    times: Times,
    posts: Vec<Post>,
    next_pid: i64,
}

pub struct RamStore {
    times: HashMap<i64, LocalTimes>,
    next_tid: i64,
}

impl RamStore {
    pub fn new() -> Self {
        Self {
            times: HashMap::new(),
            next_tid: 0,
        }
    }
}

impl Store for RamStore {
    fn get_times(&self) -> Result<Vec<super::Times>, String> {
        Ok(self.times.iter().map(|t| t.1.times.clone()).collect())
    }

    fn create_times(&mut self, title: String) -> Result<super::Times, String> {
        let id = self.next_tid;
        self.next_tid += 1;

        let now = Local::now();

        let times = Times {
            id,
            title,
            created_at: now.naive_local(),
            updated_at: None,
        };

        let ltimes = LocalTimes {
            times: times.clone(),
            posts: vec![],
            next_pid: 0,
        };

        self.times.insert(id, ltimes);

        Ok(times)
    }

    fn delete_times(&mut self, _tid: i64) -> Result<(), String> {
        unimplemented!();
    }

    fn update_times(&mut self, _times: super::Times) -> Result<(), String> {
        unimplemented!();
    }

    fn get_posts(&self, tid: i64) -> Result<Vec<super::Post>, String> {
        let ltimes = self.times.get(&tid).ok_or("invalid tid")?;

        Ok(ltimes.posts.clone())
    }

    fn create_post(
        &mut self,
        tid: i64,
        post: String,
    ) -> Result<super::Post, String> {
        let ltimes = self.times.get_mut(&tid).ok_or("invalid tid")?;

        let post = Post {
            id: ltimes.next_pid,
            times_id: tid,
            post,
            created_at: Local::now().naive_local(),
            updated_at: None,
        };

        ltimes.posts.push(post.clone());

        Ok(post)
    }

    fn update_post(
        &mut self,
        _tid: i64,
        _post: super::Post,
    ) -> Result<super::Post, String> {
        unimplemented!();
    }

    fn delete_post(&mut self, _tid: i64, _pid: i64) -> Result<(), String> {
        unimplemented!();
    }
}
