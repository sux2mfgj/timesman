mod grpc;
mod tui;

use clap::{Parser, Subcommand};
use chrono;

use timesman_type::{File, FileType, Post, Times, Todo};
use std::fs;
use std::path::Path;

#[cfg(test)]
pub mod mock_client;
#[cfg(test)]
mod tests;

trait Client {
    fn get_times(&mut self) -> Result<Vec<Times>, String>;
    fn create_times(&mut self, title: String) -> Result<Times, String>;
    fn delete_times(&mut self, tid: u64) -> Result<(), String>;
    fn update_times(&mut self, times: Times) -> Result<Times, String>;

    fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String>;
    fn create_post(&mut self, tid: u64, text: String) -> Result<Post, String>;
    fn create_post_with_file(&mut self, tid: u64, text: String, file: File) -> Result<Post, String>;
    fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String>;
    fn update_post(&mut self, tid: u64, post: Post) -> Result<Post, String>;

    fn get_todos(&mut self, tid: u64) -> Result<Vec<Todo>, String>;
    fn create_todo(&mut self, tid: u64, content: String) -> Result<Todo, String>;
    fn create_todo_with_detail(&mut self, tid: u64, content: String, detail: Option<String>) -> Result<Todo, String>;
    fn delete_todo(&mut self, tid: u64, tdid: u64) -> Result<(), String>;
    fn update_todo(&mut self, tid: u64, todo: Todo) -> Result<Todo, String>;
    fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String>;
    fn update_todo_detail(&mut self, tid: u64, tdid: u64, detail: String) -> Result<Todo, String>;
    fn mark_todo_done(&mut self, tid: u64, tdid: u64, done: bool) -> Result<Todo, String>;
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required(true))]
    conn_type: String,
    #[arg(short, long)]
    server: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Interactive TUI (Text User Interface) mode
    Tui,
    GetTimesList,
    CreateTimes {
        #[arg(short, long)]
        title: String,
    },
    DeleteTimes {
        #[arg(short, long)]
        tid: u64,
    },
    UpdateTimes {
        #[arg(short, long)]
        tid: u64,
        #[arg(short = 'T', long)]
        title: String,
    },
    GetPostList {
        #[arg(short, long)]
        tid: u64,
    },
    CreatePost {
        #[arg(short, long)]
        tid: u64,
        #[arg(short = 'T', long)]
        text: String,
    },
    CreatePostWithFile {
        #[arg(short, long)]
        tid: u64,
        #[arg(short = 'T', long)]
        text: String,
        #[arg(short = 'f', long)]
        file_path: String,
    },
    DeletePost {
        #[arg(short, long)]
        tid: u64,
        #[arg(short, long)]
        pid: u64,
    },
    UpdatePost {
        #[arg(short, long)]
        tid: u64,
        #[arg(short, long)]
        pid: u64,
        #[arg(short = 'T', long)]
        text: String,
    },
    GetTodoList {
        #[arg(short, long)]
        tid: u64,
    },
    CreateTodo {
        #[arg(short, long)]
        tid: u64,
        #[arg(short = 'c', long)]
        content: String,
    },
    CreateTodoWithDetail {
        #[arg(short, long)]
        tid: u64,
        #[arg(short = 'c', long)]
        content: String,
        #[arg(short = 'd', long)]
        detail: String,
    },
    DeleteTodo {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
    },
    UpdateTodo {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
        #[arg(short = 'c', long)]
        content: String,
    },
    GetTodoDetail {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
    },
    UpdateTodoDetail {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
        #[arg(short = 'd', long)]
        detail: String,
    },
    MarkTodoDone {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
        #[arg(short = 'D', long, action = clap::ArgAction::SetTrue)]
        done: bool,
    },
    MarkTodoUndone {
        #[arg(short, long)]
        tid: u64,
        #[arg(long)]
        tdid: u64,
    },
}

fn list_times(times: Vec<Times>) {
    for t in times {
        println!("{}", t);
    }
}

fn list_posts(posts: Vec<Post>) {
    for p in posts {
        println!("ID: {}, Post: {}, Created: {}, Updated: {:?}, Tag: {:?}", 
                 p.id, p.post, p.created_at, p.updated_at, p.tag);
    }
}

fn list_todos(todos: Vec<Todo>) {
    for t in todos {
        let status = if t.done_at.is_some() { "DONE" } else { "PENDING" };
        let detail = if let Some(detail) = &t.detail {
            if detail.len() > 50 {
                format!(" - {}", &detail[..47]).to_string() + "..."
            } else {
                format!(" - {}", detail)
            }
        } else {
            String::new()
        };
        println!("ID: {}, Content: {}{}, Status: {}, Created: {}, Done: {:?}", 
                 t.id, t.content, detail, status, t.created_at, t.done_at);
    }
}

