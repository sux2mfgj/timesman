use std::sync::Arc;
use tokio::sync::Mutex;

use super::TimesManServer;

use timesman_bstore::Store;

use async_trait::async_trait;

use timesman_grpc::grpc;
use timesman_grpc::grpc::times_man_server;

use tonic::transport::server::Server;

pub struct GrpcServer {}

#[async_trait]
impl TimesManServer for GrpcServer {
    async fn run(
        &self,
        listen: &str,
        store: Arc<Mutex<dyn Store>>,
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

pub struct TMServer {
    store: Arc<Mutex<dyn Store>>,
}

#[async_trait]
impl times_man_server::TimesMan for TMServer {
    async fn get_times(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<grpc::TimesArray>, tonic::Status> {
        let mut store = self.store.lock().await;

        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        let mut timeses = Vec::new();
        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            timeses.push(grpc::Times::from(times));
        }

        Ok(tonic::Response::new(grpc::TimesArray { timeses }))
    }

    async fn create_times(
        &self,
        request: tonic::Request<grpc::TimesTitle>,
    ) -> Result<tonic::Response<grpc::Times>, tonic::Status> {
        let mut store = self.store.lock().await;
        let title = request.into_inner().title;

        let times_store = store.create(title).await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        let mut ts = times_store.lock().await;
        let times = ts.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        Ok(tonic::Response::new(grpc::Times::from(times)))
    }

    async fn delete_times(
        &self,
        request: tonic::Request<grpc::TimesId>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut store = self.store.lock().await;
        let tid = request.into_inner().id;

        store.delete(tid).await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        Ok(tonic::Response::new(()))
    }

    async fn update_times(
        &self,
        request: tonic::Request<grpc::Times>,
    ) -> Result<tonic::Response<grpc::Times>, tonic::Status> {
        let mut store = self.store.lock().await;
        let times_data: timesman_type::Times = request.into_inner().into();
        let tid = times_data.id;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let updated_times = ts.update(times_data).await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                return Ok(tonic::Response::new(grpc::Times::from(updated_times)));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn get_posts(
        &self,
        request: tonic::Request<grpc::TimesId>,
    ) -> Result<tonic::Response<grpc::PostArray>, tonic::Status> {
        let mut store = self.store.lock().await;
        let tid = request.into_inner().id;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let post_store = ts.pstore().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;
                let mut ps = post_store.lock().await;
                let posts = ps.get_all().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                let posts = posts
                    .iter()
                    .map(|p| grpc::Post::from(p.clone()))
                    .collect::<Vec<grpc::Post>>();

                return Ok(tonic::Response::new(grpc::PostArray { posts }));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn create_post(
        &self,
        request: tonic::Request<grpc::CreatePostPrams>,
    ) -> Result<tonic::Response<grpc::Post>, tonic::Status> {
        let mut store = self.store.lock().await;
        let params = request.into_inner();
        let tid = params.id;
        let text = params.text;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let post_store = ts.pstore().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;
                let mut ps = post_store.lock().await;
                let post = ps.post(text, None).await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                return Ok(tonic::Response::new(grpc::Post::from(post)));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn delete_post(
        &self,
        request: tonic::Request<grpc::DeletePostParam>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut store = self.store.lock().await;
        let params = request.into_inner();
        let tid = params.tid;
        let pid = params.pid;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let post_store = ts.pstore().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;
                let mut ps = post_store.lock().await;
                ps.delete(pid).await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                return Ok(tonic::Response::new(()));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn update_post(
        &self,
        request: tonic::Request<grpc::UpdatePostParam>,
    ) -> Result<tonic::Response<grpc::Post>, tonic::Status> {
        let mut store = self.store.lock().await;
        let params = request.into_inner();
        let tid = params.tid;
        let post_data = params.post.ok_or_else(|| {
            tonic::Status::new(tonic::Code::InvalidArgument, "Post data is required")
        })?;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let post_store = ts.pstore().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;
                let mut ps = post_store.lock().await;
                let updated_post = ps.update(post_data.into()).await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                return Ok(tonic::Response::new(grpc::Post::from(updated_post)));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn get_todos(
        &self,
        request: tonic::Request<grpc::TimesId>,
    ) -> Result<tonic::Response<grpc::TodoArray>, tonic::Status> {
        let mut store = self.store.lock().await;
        let tid = request.into_inner().id;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let todo_store = ts.tdstore().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;
                let mut tds = todo_store.lock().await;
                let todos = tds.get().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                let todos = todos
                    .iter()
                    .map(|t| grpc::Todo::from(t.clone()))
                    .collect::<Vec<grpc::Todo>>();

                return Ok(tonic::Response::new(grpc::TodoArray { todos }));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn create_todo(
        &self,
        request: tonic::Request<grpc::CreateTodoParams>,
    ) -> Result<tonic::Response<grpc::Todo>, tonic::Status> {
        let mut store = self.store.lock().await;
        let params = request.into_inner();
        let tid = params.tid;
        let content = params.content;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let todo_store = ts.tdstore().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;
                let mut tds = todo_store.lock().await;
                let todo = tds.new(content).await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                return Ok(tonic::Response::new(grpc::Todo::from(todo)));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn done_todo(
        &self,
        request: tonic::Request<grpc::DoneTodoParams>,
    ) -> Result<tonic::Response<grpc::Todo>, tonic::Status> {
        let mut store = self.store.lock().await;
        let params = request.into_inner();
        let tid = params.tid;
        let tdid = params.tdid;
        let done = params.done;

        // Find the times store by ID
        let times_stores = store.get().await.map_err(|e| {
            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
        })?;

        for times_store in times_stores {
            let mut ts = times_store.lock().await;
            let times = ts.get().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
            })?;
            
            if times.id == tid {
                let todo_store = ts.tdstore().await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;
                let mut tds = todo_store.lock().await;
                let todo = tds.done(tdid, done).await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                return Ok(tonic::Response::new(grpc::Todo::from(todo)));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }
}
