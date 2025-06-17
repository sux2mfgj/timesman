use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::error;
use timesman_type::{Post, Times, Todo};

use crate::Client;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    TimesList,
    PostsList,
    TodosList,
    CreateTimes,
    EditTimes,
    CreatePost,
    EditPost,
    TodoDetail,
    EditTodoDetail,
    CreateTodoDetail,
    Help,
}

pub struct App {
    pub client: Box<dyn Client>,
    pub mode: AppMode,
    pub should_quit: bool,
    pub times_list: Vec<Times>,
    pub selected_times_index: usize,
    pub posts_list: Vec<Post>,
    pub selected_post_index: usize,
    pub todos_list: Vec<Todo>,
    pub selected_todo_index: usize,
    pub input: String,
    pub detail_input: String,
    pub detail_scroll_offset: usize,
    pub status_message: String,
    pub error_message: Option<String>,
    pub loading: bool,
}

impl App {
    pub fn new(client: Box<dyn Client>) -> Self {
        let mut app = Self {
            client,
            mode: AppMode::TimesList,
            should_quit: false,
            times_list: Vec::new(),
            selected_times_index: 0,
            posts_list: Vec::new(),
            selected_post_index: 0,
            todos_list: Vec::new(),
            selected_todo_index: 0,
            input: String::new(),
            detail_input: String::new(),
            detail_scroll_offset: 0,
            status_message: "Welcome to TimesMan TUI! Press 'h' for help, 'q' to quit".to_string(),
            error_message: None,
            loading: false,
        };
        
        // Load initial data
        if let Err(e) = app.refresh_times() {
            app.error_message = Some(format!("Failed to load times: {}", e));
        }
        
        app
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<bool, String> {
        // Handle mode-specific keys first for input modes
        match self.mode {
            AppMode::CreateTimes | AppMode::EditTimes | AppMode::CreatePost | AppMode::EditPost | 
            AppMode::CreateTodoDetail | AppMode::EditTodoDetail => {
                // In input modes, handle Escape and Ctrl+Q globally, but let input handler process other keys
                match key.code {
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    KeyCode::Esc => {
                        self.error_message = None;
                        match self.mode {
                            AppMode::CreateTimes | AppMode::EditTimes => {
                                self.mode = AppMode::TimesList;
                                self.input.clear();
                            }
                            AppMode::CreatePost | AppMode::EditPost => {
                                self.mode = AppMode::PostsList;
                                self.input.clear();
                            }
                            AppMode::CreateTodoDetail | AppMode::EditTodoDetail => {
                                self.mode = AppMode::TodosList;
                                self.input.clear();
                                self.detail_input.clear();
                            }
                            _ => {}
                        }
                        return Ok(false);
                    }
                    _ => {
                        // Let input handler process all other keys, including 'h'
                        return self.handle_input_keys(key);
                    }
                }
            }
            _ => {
                // For non-input modes, handle global keys first
                match key.code {
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    KeyCode::Char('h') => {
                        self.mode = AppMode::Help;
                        return Ok(false);
                    }
                    KeyCode::Esc => {
                        self.error_message = None;
                        match self.mode {
                            AppMode::Help => self.mode = AppMode::TimesList,
                            AppMode::PostsList => {
                                self.mode = AppMode::TimesList;
                            }
                            AppMode::TodosList => {
                                self.mode = AppMode::TimesList;
                            }
                            AppMode::TodoDetail => {
                                self.mode = AppMode::TodosList;
                            }
                            _ => {}
                        }
                        return Ok(false);
                    }
                    _ => {}
                }

                // Then handle mode-specific keys
                match self.mode {
                    AppMode::TimesList => self.handle_times_list_keys(key),
                    AppMode::PostsList => self.handle_posts_list_keys(key),
                    AppMode::TodosList => self.handle_todos_list_keys(key),
                    AppMode::TodoDetail => self.handle_todo_detail_keys(key),
                    AppMode::Help => self.handle_help_keys(key),
                    _ => Ok(false),
                }
            }
        }
    }

    fn handle_times_list_keys(&mut self, key: KeyEvent) -> Result<bool, String> {
        match key.code {
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('r') => {
                self.refresh_times()?;
                Ok(false)
            }
            KeyCode::Char('n') => {
                self.mode = AppMode::CreateTimes;
                self.input.clear();
                Ok(false)
            }
            KeyCode::Char('e') => {
                if !self.times_list.is_empty() {
                    self.mode = AppMode::EditTimes;
                    self.input = self.times_list[self.selected_times_index].title.clone();
                }
                Ok(false)
            }
            KeyCode::Char('d') => {
                self.delete_selected_times()?;
                Ok(false)
            }
            KeyCode::Enter => {
                if !self.times_list.is_empty() {
                    self.mode = AppMode::PostsList;
                    self.refresh_posts()?;
                }
                Ok(false)
            }
            KeyCode::Char('t') => {
                if !self.times_list.is_empty() {
                    self.mode = AppMode::TodosList;
                    self.refresh_todos()?;
                }
                Ok(false)
            }
            KeyCode::Up => {
                if !self.times_list.is_empty() && self.selected_times_index > 0 {
                    self.selected_times_index -= 1;
                }
                Ok(false)
            }
            KeyCode::Down => {
                if !self.times_list.is_empty() && self.selected_times_index < self.times_list.len() - 1 {
                    self.selected_times_index += 1;
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_posts_list_keys(&mut self, key: KeyEvent) -> Result<bool, String> {
        match key.code {
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('r') => {
                self.refresh_posts()?;
                Ok(false)
            }
            KeyCode::Char('n') => {
                self.mode = AppMode::CreatePost;
                self.input.clear();
                Ok(false)
            }
            KeyCode::Char('e') => {
                if !self.posts_list.is_empty() {
                    self.mode = AppMode::EditPost;
                    self.input = self.posts_list[self.selected_post_index].post.clone();
                }
                Ok(false)
            }
            KeyCode::Char('d') => {
                self.delete_selected_post()?;
                Ok(false)
            }
            KeyCode::Up => {
                if !self.posts_list.is_empty() && self.selected_post_index > 0 {
                    self.selected_post_index -= 1;
                }
                Ok(false)
            }
            KeyCode::Down => {
                if !self.posts_list.is_empty() && self.selected_post_index < self.posts_list.len() - 1 {
                    self.selected_post_index += 1;
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_input_keys(&mut self, key: KeyEvent) -> Result<bool, String> {
        match key.code {
            KeyCode::Enter => {
                match self.mode {
                    AppMode::CreateTimes => {
                        if !self.input.trim().is_empty() {
                            self.create_times()?;
                        }
                    }
                    AppMode::EditTimes => {
                        if !self.input.trim().is_empty() {
                            self.update_times()?;
                        }
                    }
                    AppMode::CreatePost => {
                        if !self.input.trim().is_empty() {
                            self.create_post()?;
                        }
                    }
                    AppMode::EditPost => {
                        if !self.input.trim().is_empty() {
                            self.update_post()?;
                        }
                    }
                    AppMode::CreateTodoDetail => {
                        // For todo detail mode, allow creation even if content is empty but detail is provided
                        if !self.input.trim().is_empty() || !self.detail_input.trim().is_empty() {
                            self.create_todo()?;
                        }
                    }
                    AppMode::EditTodoDetail => {
                        // Handle Ctrl+S for saving todo detail
                        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('s') {
                            self.update_todo()?;
                        } else {
                            // Add newline to detail input in multi-line mode
                            self.detail_input.push('\n');
                        }
                    }
                    _ => {}
                }
                Ok(false)
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+S to save in edit modes
                match self.mode {
                    AppMode::EditTodoDetail => {
                        self.update_todo()?;
                    }
                    _ => {}
                }
                Ok(false)
            }
            KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+X to cancel in edit modes
                match self.mode {
                    AppMode::EditTodoDetail => {
                        self.mode = AppMode::TodosList;
                        self.input.clear();
                        self.detail_input.clear();
                    }
                    _ => {}
                }
                Ok(false)
            }
            KeyCode::Up if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+Up to move between paragraphs in detail editing
                match self.mode {
                    AppMode::EditTodoDetail => {
                        // Move to previous paragraph (find previous double newline)
                        if let Some(pos) = self.detail_input[..self.detail_input.len().saturating_sub(1)]
                            .rfind("\n\n") {
                            // For now, just scroll up
                            if self.detail_scroll_offset > 0 {
                                self.detail_scroll_offset -= 1;
                            }
                        }
                    }
                    _ => {}
                }
                Ok(false)
            }
            KeyCode::Down if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+Down to move between paragraphs in detail editing
                match self.mode {
                    AppMode::EditTodoDetail => {
                        // Move to next paragraph (find next double newline)
                        if self.detail_input.contains("\n\n") {
                            // For now, just scroll down
                            self.detail_scroll_offset += 1;
                        }
                    }
                    _ => {}
                }
                Ok(false)
            }
            KeyCode::Tab => {
                // Tab to switch between content and detail input in todo modes
                match self.mode {
                    AppMode::CreateTodoDetail | AppMode::EditTodoDetail => {
                        // For now, just add tab character to current input
                        if self.mode == AppMode::CreateTodoDetail {
                            self.input.push('\t');
                        } else {
                            self.detail_input.push('\t');
                        }
                    }
                    _ => {}
                }
                Ok(false)
            }
            KeyCode::Char(c) => {
                match self.mode {
                    AppMode::CreateTodoDetail => {
                        self.input.push(c);
                    }
                    AppMode::EditTodoDetail => {
                        self.detail_input.push(c);
                    }
                    _ => {
                        self.input.push(c);
                    }
                }
                Ok(false)
            }
            KeyCode::Backspace => {
                match self.mode {
                    AppMode::CreateTodoDetail => {
                        self.input.pop();
                    }
                    AppMode::EditTodoDetail => {
                        self.detail_input.pop();
                    }
                    _ => {
                        self.input.pop();
                    }
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_help_keys(&mut self, key: KeyEvent) -> Result<bool, String> {
        match key.code {
            KeyCode::Char('q') => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn refresh_times(&mut self) -> Result<(), String> {
        self.loading = true;
        self.error_message = None;
        
        match self.client.get_times() {
            Ok(times) => {
                self.times_list = times;
                if self.selected_times_index >= self.times_list.len() && !self.times_list.is_empty() {
                    self.selected_times_index = self.times_list.len() - 1;
                } else if self.times_list.is_empty() {
                    self.selected_times_index = 0;
                }
                self.status_message = format!("Loaded {} times entries", self.times_list.len());
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to load times: {}", e));
            }
        }
        
        self.loading = false;
        Ok(())
    }

    pub fn refresh_posts(&mut self) -> Result<(), String> {
        if self.times_list.is_empty() {
            return Ok(());
        }

        self.loading = true;
        self.error_message = None;
        
        let times_id = self.times_list[self.selected_times_index].id;
        match self.client.get_posts(times_id) {
            Ok(posts) => {
                self.posts_list = posts;
                if self.selected_post_index >= self.posts_list.len() && !self.posts_list.is_empty() {
                    self.selected_post_index = self.posts_list.len() - 1;
                } else if self.posts_list.is_empty() {
                    self.selected_post_index = 0;
                }
                self.status_message = format!("Loaded {} posts", self.posts_list.len());
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to load posts: {}", e));
            }
        }
        
        self.loading = false;
        Ok(())
    }

    fn create_times(&mut self) -> Result<(), String> {
        match self.client.create_times(self.input.trim().to_string()) {
            Ok(_) => {
                self.status_message = format!("Created times: {}", self.input.trim());
                self.input.clear();
                self.mode = AppMode::TimesList;
                self.refresh_times()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to create times: {}", e));
            }
        }
        Ok(())
    }

    fn update_times(&mut self) -> Result<(), String> {
        if self.times_list.is_empty() {
            return Ok(());
        }

        let mut times = self.times_list[self.selected_times_index].clone();
        times.title = self.input.trim().to_string();
        
        match self.client.update_times(times) {
            Ok(_) => {
                self.status_message = format!("Updated times: {}", self.input.trim());
                self.input.clear();
                self.mode = AppMode::TimesList;
                self.refresh_times()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to update times: {}", e));
            }
        }
        Ok(())
    }

    fn delete_selected_times(&mut self) -> Result<(), String> {
        if self.times_list.is_empty() {
            return Ok(());
        }

        let times_id = self.times_list[self.selected_times_index].id;
        match self.client.delete_times(times_id) {
            Ok(_) => {
                self.status_message = format!("Deleted times with ID: {}", times_id);
                self.refresh_times()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to delete times: {}", e));
            }
        }
        Ok(())
    }

    fn create_post(&mut self) -> Result<(), String> {
        if self.times_list.is_empty() {
            return Ok(());
        }

        let times_id = self.times_list[self.selected_times_index].id;
        match self.client.create_post(times_id, self.input.trim().to_string()) {
            Ok(_) => {
                self.status_message = format!("Created post: {}", self.input.trim());
                self.input.clear();
                self.mode = AppMode::PostsList;
                self.refresh_posts()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to create post: {}", e));
            }
        }
        Ok(())
    }

    fn update_post(&mut self) -> Result<(), String> {
        if self.posts_list.is_empty() || self.times_list.is_empty() {
            return Ok(());
        }

        let times_id = self.times_list[self.selected_times_index].id;
        let mut post = self.posts_list[self.selected_post_index].clone();
        post.post = self.input.trim().to_string();
        
        match self.client.update_post(times_id, post) {
            Ok(_) => {
                self.status_message = format!("Updated post: {}", self.input.trim());
                self.input.clear();
                self.mode = AppMode::PostsList;
                self.refresh_posts()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to update post: {}", e));
            }
        }
        Ok(())
    }

    fn delete_selected_post(&mut self) -> Result<(), String> {
        if self.posts_list.is_empty() || self.times_list.is_empty() {
            return Ok(());
        }

        let times_id = self.times_list[self.selected_times_index].id;
        let post_id = self.posts_list[self.selected_post_index].id;
        
        match self.client.delete_post(times_id, post_id) {
            Ok(_) => {
                self.status_message = format!("Deleted post with ID: {}", post_id);
                self.refresh_posts()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to delete post: {}", e));
            }
        }
        Ok(())
    }

    pub fn get_selected_times(&self) -> Option<&Times> {
        if self.selected_times_index < self.times_list.len() {
            Some(&self.times_list[self.selected_times_index])
        } else {
            None
        }
    }

    pub fn get_selected_post(&self) -> Option<&Post> {
        if self.selected_post_index < self.posts_list.len() {
            Some(&self.posts_list[self.selected_post_index])
        } else {
            None
        }
    }

    pub fn get_selected_todo(&self) -> Option<&Todo> {
        if self.selected_todo_index < self.todos_list.len() {
            Some(&self.todos_list[self.selected_todo_index])
        } else {
            None
        }
    }

    pub fn refresh_todos(&mut self) -> Result<(), String> {
        if self.times_list.is_empty() {
            return Ok(());
        }

        self.loading = true;
        self.error_message = None;
        
        let times_id = self.times_list[self.selected_times_index].id;
        match self.client.get_todos(times_id) {
            Ok(todos) => {
                self.todos_list = todos;
                if self.selected_todo_index >= self.todos_list.len() && !self.todos_list.is_empty() {
                    self.selected_todo_index = self.todos_list.len() - 1;
                } else if self.todos_list.is_empty() {
                    self.selected_todo_index = 0;
                }
                self.status_message = format!("Loaded {} todos", self.todos_list.len());
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to load todos: {}", e));
            }
        }
        
        self.loading = false;
        Ok(())
    }

    fn handle_todos_list_keys(&mut self, key: KeyEvent) -> Result<bool, String> {
        match key.code {
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('r') => {
                self.refresh_todos()?;
                Ok(false)
            }
            KeyCode::Char('n') => {
                self.mode = AppMode::CreateTodoDetail;
                self.input.clear();
                self.detail_input.clear();
                Ok(false)
            }
            KeyCode::Char('e') => {
                if !self.todos_list.is_empty() {
                    self.mode = AppMode::EditTodoDetail;
                    let todo = &self.todos_list[self.selected_todo_index];
                    self.input = todo.content.clone();
                    self.detail_input = todo.detail.clone().unwrap_or_default();
                }
                Ok(false)
            }
            KeyCode::Char('d') => {
                if !self.todos_list.is_empty() {
                    self.mode = AppMode::TodoDetail;
                    self.detail_scroll_offset = 0;
                }
                Ok(false)
            }
            KeyCode::Delete => {
                self.delete_selected_todo()?;
                Ok(false)
            }
            KeyCode::Enter => {
                if !self.todos_list.is_empty() {
                    self.mode = AppMode::TodoDetail;
                    self.detail_scroll_offset = 0;
                }
                Ok(false)
            }
            KeyCode::Up => {
                if !self.todos_list.is_empty() && self.selected_todo_index > 0 {
                    self.selected_todo_index -= 1;
                }
                Ok(false)
            }
            KeyCode::Down => {
                if !self.todos_list.is_empty() && self.selected_todo_index < self.todos_list.len() - 1 {
                    self.selected_todo_index += 1;
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_todo_detail_keys(&mut self, key: KeyEvent) -> Result<bool, String> {
        match key.code {
            KeyCode::Char('q') => Ok(true),
            KeyCode::Esc => {
                self.mode = AppMode::TodosList;
                Ok(false)
            }
            KeyCode::Up => {
                if self.detail_scroll_offset > 0 {
                    self.detail_scroll_offset -= 1;
                }
                Ok(false)
            }
            KeyCode::Down => {
                self.detail_scroll_offset += 1;
                Ok(false)
            }
            KeyCode::PageUp => {
                self.detail_scroll_offset = self.detail_scroll_offset.saturating_sub(5);
                Ok(false)
            }
            KeyCode::PageDown => {
                self.detail_scroll_offset += 5;
                Ok(false)
            }
            KeyCode::Char('e') => {
                if !self.todos_list.is_empty() {
                    self.mode = AppMode::EditTodoDetail;
                    let todo = &self.todos_list[self.selected_todo_index];
                    self.input = todo.content.clone();
                    self.detail_input = todo.detail.clone().unwrap_or_default();
                }
                Ok(false)
            }
            KeyCode::Char('s') => {
                // Save current changes (if in edit mode - handled elsewhere)
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn create_todo(&mut self) -> Result<(), String> {
        if self.times_list.is_empty() {
            return Ok(());
        }

        let times_id = self.times_list[self.selected_times_index].id;
        let detail = if self.detail_input.trim().is_empty() {
            None
        } else {
            Some(self.detail_input.trim().to_string())
        };

        match self.client.create_todo_with_detail(times_id, self.input.trim().to_string(), detail) {
            Ok(_) => {
                self.status_message = format!("Created todo: {}", self.input.trim());
                self.input.clear();
                self.detail_input.clear();
                self.mode = AppMode::TodosList;
                self.refresh_todos()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to create todo: {}", e));
            }
        }
        Ok(())
    }

    fn update_todo(&mut self) -> Result<(), String> {
        if self.todos_list.is_empty() || self.times_list.is_empty() {
            return Ok(());
        }

        let times_id = self.times_list[self.selected_times_index].id;
        let todo_id = self.todos_list[self.selected_todo_index].id;
        let detail = if self.detail_input.trim().is_empty() {
            None
        } else {
            Some(self.detail_input.trim().to_string())
        };

        let todo = Todo {
            id: todo_id,
            content: self.input.trim().to_string(),
            detail,
            created_at: self.todos_list[self.selected_todo_index].created_at,
            done_at: self.todos_list[self.selected_todo_index].done_at,
        };

        match self.client.update_todo(times_id, todo) {
            Ok(_) => {
                self.status_message = format!("Updated todo: {}", self.input.trim());
                self.input.clear();
                self.detail_input.clear();
                self.mode = AppMode::TodosList;
                self.refresh_todos()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to update todo: {}", e));
            }
        }
        Ok(())
    }

    fn delete_selected_todo(&mut self) -> Result<(), String> {
        if self.todos_list.is_empty() || self.times_list.is_empty() {
            return Ok(());
        }

        let times_id = self.times_list[self.selected_times_index].id;
        let todo_id = self.todos_list[self.selected_todo_index].id;
        
        match self.client.delete_todo(times_id, todo_id) {
            Ok(_) => {
                self.status_message = format!("Deleted todo with ID: {}", todo_id);
                self.refresh_todos()?;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to delete todo: {}", e));
            }
        }
        Ok(())
    }
}