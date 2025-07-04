use chrono::Utc;
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use async_trait::async_trait;

use crate::{PostStore, TimesStore, TodoStore};

use super::Store;
use timesman_type::{File, Pid, Post, Tag, TagId, Tdid, Tid, Times, Todo};

pub struct RamStore {
    tstores: HashMap<Tid, Arc<Mutex<dyn TimesStore + Send + Sync>>>,
    ntid: Tid,
}

impl RamStore {
    pub fn new() -> Self {
        let tstores = HashMap::new();
        Self { tstores, ntid: 0 }
    }
}

#[async_trait]
impl Store for RamStore {
    async fn check(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn get(
        &mut self,
    ) -> Result<Vec<Arc<Mutex<dyn TimesStore + Send + Sync>>>, String> {
        let mut pairs: Vec<(&Tid, &Arc<Mutex<dyn TimesStore + Send + Sync>>)> =
            self.tstores.iter().collect();

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        let times = pairs.iter().map(|x| x.1.clone()).collect();
        Ok(times)
    }

    async fn create(
        &mut self,
        title: String,
    ) -> Result<Arc<Mutex<dyn TimesStore + Send + Sync>>, String> {
        let tid = self.ntid;

        let times = Times {
            id: tid,
            title,
            created_at: Utc::now().naive_local(),
            updated_at: None,
        };

        let tstore: Arc<Mutex<dyn TimesStore + Send + Sync>> =
            Arc::new(Mutex::new(RamTimesStore::new(times.clone())));

        self.ntid += 1;

        self.tstores.insert(tid, tstore.clone());

        Ok(tstore)
    }

    async fn delete(&mut self, tid: Tid) -> Result<(), String> {
        let r = self.tstores.remove(&tid);
        if r.is_some() {
            Ok(())
        } else {
            Err("invalid tid".to_string())
        }
    }
}

struct RamTimesStore {
    times: Times,
    pstore: Arc<Mutex<dyn PostStore + Send + Sync>>,
    tdstore: Arc<Mutex<dyn TodoStore + Send + Sync>>,
}

impl RamTimesStore {
    pub fn new(times: Times) -> Self {
        let pstore = Arc::new(Mutex::new(RamPostStore::new()));
        let tdstore = Arc::new(Mutex::new(RamTodoStore::new()));
        Self {
            times,
            pstore,
            tdstore,
        }
    }
}

#[async_trait]
impl TimesStore for RamTimesStore {
    async fn get(&mut self) -> Result<Times, String> {
        Ok(self.times.clone())
    }

    async fn update(&mut self, times: Times) -> Result<Times, String> {
        self.times = times.clone();
        Ok(times)
    }

    async fn pstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn PostStore + Send + Sync>>, String> {
        Ok(self.pstore.clone())
    }

    async fn tdstore(
        &mut self,
    ) -> Result<Arc<Mutex<dyn TodoStore + Send + Sync>>, String> {
        Ok(self.tdstore.clone())
    }
}

struct RamPostStore {
    posts: HashMap<Pid, Post>,
    npid: Pid,
    tags: HashMap<TagId, Tag>,
    ntagid: TagId,
}

impl RamPostStore {
    pub fn new() -> Self {
        let posts = HashMap::new();
        let tags = HashMap::new();
        Self {
            posts,
            npid: 0,
            tags,
            ntagid: 0,
        }
    }
}

#[async_trait]
impl PostStore for RamPostStore {
    async fn get(&mut self, pid: Pid) -> Result<Post, String> {
        if let Some(post) = self.posts.get(&pid) {
            Ok(post.clone())
        } else {
            Err("invalid pid".to_string())
        }
    }

    async fn get_all(&mut self) -> Result<Vec<Post>, String> {
        let mut pairs: Vec<(&Tid, &Post)> = self.posts.iter().collect();

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        let posts = pairs.iter().map(|x| x.1.clone()).collect();

        Ok(posts)
    }

    async fn get_tags(&mut self) -> Result<Vec<Tag>, String> {
        let mut pairs: Vec<(&TagId, &Tag)> = self.tags.iter().collect();

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        let tags = pairs.iter().map(|x| x.1.clone()).collect();

        Ok(tags)
    }

    async fn create_tag(&mut self, name: String) -> Result<Tag, String> {
        let id = self.ntagid;
        let tag = Tag { id, name };
        self.tags.insert(id, tag.clone());
        self.ntagid += 1;

        Ok(tag)
    }

    async fn post(
        &mut self,
        post: String,
        file: Option<File>,
    ) -> Result<Post, String> {
        let id = self.npid;
        self.npid += 1;

        let post = Post {
            id,
            post,
            created_at: Utc::now().naive_local(),
            updated_at: None,
            file,
            tag: None,
        };

        self.posts.insert(id, post.clone());

        Ok(post)
    }

    async fn delete(&mut self, pid: Pid) -> Result<(), String> {
        let r = self.posts.remove(&pid);
        if r.is_some() {
            Ok(())
        } else {
            Err("invalid pid".to_string())
        }
    }

    async fn update(&mut self, post: Post) -> Result<Post, String> {
        match self.posts.get_mut(&post.id) {
            Some(val) => {
                *val = post.clone();
                Ok(post)
            }
            None => Err("invalid pid".to_string()),
        }
    }
}

struct RamTodoStore {
    todos: HashMap<Tdid, Todo>,
    ntdid: Tdid,
}

impl RamTodoStore {
    pub fn new() -> Self {
        let todos = HashMap::new();
        Self { todos, ntdid: 0 }
    }
}

#[async_trait]
impl TodoStore for RamTodoStore {
    async fn get(&mut self) -> Result<Vec<Todo>, String> {
        let mut pairs: Vec<(&Tdid, &Todo)> = self.todos.iter().collect();

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        let todos = pairs.iter().map(|x| x.1.clone()).collect();

        Ok(todos)
    }

    async fn new(&mut self, content: String) -> Result<Todo, String> {
        let id = self.ntdid;
        self.ntdid += 1;

        let todo = Todo {
            id,
            content,
            detail: None,
            created_at: Utc::now().naive_local(),
            done_at: None,
        };

        self.todos.insert(id, todo.clone());

        Ok(todo)
    }

    async fn update(&mut self, todo: Todo) -> Result<Todo, String> {
        match self.todos.get_mut(&todo.id) {
            Some(val) => {
                *val = todo.clone();
                Ok(todo)
            }
            None => Err("invalid pid".to_string()),
        }
    }

    async fn done(&mut self, tdid: Tdid, done: bool) -> Result<Todo, String> {
        let Some(todo) = self.todos.get_mut(&tdid) else {
            return Err("invalid tdid".to_string());
        };

        if todo.done_at.is_some() == done {
            return Err("invalid state".to_string());
        }

        todo.done_at = if done {
            Some(chrono::Utc::now().naive_local())
        } else {
            None
        };

        let new = todo.clone();

        Ok(new)
    }

    async fn delete(&mut self, tdid: Tdid) -> Result<(), String> {
        if let Some(_) = self.todos.remove(&tdid) {
            Ok(())
        } else {
            Err("invalid tdid".to_string())
        }
    }
}