fn load_file_from_path(file_path: &str) -> Result<File, String> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(format!("File does not exist: {}", file_path));
    }
    
    let file_name = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let file_data = fs::read(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
    
    let file_type = if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "txt" | "md" | "json" | "toml" | "yaml" | "yml" | "rs" | "py" | "js" | "ts" | "html" | "css" | "xml" => {
                let text_content = String::from_utf8(file_data)
                    .map_err(|_| format!("File {} is not valid UTF-8 text", file_path))?;
                FileType::Text(text_content)
            }
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "ico" | "svg" => {
                FileType::Image(file_data)
            }
            _ => {
                FileType::Other(file_data)
            }
        }
    } else {
        if file_data.iter().all(|&b| b.is_ascii() && (b.is_ascii_graphic() || b.is_ascii_whitespace())) {
            let text_content = String::from_utf8(file_data)
                .map_err(|_| format!("File {} could not be interpreted as text", file_path))?;
            FileType::Text(text_content)
        } else {
            FileType::Other(file_data)
        }
    };
    
    Ok(File {
        name: file_name,
        ftype: file_type,
    })
}

fn run_command(mut c: Box<dyn Client>, cmd: &Command) -> Result<(), String> {
    match cmd {
        Command::Tui => {
            tui::run_tui(c)?;
        }
        Command::GetTimesList => {
            list_times(c.get_times()?);
        }
        Command::CreateTimes { title } => {
            let times = c.create_times(title.clone())?;
            println!("Created times: {}", times);
        }
        Command::DeleteTimes { tid } => {
            c.delete_times(*tid)?;
            println!("Deleted times with ID: {}", tid);
        }
        Command::UpdateTimes { tid, title } => {
            let times = Times {
                id: *tid,
                title: title.clone(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: Some(chrono::Utc::now().naive_utc()),
            };
            let updated_times = c.update_times(times)?;
            println!("Updated times: {}", updated_times);
        }
        Command::GetPostList { tid } => {
            list_posts(c.get_posts(*tid)?);
        }
        Command::CreatePost { tid, text } => {
            let post = c.create_post(*tid, text.clone())?;
            println!("Created post: ID {}, Text: {}", post.id, post.post);
        }
        Command::CreatePostWithFile { tid, text, file_path } => {
            let file = load_file_from_path(file_path)?;
            let post = c.create_post_with_file(*tid, text.clone(), file)?;
            println!("Created post with file: ID {}, Text: {}, File: {}", post.id, post.post, 
                     post.file.as_ref().map(|f| f.name.as_str()).unwrap_or("unknown"));
        }
        Command::DeletePost { tid, pid } => {
            c.delete_post(*tid, *pid)?;
            println!("Deleted post with ID: {} from times: {}", pid, tid);
        }
        Command::UpdatePost { tid, pid, text } => {
            let post = Post {
                id: *pid,
                post: text.clone(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: Some(chrono::Utc::now().naive_utc()),
                file: None,
                tag: None,
            };
            let updated_post = c.update_post(*tid, post)?;
            println!("Updated post: ID {}, Text: {}", updated_post.id, updated_post.post);
        }
        Command::GetTodoList { tid } => {
            list_todos(c.get_todos(*tid)?);
        }
        Command::CreateTodo { tid, content } => {
            let todo = c.create_todo(*tid, content.clone())?;
            println!("Created todo: ID {}, Content: {}", todo.id, todo.content);
        }
        Command::CreateTodoWithDetail { tid, content, detail } => {
            let todo = c.create_todo_with_detail(*tid, content.clone(), Some(detail.clone()))?;
            println!("Created todo with detail: ID {}, Content: {}, Detail: {}", 
                     todo.id, todo.content, detail);
        }
        Command::DeleteTodo { tid, tdid } => {
            c.delete_todo(*tid, *tdid)?;
            println!("Deleted todo with ID: {} from times: {}", tdid, tid);
        }
        Command::UpdateTodo { tid, tdid, content } => {
            let todo = Todo {
                id: *tdid,
                content: content.clone(),
                detail: None,
                created_at: chrono::Utc::now().naive_utc(),
                done_at: None,
            };
            let updated_todo = c.update_todo(*tid, todo)?;
            println!("Updated todo: ID {}, Content: {}", updated_todo.id, updated_todo.content);
        }
        Command::GetTodoDetail { tid, tdid } => {
            let todo = c.get_todo_detail(*tid, *tdid)?;
            println!("Todo ID: {}, Content: {}", todo.id, todo.content);
            if let Some(detail) = &todo.detail {
                println!("Detail: {}", detail);
            } else {
                println!("Detail: None");
            }
            println!("Created: {}, Done: {:?}", todo.created_at, todo.done_at);
        }
        Command::UpdateTodoDetail { tid, tdid, detail } => {
            let updated_todo = c.update_todo_detail(*tid, *tdid, detail.clone())?;
            println!("Updated todo detail: ID {}, Detail: {}", updated_todo.id, detail);
        }
        Command::MarkTodoDone { tid, tdid, done } => {
            let updated_todo = c.mark_todo_done(*tid, *tdid, *done)?;
            let status = if *done { "DONE" } else { "PENDING" };
            println!("Marked todo ID {} as {}", updated_todo.id, status);
        }
        Command::MarkTodoUndone { tid, tdid } => {
            let updated_todo = c.mark_todo_done(*tid, *tdid, false)?;
            println!("Marked todo ID {} as PENDING", updated_todo.id);
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let server = if let Some(server) = args.server {
        server
    } else {
        "http://127.0.0.1:8080/".to_string()
    };

    let client = match &*args.conn_type {
        "grpc" => Box::new(grpc::GrpcClient::new(&server)),
        _ => {
            unimplemented!();
        }
    };

    match run_command(client, &args.command) {
        Ok(()) => {
            println!("Success");
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
