use super::{Post, Store, Times};
use chrono::Local;

struct LocalTimes {
    times: Times,
    posts: Vec<Post>,
}

pub struct RamStore {
    times: Vec<LocalTimes>,
    next_tid: i64,
}

impl RamStore {
    pub fn new() -> Self {
        Self {
            times: vec![],
            next_tid: 0,
        }
    }
}

impl Store for RamStore {
    fn get_times(&self) -> Result<Vec<super::Times>, String> {
        Ok(self.times.iter().map(|t| t.times.clone()).collect())
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
        };
        self.times.push(ltimes);

        Ok(times)
    }

    fn delete_times(&self, tid: i64) -> Result<(), String> {
        unimplemented!();
    }

    fn update_times(&self, times: super::Times) -> Result<(), String> {
        unimplemented!();
    }

    fn get_posts(&self, tid: i64) -> Result<Vec<super::Post>, String> {
        unimplemented!();
    }

    fn create_post(
        &self,
        tid: i64,
        post: String,
    ) -> Result<super::Post, String> {
        unimplemented!();
    }

    fn update_post(
        &self,
        tid: i64,
        post: super::Post,
    ) -> Result<super::Post, String> {
        unimplemented!();
    }

    fn delete_post(&self, tid: i64, pid: i64) -> Result<(), String> {
        unimplemented!();
    }
}
