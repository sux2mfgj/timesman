use super::{Post, Store, Times};
use async_trait::async_trait;
use chrono::Local;
use core::fmt;
use std::collections::HashMap;

struct LocalTimes {
    times: Times,
    posts: HashMap<u64, Post>,
    next_pid: u64,
}

pub struct RamStore {
    times: HashMap<u64, LocalTimes>,
    next_tid: u64,
}

impl RamStore {
    pub fn new() -> Self {
        Self {
            times: HashMap::new(),
            next_tid: 0,
        }
    }
}

#[async_trait]
impl Store for RamStore {
    async fn check(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn get_times(&mut self) -> Result<Vec<super::Times>, String> {
        Ok(self.times.iter().map(|t| t.1.times.clone()).collect())
    }

    async fn create_times(
        &mut self,
        title: String,
    ) -> Result<super::Times, String> {
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
            posts: HashMap::new(),
            next_pid: 0,
        };

        self.times.insert(id, ltimes);

        Ok(times)
    }

    async fn delete_times(&mut self, _tid: u64) -> Result<(), String> {
        unimplemented!();
    }

    async fn update_times(
        &mut self,
        times: super::Times,
    ) -> Result<Times, String> {
        if let Some(t) = self.times.get_mut(&times.id) {
            t.times = times;
            let now = Local::now();
            t.times.updated_at = Some(now.naive_local());
            Ok(t.times.clone())
        } else {
            return Err("times id is invalid".to_string());
        }
    }

    async fn get_posts(
        &mut self,
        tid: u64,
    ) -> Result<Vec<super::Post>, String> {
        let ltimes = self.times.get(&tid).ok_or("invalid tid")?;

        let mut pairs: Vec<(&u64, &Post)> = ltimes.posts.iter().collect();

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        let posts = pairs.iter().map(|x| x.1.clone()).collect();

        Ok(posts)
    }

    async fn create_post(
        &mut self,
        tid: u64,
        post: String,
    ) -> Result<super::Post, String> {
        let ltimes = self.times.get_mut(&tid).ok_or("invalid tid")?;

        let post = Post {
            id: ltimes.next_pid,
            post,
            created_at: Local::now().naive_local(),
            updated_at: None,
        };

        ltimes.posts.insert(post.id, post.clone());
        ltimes.next_pid += 1;

        Ok(post)
    }

    async fn update_post(
        &mut self,
        tid: u64,
        mut post: super::Post,
    ) -> Result<super::Post, String> {
        let times = match self.times.get_mut(&tid) {
            Some(t) => t,
            None => {
                return Err("Invalid tid".to_string());
            }
        };

        let oldpost = match times.posts.get_mut(&post.id) {
            Some(p) => p,
            None => return Err("Invalid pid".to_string()),
        };

        post.updated_at = Some(Local::now().naive_local());

        *oldpost = post.clone();

        Ok(post)
    }

    async fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        if let Some(times) = self.times.get_mut(&tid) {
            if let Some(_) = times.posts.remove(&pid) {
                Ok(())
            } else {
                Err("Invalid pid".to_string())
            }
        } else {
            Err("Invalid tid".to_string())
        }
    }

    async fn get_latest_post(
        &mut self,
        tid: u64,
    ) -> Result<Option<Post>, String> {
        if let Some(ltimes) = self.times.get(&tid) {
            let keys: Vec<u64> = ltimes.posts.clone().into_keys().collect();
            if let Some(latest_pid) = keys.iter().max() {
                if let Some(post) = ltimes.posts.get(latest_pid) {
                    return Ok(Some(post.clone()));
                }
            }
        }

        Ok(None)
    }
}

impl fmt::Debug for RamStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}
