use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MockClient {
    pub times: HashMap<u64, Times>,
    pub posts: HashMap<u64, Vec<Post>>,
    pub todos: HashMap<u64, Vec<Todo>>,
    pub next_times_id: u64,
    pub next_post_id: u64,
    pub next_todo_id: u64,
    pub should_error: bool,
    pub error_message: String,
}

impl MockClient {
    pub fn new() -> Self {
        Self {
            times: HashMap::new(),
            posts: HashMap::new(),
            todos: HashMap::new(),
            next_times_id: 1,
            next_post_id: 1,
            next_todo_id: 1,
            should_error: false,
            error_message: "Mock error".to_string(),
        }
    }

    pub fn with_error(mut self, error_msg: &str) -> Self {
        self.should_error = true;
        self.error_message = error_msg.to_string();
        self
    }

    pub fn with_sample_data(mut self) -> Self {
        let now = chrono::Utc::now().naive_utc();
        
        // Add sample times
        let times1 = Times {
            id: 1,
            title: "Test Project".to_string(),
            created_at: now,
            updated_at: None,
        };
        let times2 = Times {
            id: 2,
            title: "Another Project".to_string(),
            created_at: now,
            updated_at: Some(now),
        };
        
        self.times.insert(1, times1);
        self.times.insert(2, times2);
        
        // Add sample posts
        let post1 = Post {
            id: 1,
            post: "First post".to_string(),
            created_at: now,
            updated_at: None,
            file: None,
            tag: None,
        };
        let post2 = Post {
            id: 2,
            post: "Second post".to_string(),
            created_at: now,
            updated_at: Some(now),
            file: None,
            tag: Some(1),
        };
        
        self.posts.insert(1, vec![post1, post2]);
        self.posts.insert(2, vec![]);
        
        // Add sample todos
        let todo1 = Todo {
            id: 1,
            content: "Complete documentation".to_string(),
            detail: Some("Write comprehensive documentation for the todo detail feature including API endpoints and CLI usage examples.".to_string()),
            created_at: now,
            done_at: None,
        };
        let todo2 = Todo {
            id: 2,
            content: "Review code".to_string(),
            detail: None,
            created_at: now,
            done_at: Some(now),
        };
        let todo3 = Todo {
            id: 3,
            content: "Plan next sprint".to_string(),
            detail: Some("Organize upcoming tasks for the next development sprint, including priority assessment and resource allocation.".to_string()),
            created_at: now,
            done_at: None,
        };
        
        self.todos.insert(1, vec![todo1, todo2]);
        self.todos.insert(2, vec![todo3]);
        
        self.next_times_id = 3;
        self.next_post_id = 3;
        self.next_todo_id = 4;
        
        self
    }
}

