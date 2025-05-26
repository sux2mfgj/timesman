//pub mod http;

use std::sync::Arc;
use tokio::sync::Mutex;

//#[cfg(feature = "grpc")]
//mod grpc;
//#[cfg(feature = "grpc")]
//pub use grpc::GrpcServer;

use async_trait::async_trait;

use timesman_bstore::Store;

#[async_trait]
pub trait TimesManServer {
    async fn run(&self, listen: &str, store: Arc<Mutex<dyn Store>>);
}
