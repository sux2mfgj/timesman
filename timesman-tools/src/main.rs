mod grpc;

use clap::{Parser, Subcommand};

use timesman_bstore::{Post, Times};

trait Client {
    fn get_times(&mut self) -> Result<Vec<Times>, String>;
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
    GetList,
}

fn list_times(times: Vec<Times>) {
    for t in times {
        println!("{}", t.id);
    }
}

fn run_command(mut c: Box<dyn Client>, cmd: &Command) -> Result<(), String> {
    match cmd {
        Command::GetList => {
            list_times(c.get_times()?);
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
