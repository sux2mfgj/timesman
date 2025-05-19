use super::serde_json;
use super::PostStore;
use super::{async_trait, Arc, Mutex, UnQLite, KV};
use super::{File, Pid, Post, Tid};
use serde::{Deserialize, Serialize};

pub struct LocalPostStore {
    tid: Tid,
    npid: Pid,
    pids: Vec<Pid>,
    store: Arc<Mutex<UnQLite>>,
}

fn get_pmeta_path(tid: Tid) -> String {
    format!("{}/posts/meta.data", tid)
}

impl LocalPostStore {
    pub async fn new(tid: Tid, store: Arc<Mutex<UnQLite>>) -> Self {
        let pmeta = {
            let store = store.lock().await;
            let meta_path = format!("{}/posts/meta.data", tid);
            if !store.kv_contains(&meta_path) {
                let meta = PostMeta {
                    npid: 0,
                    pids: vec![],
                };
                let data = serde_json::to_string(&meta).unwrap();
                store.kv_store(&meta_path, data.into_bytes()).unwrap();
                meta
            } else {
                let data = store.kv_fetch(&meta_path).unwrap();
                serde_json::from_slice(&data).unwrap()
            }
        };

        Self {
            tid,
            npid: pmeta.npid,
            pids: pmeta.pids,
            store,
        }
    }

    async fn sync_meta(&self) {
        let pmeta = PostMeta {
            npid: self.npid,
            pids: self.pids.clone(),
        };
        let data = serde_json::to_string(&pmeta).unwrap();

        let store = self.store.lock().await;
        store
            .kv_store(get_pmeta_path(self.tid), data.into_bytes())
            .unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PostMeta {
    npid: Pid,
    pids: Vec<Pid>,
}

#[async_trait]
impl PostStore for LocalPostStore {
    async fn get(&mut self, _pid: Pid) -> Result<Post, String> {
        todo!();
    }

    async fn get_all(&mut self) -> Result<Vec<Post>, String> {
        let store = self.store.lock().await;
        let mut posts = vec![];
        for pid in &self.pids {
            let data = store
                .kv_fetch(format!("{}/posts/{}", self.tid, pid))
                .unwrap();
            let post: Post = serde_json::from_slice(&data).unwrap();
            posts.push(post);
        }

        Ok(posts)
    }

    async fn post(
        &mut self,
        post: String,
        file: Option<File>,
    ) -> Result<Post, String> {
        let pid = self.npid;

        let post = Post {
            id: pid,
            post,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: None,
            file,
        };

        let text = serde_json::to_string(&post).unwrap();
        // add a scope to avoid deadlock
        {
            let store = self.store.lock().await;
            store
                .kv_store(
                    format!("{}/posts/{}", self.tid, pid),
                    text.into_bytes(),
                )
                .unwrap();
        }

        self.pids.push(pid);
        self.npid += 1;

        self.sync_meta().await;

        Ok(post)
    }

    async fn delete(&mut self, _pid: Pid) -> Result<(), String> {
        todo!();
    }

    async fn update(&mut self, _post: Post) -> Result<Post, String> {
        todo!();
    }
}
