pub mod http;

use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;

use store::Store;

#[async_trait]
pub trait TimesManServer {
    async fn run(&self, listen: &str, store: Arc<Mutex<dyn Store>>);
}
