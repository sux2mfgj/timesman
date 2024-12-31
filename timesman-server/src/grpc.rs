use std::sync::Arc;
use tokio::sync::Mutex;

use super::TimesManServer;

use timesman_bstore::Store;

use async_trait::async_trait;

use timesman_grpc::grpc;
use timesman_grpc::grpc::times_man_server;

use tonic::transport::server::Server;

pub struct GrpcServer {}

#[tonic::async_trait]
impl TimesManServer for GrpcServer {
    async fn run(
        &self,
        listen: &str,
        store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
    ) {
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
    store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
}

#[async_trait]
impl times_man_server::TimesMan for TMServer {
    async fn get_times(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<grpc::TimesArray>, tonic::Status> {
        let mut store = self.store.lock().await;

        let times = store.get_times().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        let timeses = times
            .iter()
            .map(|t| t.clone().into())
            .collect::<Vec<grpc::Times>>();

        Ok(tonic::Response::new(grpc::TimesArray { timeses }))
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

    async fn delete_times(
        &self,
        _request: tonic::Request<grpc::TimesId>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }

    async fn update_times(
        &self,
        _request: tonic::Request<grpc::Times>,
    ) -> Result<tonic::Response<grpc::Times>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }

    async fn get_posts(
        &self,
        _request: tonic::Request<grpc::TimesId>,
    ) -> Result<tonic::Response<grpc::PostArray>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }

    async fn create_post(
        &self,
        _request: tonic::Request<grpc::CreatePostPrams>,
    ) -> Result<tonic::Response<grpc::Post>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }

    async fn delete_post(
        &self,
        _request: tonic::Request<grpc::DeletePostParam>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }

    async fn update_post(
        &self,
        _request: tonic::Request<grpc::UpdatePostParam>,
    ) -> Result<tonic::Response<grpc::Post>, tonic::Status> {
        Err(tonic::Status::new(
            tonic::Code::Unimplemented,
            "unimplemented",
        ))
    }
}
