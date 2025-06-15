mod config;
//#[cfg(feature = "grpc")]
//mod grpc;
//mod http;

use config::FrontType;

use clap::Parser;
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

    let store_type = config.store.to_store_type().unwrap();
    let store = store_type.to_store().await.unwrap();

    let server: Box<dyn TimesManServer> = match config.front_type {
        FrontType::Grpc => {
            #[cfg(feature = "grpc")]
            {
                Box::new(timesman_server::GrpcServer {})
            }
            #[cfg(not(feature = "grpc"))]
            {
                panic!("gRPC feature not enabled");
            }
        }
        FrontType::Http => {
            panic!("HTTP server not implemented");
        }
    };

    server.run(&config.listen, store).await;

    Ok(())
}
