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

    async fn sync_meta(&self) -> Result<(), String> {
        let data = serde_json::to_string(&self.meta)
            .map_err(|e| format!("Failed to serialize meta: {}", e))?;

        let store = self.store.lock().await;
        store
            .kv_store(get_meta_path(self.tid), data.into_bytes())
            .map_err(|e| format!("Failed to store meta: {}", e))?;
        
        Ok(())
    }
}

#[async_trait]
impl TodoStore for LocalTodoStore {
    async fn get(&mut self) -> Result<Vec<Todo>, String> {
        let mut resp = vec![];
        let store = self.store.lock().await;
        for id in &self.meta.tdids {
            let todo_path = get_todo_path(self.tid, *id);
            match store.kv_fetch(&todo_path) {
                Ok(data) => {
                    match serde_json::from_slice::<Todo>(&data) {
                        Ok(todo) => resp.push(todo),
                        Err(e) => {
                            eprintln!("Warning: Failed to deserialize todo {}: {}", id, e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to fetch todo {}: {}", id, e);
                    continue;
                }
            }
        }

        Ok(resp)
    }

    async fn new(&mut self, content: String) -> Result<Todo, String> {
        let id = self.meta.ntdid;

        let todo = Todo {
            id,
            content,
            detail: None,
            created_at: chrono::Utc::now().naive_local(),
            done_at: None,
        };

        let text = serde_json::to_string(&todo)
            .map_err(|e| format!("Failed to serialize todo: {}", e))?;
        
        {
            let store = self.store.lock().await;
            store
                .kv_store(get_todo_path(self.tid, id), text.into_bytes())
                .map_err(|e| format!("Failed to store todo: {}", e))?;
        }

        self.meta.ntdid += 1;
        self.meta.tdids.push(id);

        self.sync_meta().await?;

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
        let store = self.store.lock().await;
        
        // Check if todo exists
        let todo_path = get_todo_path(self.tid, todo.id);
        if !store.kv_contains(&todo_path) {
            return Err("Todo not found".to_string());
        }
        
        // Update the todo
        let text = serde_json::to_string(&todo)
            .map_err(|e| format!("Failed to serialize todo: {}", e))?;
        
        store
            .kv_store(&todo_path, text.into_bytes())
            .map_err(|e| format!("Failed to store todo: {}", e))?;
        
        Ok(todo)
    }

    async fn delete(&mut self, tdid: Tdid) -> Result<(), String> {
        let store = self.store.lock().await;
        
        // Check if todo exists and remove from metadata
        if let Some(pos) = self.meta.tdids.iter().position(|&x| x == tdid) {
            self.meta.tdids.remove(pos);
        } else {
            return Err("Todo not found".to_string());
        }
        
        // Delete the todo from storage
        let todo_path = get_todo_path(self.tid, tdid);
        store
            .kv_delete(&todo_path)
            .map_err(|e| format!("Failed to delete todo: {}", e))?;
        
        // Update metadata
        self.sync_meta().await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    async fn create_test_store() -> (LocalTodoStore, String) {
        let test_db = format!("{}/test_todos_{}.db", env::temp_dir().display(), uuid::Uuid::new_v4());
        let store = Arc::new(Mutex::new(UnQLite::create(&test_db)));
        let todo_store = LocalTodoStore::new(1, store).await.unwrap();
        (todo_store, test_db)
    }

    fn cleanup_test_db(path: &str) {
        let _ = fs::remove_file(path);
    }

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

    #[tokio::test]
    async fn test_todo_crud_operations() {
        let (mut todo_store, test_db) = create_test_store().await;

        // Test creating a new todo
        let todo = todo_store.new("Test todo".to_string()).await.unwrap();
        assert_eq!(todo.content, "Test todo");
        assert_eq!(todo.id, 0);
        assert!(todo.detail.is_none());
        assert!(todo.done_at.is_none());

        // Test getting all todos
        let todos = todo_store.get().await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].content, "Test todo");

        // Test updating a todo
        let mut updated_todo = todo.clone();
        updated_todo.detail = Some("Detailed description".to_string());
        let result = todo_store.update(updated_todo).await.unwrap();
        assert_eq!(result.detail, Some("Detailed description".to_string()));

        // Test marking todo as done
        let done_todo = todo_store.done(todo.id, true).await.unwrap();
        assert!(done_todo.done_at.is_some());

        // Test marking todo as undone
        let undone_todo = todo_store.done(todo.id, false).await.unwrap();
        assert!(undone_todo.done_at.is_none());

        // Test deleting a todo
        todo_store.delete(todo.id).await.unwrap();
        let todos = todo_store.get().await.unwrap();
        assert_eq!(todos.len(), 0);

        cleanup_test_db(&test_db);
    }

    #[tokio::test]
    async fn test_todo_error_cases() {
        let (mut todo_store, test_db) = create_test_store().await;

        // Test updating non-existent todo
        let fake_todo = Todo {
            id: 999,
            content: "Fake".to_string(),
            detail: None,
            created_at: chrono::Utc::now().naive_local(),
            done_at: None,
        };
        assert!(todo_store.update(fake_todo).await.is_err());

        // Test deleting non-existent todo
        assert!(todo_store.delete(999).await.is_err());

        // Test invalid state transition
        let todo = todo_store.new("Test".to_string()).await.unwrap();
        
        // Try to mark as done twice
        todo_store.done(todo.id, true).await.unwrap();
        assert!(todo_store.done(todo.id, true).await.is_err());

        cleanup_test_db(&test_db);
    }
}