impl Client for MockClient {
    fn get_times(&mut self) -> Result<Vec<Times>, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        let mut times: Vec<Times> = self.times.values().cloned().collect();
        times.sort_by_key(|t| t.id);
        Ok(times)
    }

    fn create_times(&mut self, title: String) -> Result<Times, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        let now = chrono::Utc::now().naive_utc();
        let times = Times {
            id: self.next_times_id,
            title,
            created_at: now,
            updated_at: None,
        };
        
        self.times.insert(self.next_times_id, times.clone());
        self.posts.insert(self.next_times_id, vec![]);
        self.todos.insert(self.next_times_id, vec![]);
        self.next_times_id += 1;
        
        Ok(times)
    }

    fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        self.times.remove(&tid);
        self.posts.remove(&tid);
        self.todos.remove(&tid);
        Ok(())
    }

    fn update_times(&mut self, times: Times) -> Result<Times, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&times.id) {
            return Err(format!("Times with ID {} not found", times.id));
        }
        
        let mut updated_times = times.clone();
        updated_times.updated_at = Some(chrono::Utc::now().naive_utc());
        
        self.times.insert(times.id, updated_times.clone());
        Ok(updated_times)
    }

    fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        Ok(self.posts.get(&tid).unwrap_or(&vec![]).clone())
    }

    fn create_post(&mut self, tid: u64, text: String) -> Result<Post, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let now = chrono::Utc::now().naive_utc();
        let post = Post {
            id: self.next_post_id,
            post: text,
            created_at: now,
            updated_at: None,
            file: None,
            tag: None,
        };
        
        self.posts.entry(tid).or_insert_with(Vec::new).push(post.clone());
        self.next_post_id += 1;
        
        Ok(post)
    }

    fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let posts = self.posts.entry(tid).or_insert_with(Vec::new);
        let initial_len = posts.len();
        posts.retain(|p| p.id != pid);
        
        if posts.len() == initial_len {
            return Err(format!("Post with ID {} not found in times {}", pid, tid));
        }
        
        Ok(())
    }

    fn update_post(&mut self, tid: u64, post: Post) -> Result<Post, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let posts = self.posts.entry(tid).or_insert_with(Vec::new);
        
        for existing_post in posts.iter_mut() {
            if existing_post.id == post.id {
                let mut updated_post = post.clone();
                updated_post.updated_at = Some(chrono::Utc::now().naive_utc());
                *existing_post = updated_post.clone();
                return Ok(updated_post);
            }
        }
        
        Err(format!("Post with ID {} not found in times {}", post.id, tid))
    }

    fn get_todos(&mut self, tid: u64) -> Result<Vec<Todo>, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        Ok(self.todos.get(&tid).unwrap_or(&vec![]).clone())
    }

    fn create_todo(&mut self, tid: u64, content: String) -> Result<Todo, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let now = chrono::Utc::now().naive_utc();
        let todo = Todo {
            id: self.next_todo_id,
            content,
            detail: None,
            created_at: now,
            done_at: None,
        };
        
        self.todos.entry(tid).or_insert_with(Vec::new).push(todo.clone());
        self.next_todo_id += 1;
        
        Ok(todo)
    }

    fn create_todo_with_detail(&mut self, tid: u64, content: String, detail: Option<String>) -> Result<Todo, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let now = chrono::Utc::now().naive_utc();
        let todo = Todo {
            id: self.next_todo_id,
            content,
            detail,
            created_at: now,
            done_at: None,
        };
        
        self.todos.entry(tid).or_insert_with(Vec::new).push(todo.clone());
        self.next_todo_id += 1;
        
        Ok(todo)
    }

    fn delete_todo(&mut self, tid: u64, tdid: u64) -> Result<(), String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let todos = self.todos.entry(tid).or_insert_with(Vec::new);
        let initial_len = todos.len();
        todos.retain(|t| t.id != tdid);
        
        if todos.len() == initial_len {
            return Err(format!("Todo with ID {} not found in times {}", tdid, tid));
        }
        
        Ok(())
    }

    fn update_todo(&mut self, tid: u64, todo: Todo) -> Result<Todo, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let todos = self.todos.entry(tid).or_insert_with(Vec::new);
        
        for existing_todo in todos.iter_mut() {
            if existing_todo.id == todo.id {
                let updated_todo = todo.clone();
                *existing_todo = updated_todo.clone();
                return Ok(updated_todo);
            }
        }
        
        Err(format!("Todo with ID {} not found in times {}", todo.id, tid))
    }

    fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let empty_vec = vec![];
        let todos = self.todos.get(&tid).unwrap_or(&empty_vec);
        
        for todo in todos {
            if todo.id == tdid {
                return Ok(todo.clone());
            }
        }
        
        Err(format!("Todo with ID {} not found in times {}", tdid, tid))
    }

    fn update_todo_detail(&mut self, tid: u64, tdid: u64, detail: String) -> Result<Todo, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let todos = self.todos.entry(tid).or_insert_with(Vec::new);
        
        for existing_todo in todos.iter_mut() {
            if existing_todo.id == tdid {
                existing_todo.detail = Some(detail);
                return Ok(existing_todo.clone());
            }
        }
        
        Err(format!("Todo with ID {} not found in times {}", tdid, tid))
    }

    fn mark_todo_done(&mut self, tid: u64, tdid: u64, done: bool) -> Result<Todo, String> {
        if self.should_error {
            return Err(self.error_message.clone());
        }
        
        if !self.times.contains_key(&tid) {
            return Err(format!("Times with ID {} not found", tid));
        }
        
        let todos = self.todos.entry(tid).or_insert_with(Vec::new);
        
        for existing_todo in todos.iter_mut() {
            if existing_todo.id == tdid {
                if done {
                    existing_todo.done_at = Some(chrono::Utc::now().naive_utc());
                } else {
                    existing_todo.done_at = None;
                }
                return Ok(existing_todo.clone());
            }
        }
        
        Err(format!("Todo with ID {} not found in times {}", tdid, tid))
    }
}