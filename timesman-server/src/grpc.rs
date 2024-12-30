use std::sync::Arc;
use std::sync::Mutex;

use super::TimesManServer;

use timesman_bstore::{Post, Store, Times};

use async_trait::async_trait;

use timesman_grpc::grpc;
use timesman_grpc::grpc::times_man_server;

use tonic::transport::server::Server;

pub struct GrpcServer {}

#[async_trait]
impl TimesManServer for GrpcServer {
    async fn run(&self, listen: &str, store: Arc<Mutex<dyn Store>>) {
        let addr = listen.parse().unwrap();

        Server::builder()
            .add_service(times_man_server::TimesManServer::new(TMServer {
                store,
            }))
            .serve(addr)
            .await
            .unwrap();
    }
}

struct TMServer {
    store: Arc<Mutex<dyn Store>>,
}

#[async_trait]
impl times_man_server::TimesMan for TMServer {
    async fn get_times(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<grpc::TimesArray>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }

    async fn create_times(
        &self,
        _request: tonic::Request<grpc::TimesTitle>,
    ) -> Result<tonic::Response<grpc::Times>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }
}
