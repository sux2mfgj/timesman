use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;
use unqlite::{UnQLite, KV};

use timesman_type::{File, Pid, Post, Tag, TagId, Tdid, Tid, Times, Todo};

use super::{PostStore, Store, TimesStore, TodoStore};

mod times;
use times::LocalTimesStore;

mod post;
use post::LocalPostStore;

mod todo;

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
        let store = UnQLite::create(&path);

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
 * Storage structure:
 * /meta.data                    - Root metadata
 * /{tid}/meta.data              - Times metadata  
 * /{tid}/posts/meta.data        - Posts metadata
 * /{tid}/posts/{pid}            - Individual posts
 * /{tid}/tags/meta.data         - Tags metadata
 * /{tid}/tags/{tagid}           - Individual tags
 * /{tid}/todos/meta.data        - Todos metadata
 * /{tid}/todos/{tdid}           - Individual todos
 */

#[derive(Serialize, Deserialize)]
struct TimesMeta {
    title: String,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl TimesMeta {
    pub fn new(title: String) -> Self {
        Self {
            title,
            created_at: chrono::Utc::now().naive_local(),
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

// /meta.data
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

        let tid = self.ntid;
        let tmeta = TimesMeta::new(title);
        let data = serde_json::to_string(&tmeta).map_err(|e| format!("{e}"))?;

        store
            .kv_store(format!("{}", tid), data.into_bytes())
            .map_err(|e| format!("{e}"))?;

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
        let data = serde_json::to_string(&rmeta).map_err(|e| format!("{e}"))?;
        store
            .kv_store("meta.data", data.into_bytes())
            .map_err(|e| format!("{e}"))?;

        let data = serde_json::to_string(&tmeta).unwrap();
        store
            .kv_store(format!("{}/meta.data", tid), data.into_bytes())
            .unwrap();

        Ok(tstore)
    }

    async fn delete(&mut self, tid: Tid) -> Result<(), String> {
        // Remove from tids list
        self.tids.retain(|&x| x != tid);
        
        // Update root metadata
        let root_meta = RootMeta {
            ntid: self.ntid,
            tids: self.tids.clone(),
        };
        let text = serde_json::to_string(&root_meta).map_err(|e| format!("Failed to serialize root meta: {}", e))?;
        
        let store = self.store.lock().await;
        store.kv_store("meta.data", text.into_bytes()).map_err(|e| format!("Failed to store root meta: {}", e))?;
        
        // Delete times metadata
        let meta_key = format!("{}/meta.data", tid);
        let _ = store.kv_delete(&meta_key); // Ignore error if doesn't exist
        
        // Delete posts directory
        let posts_meta_key = format!("{}/posts/meta.data", tid);
        let _ = store.kv_delete(&posts_meta_key); // Ignore error if doesn't exist
        
        // Delete tags directory  
        let tags_meta_key = format!("{}/tags/meta.data", tid);
        let _ = store.kv_delete(&tags_meta_key); // Ignore error if doesn't exist
        
        // Delete todos directory
        let todos_meta_key = format!("{}/todos/meta.data", tid);
        let _ = store.kv_delete(&todos_meta_key); // Ignore error if doesn't exist
        
        // Remove from tstores - collect IDs first, then filter
        let mut indices_to_remove = Vec::new();
        for (index, tstore) in self.tstores.iter().enumerate() {
            match tstore.lock().await.get().await {
                Ok(times) if times.id == tid => {
                    indices_to_remove.push(index);
                }
                Err(_) => {
                    // Remove corrupted stores too
                    indices_to_remove.push(index);
                }
                _ => {} // Keep stores with different IDs
            }
        }
        
        // Remove in reverse order to maintain correct indices
        for &index in indices_to_remove.iter().rev() {
            self.tstores.remove(index);
        }
        
        Ok(())
    }
}
