use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;
use unqlite::{UnQLite, KV};

use timesman_type::{File, Pid, Post, Tdid, Tid, Times, Todo};

use super::{PostStore, Store, TimesStore, TodoStore};

#[derive(Serialize, Deserialize)]
struct RootMeta {
    ntid: u64,
    tids: Vec<Tid>,
}

pub struct LocalStore {
    store: Arc<Mutex<UnQLite>>,
    tids: Vec<Tid>,
    ntid: u64,
    tstores: Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>>,
}

impl LocalStore {
    pub async fn new(path: &str) -> Self {
        let store = UnQLite::create(path);

        let meta = if !store.kv_contains("meta.data") {
            let meta = RootMeta {
                ntid: 0,
                tids: vec![],
            };
            let text = serde_json::to_string(&meta).unwrap();
            store.kv_store("meta.data", text.into_bytes()).unwrap();
            meta
        } else {
            let data = store.kv_fetch("meta.data").unwrap();
            serde_json::from_slice(&data).unwrap()
        };

        let storep = Arc::new(Mutex::new(store));

        let mut tstores = vec![];

        for tid in &meta.tids {
            let store = storep.lock().await;
            let data = store.kv_fetch(format!("{}/meta.data", tid)).unwrap();
            let tmeta: TimesMeta = serde_json::from_slice(&data).unwrap();

            let tstore: Arc<Mutex<dyn TimesStore + Send + Sync>> =
                Arc::new(Mutex::new(LocalTimesStore::new(
                    tmeta.to_times(*tid),
                    storep.clone(),
                )));

            tstores.push(tstore);
        }

        Self {
            store: storep,
            tids: meta.tids,
            ntid: meta.ntid,
            tstores,
        }
    }
}

/*
 * /meta.data
 * /{tid}/meta.data
 * /{tid}/posts/{posts}
 * /{tid}/todo/{todos}
 */

#[derive(Serialize, Deserialize)]
struct TimesMeta {
    // pids: Vec<Pid>,
    // tdids: Vec<Tdid>,
    title: String,
    // npid: u64,
    // ntdid: u64,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl TimesMeta {
    pub fn new(title: String) -> Self {
        Self {
            // pids: vec![],
            // tdids: vec![],
            title,
            // npid: 0,
            // ntdid: 0,
            created_at: chrono::Local::now().naive_local(),
            updated_at: None,
        }
    }

    fn to_times(&self, tid: Tid) -> Times {
        Times {
            id: tid,
            title: self.title.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[async_trait]
impl Store for LocalStore {
    async fn check(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn get(
        &mut self,
    ) -> Result<Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>>, String> {
        Ok(self.tstores.clone())
    }

    async fn create(
        &mut self,
        title: String,
    ) -> Result<Arc<Mutex<dyn TimesStore + Send + Sync>>, String> {
        let store = self.store.lock().await;
        if store.kv_contains(&title) {
            return Err("Already exists".to_string());
        }

        let tmeta = TimesMeta::new(title);
        let tid = self.ntid;
        let data = serde_json::to_string(&tmeta).unwrap();

        store
            .kv_store(format!("{}", tid), data.into_bytes())
            .unwrap();

        let tstore = Arc::new(Mutex::new(LocalTimesStore::new(
            tmeta.to_times(tid),
            self.store.clone(),
        )));
        self.tstores.push(tstore.clone());

        self.ntid += 1;
        self.tids.push(tid);

        let rmeta = RootMeta {
            ntid: self.ntid,
            tids: self.tids.clone(),
        };
        let data = serde_json::to_string(&rmeta).unwrap();
        store.kv_store("meta.data", data.into_bytes()).unwrap();

        let data = serde_json::to_string(&tmeta).unwrap();
        store
            .kv_store(format!("{}/meta.data", tid), data.into_bytes())
            .unwrap();

        Ok(tstore)
    }

    async fn delete(&mut self, tid: Tid) -> Result<(), String> {
        todo!();
    }
}

struct LocalTimesStore {
    times: Times,
    store: Arc<Mutex<UnQLite>>,
}

impl LocalTimesStore {
    pub fn new(times: Times, store: Arc<Mutex<UnQLite>>) -> Self {
        Self { times, store }
    }
}

#[async_trait]
impl TimesStore for LocalTimesStore {
    async fn get(&mut self) -> Result<Times, String> {
        Ok(self.times.clone())
    }

    async fn update(&mut self, times: Times) -> Result<Times, String> {
        self.times = times.clone();
        todo!("update /times/_tid_/meta.data");
        Ok(times)
    }

    async fn pstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn PostStore + Send + Sync>>, String> {
        let pstore: Arc<Mutex<dyn PostStore + Send + Sync>> =
            Arc::new(Mutex::new(
                LocalPostStore::new(self.times.id, self.store.clone()).await,
            ));

        Ok(pstore)
    }

    async fn tdstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn TodoStore + Send + Sync>>, String> {
        todo!();
    }
}

struct LocalPostStore {
    tid: Tid,
    npid: Pid,
    pids: Vec<Pid>,
    store: Arc<Mutex<UnQLite>>,
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
}

#[derive(Serialize, Deserialize)]
struct PostMeta {
    npid: Pid,
    pids: Vec<Pid>,
}

#[async_trait]
impl PostStore for LocalPostStore {
    async fn get(&mut self, pid: Pid) -> Result<Post, String> {
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
        file: Option<(String, File)>,
    ) -> Result<Post, String> {
        let store = self.store.lock().await;

        if let Some(file) = &file {
            todo!();
        }

        let pid = self.npid;

        let post = Post {
            id: pid,
            post,
            created_at: chrono::Local::now().naive_local(),
            updated_at: None,
            file,
        };

        let text = serde_json::to_string(&post).unwrap();
        store
            .kv_store(format!("{}/posts/{}", self.tid, pid), text.into_bytes())
            .unwrap();

        self.npid += 1;

        Ok(post)
    }

    async fn delete(&mut self, pid: Pid) -> Result<(), String> {
        todo!();
    }

    async fn update(&mut self, post: Post) -> Result<Post, String> {
        todo!();
    }
}
