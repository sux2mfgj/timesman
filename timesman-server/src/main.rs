mod config;
//#[cfg(feature = "grpc")]
//mod grpc;
//mod http;

use config::FrontType;
use std::sync::Arc;
use tokio::sync::Mutex;

use clap::Parser;
use timesman_bstore::StoreType;
use timesman_server::TimesManServer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    let config = config::Config::load(args.config.into()).unwrap();

    let store = StoreType::to_store(&config.store_type).await.unwrap();

    let server: Box<dyn TimesManServer> = match config.front_type {
        FrontType::Grpc => {
            todo!();
        }
        FrontType::Http => {
            todo!();
        }
    };

    server.run(&config.listen, store).await;

    Ok(())
}
