mod config;
//#[cfg(feature = "grpc")]
//mod grpc;
//mod http;

use std::sync::Arc;
use tokio::sync::Mutex;

use clap::Parser;
//#[cfg(feature = "sqlite")]
//use timesman_bstore::SqliteStore;
use timesman_bstore::TimesStore;
use timesman_server::TimesManServer;

use timesman_bstore::RamStore;

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
        //#[cfg(feature = "sqlite")]
        //"sqlite" => SqliteStore::new(&config.store_param, true).await,
        _ => RamStore::new(),
    };

    let store = Arc::new(Mutex::new(store));

    let server: Box<dyn TimesManServer> = match &*config.front_type {
        "default" => {
            todo!();
        }
        //#[cfg(feature = "grpc")]
        //"grpc" => {
        //    let grpc_srv: Box<dyn TimesManServer> =
        //        Box::new(grpc::GrpcServer {});
        //    grpc_srv
        //}
        //"http" => {
        //    let http_srv: Box<dyn TimesManServer> =
        //        Box::new(http::HttpServer {});
        //    http_srv
        //}
        _ => {
            panic!("unsupported server type. See the parameter of config.front_type");
        }
    };

    server.run(&config.listen, store, None).await;

    Ok(())
}
