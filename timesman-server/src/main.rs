mod config;
mod http;

use std::sync::Arc;
use std::sync::Mutex;

use clap::Parser;
use store::sqlite3::SqliteStoreBuilder;

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

    let store = match &*config.store_type {
        "sqlite" => SqliteStoreBuilder::new(&config.store_param)
            .build()
            .await
            .unwrap(),
        _ => {
            tracing::error!("invalid config: store_type");
            return Ok(());
        }
    };

    let store = Arc::new(Mutex::new(store));

    let server: Box<dyn TimesManServer> = Box::new(http::HttpServer {});

    server.run(&config.listen, store).await;
    Ok(())
}
