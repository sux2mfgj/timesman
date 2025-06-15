use super::async_trait;
use super::TodoStore;
use super::{Arc, Mutex, UnQLite, KV};
use super::{Tdid, Tid, Todo};

use serde::{Deserialize, Serialize};

pub struct LocalTodoStore {
    tid: Tid,
    meta: TodoMeta,
    store: Arc<Mutex<UnQLite>>,
}

// {tid}/todos/meta.data
// {tid}/todos/{tdid}
fn get_meta_path(tid: Tid) -> String {
    format!("{tid}/todos/meta.data")
}

fn get_todo_path(tid: Tid, tdid: Tdid) -> String {
    format!("{tid}/todos/{tdid}")
}

#[derive(Serialize, Deserialize)]
struct TodoMeta {
    ntdid: Tdid,
    tdids: Vec<Tdid>,
}

async fn load_meta(
    tid: Tid,
    store: Arc<Mutex<UnQLite>>,
) -> Result<TodoMeta, String> {
    let store = store.lock().await;
    let meta_path = get_meta_path(tid);

    let meta = if !store.kv_contains(&meta_path) {
        let meta = TodoMeta {
            ntdid: 0,
            tdids: vec![],
        };

        let data = serde_json::to_string(&meta).unwrap();
        store.kv_store(&meta_path, data.into_bytes()).unwrap();

        meta
    } else {
        let data = store.kv_fetch(&meta_path).unwrap();
        serde_json::from_slice(&data).unwrap()
    };

    Ok(meta)
}

impl LocalTodoStore {
    pub async fn new(
        tid: Tid,
        store: Arc<Mutex<UnQLite>>,
    ) -> Result<Self, String> {
        let meta = load_meta(tid, store.clone()).await?;
        Ok(Self { tid, store, meta })
    }

    async fn sync_meta(&self) {
        let data = serde_json::to_string(&self.meta).unwrap();

        let store = self.store.lock().await;
        store
            .kv_store(get_meta_path(self.tid), data.into_bytes())
            .unwrap();
    }
}

#[async_trait]
impl TodoStore for LocalTodoStore {
    async fn get(&mut self) -> Result<Vec<Todo>, String> {
        let mut resp = vec![];
        let store = self.store.lock().await;
        for id in &self.meta.tdids {
            let data = store.kv_fetch(get_todo_path(self.tid, *id)).unwrap();
            let todo = serde_json::from_slice(&data).unwrap();
            resp.push(todo);
        }

        Ok(resp)
    }

    async fn new(&mut self, content: String) -> Result<Todo, String> {
        let id = self.meta.ntdid;

        let todo = Todo {
            id,
            content,
            created_at: chrono::Utc::now().naive_local(),
            done_at: None,
        };

        let text = serde_json::to_string(&todo).unwrap();
        {
            let store = self.store.lock().await;
            store
                .kv_store(get_todo_path(self.tid, id), text.into_bytes())
                .unwrap();
        }

        self.meta.ntdid += 1;
        self.meta.tdids.push(id);

        self.sync_meta().await;

        Ok(todo)
    }

    async fn done(&mut self, tdid: Tdid, done: bool) -> Result<Todo, String> {
        let store = self.store.lock().await;

        let Ok(data) = store.kv_fetch(get_todo_path(self.tid, tdid)) else {
            return Err("invalid tid".to_string());
        };
        let mut todo: Todo =
            serde_json::from_slice(&data).map_err(|e| format!("{e}"))?;

        if todo.done_at.is_some() == done {
            return Err("invalid state".to_string());
        }

        todo.done_at = if done {
            Some(chrono::Utc::now().naive_local())
        } else {
            None
        };

        let text = serde_json::to_string(&todo).unwrap();
        store
            .kv_store(get_todo_path(self.tid, tdid), text.into_bytes())
            .map_err(|e| format!("{e}"))?;

        Ok(todo)
    }

    async fn update(&mut self, todo: Todo) -> Result<Todo, String> {
        todo!()
    }

    async fn delete(&mut self, tdid: Tdid) -> Result<(), String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_path_consistency() {
        let tid = 123;
        let tdid = 456;

        // Test todo paths (should be plural and consistent)
        assert_eq!(get_meta_path(tid), "123/todos/meta.data");
        assert_eq!(get_todo_path(tid, tdid), "123/todos/456");
        
        // Verify consistent format
        assert!(get_meta_path(tid).ends_with("/meta.data"));
        assert!(get_todo_path(tid, tdid).starts_with(&format!("{tid}/todos/")));
    }
}
