mod config;
#[cfg(feature = "grpc")]
mod grpc;
mod http;

use std::sync::Arc;
use tokio::sync::Mutex;

use clap::Parser;
#[cfg(feature = "sqlite")]
use timesman_bstore::sqlite::SqliteStoreBuilder;
use timesman_bstore::Store;
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

    let store: Box<dyn Store + Send + Sync + 'static> = Box::new(store);
    let store = Arc::new(Mutex::new(store));

    let server = match &*config.front_type {
        "grpc" => {
            let grpc_srv: Box<dyn TimesManServer> =
                Box::new(grpc::GrpcServer {});
            grpc_srv
        }
        "http" => {
            let http_srv: Box<dyn TimesManServer> =
                Box::new(http::HttpServer {});
            http_srv
        }
        _ => {
            panic!("unsupported server type. See the parameter of config.front_type");
        }
    };

    server.run(&config.listen, store).await;

    Ok(())
}
