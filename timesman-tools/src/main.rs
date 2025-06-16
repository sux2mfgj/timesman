mod grpc;

use clap::{Parser, Subcommand};
use chrono;

use timesman_type::{Post, Times};

#[cfg(test)]
mod tests;

trait Client {
    fn get_times(&mut self) -> Result<Vec<Times>, String>;
    fn create_times(&mut self, title: String) -> Result<Times, String>;
    fn delete_times(&mut self, tid: u64) -> Result<(), String>;
    fn update_times(&mut self, times: Times) -> Result<Times, String>;

    fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String>;
    fn create_post(&mut self, tid: u64, text: String) -> Result<Post, String>;
    fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String>;
    fn update_post(&mut self, tid: u64, post: Post) -> Result<Post, String>;
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

fn run_command(mut c: Box<dyn Client>, cmd: &Command) -> Result<(), String> {
    match cmd {
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
