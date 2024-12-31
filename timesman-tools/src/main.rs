mod grpc;

use clap::{Parser, Subcommand};

use timesman_type::{Post, Times};

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
    CreateTimes,
    DeleteTimes,
    UpdateTimes,
    GetPostList,
    CreatePost,
    DeletePost,
    UpdatePost,
}

fn list_times(times: Vec<Times>) {
    for t in times {
        println!("{}", t);
    }
}

fn run_command(mut c: Box<dyn Client>, cmd: &Command) -> Result<(), String> {
    match cmd {
        Command::GetTimesList => {
            list_times(c.get_times()?);
        }
        Command::CreateTimes => {
            unimplemented!();
            // c.create_times()?;
        }
        Command::DeleteTimes => {
            unimplemented!();
            // c.delete_times()?;
        }
        Command::UpdateTimes => {
            unimplemented!();
            // c.update_times()?;
        }
        Command::GetPostList => {
            unimplemented!();
            // c.get_posts()?;
        }
        Command::CreatePost => {
            unimplemented!();
            // c.create_post()?;
        }
        Command::DeletePost => {
            unimplemented!();
            // c.delete_post()?;
        }
        Command::UpdatePost => {
            unimplemented!();
            // c.update_post()?;
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
