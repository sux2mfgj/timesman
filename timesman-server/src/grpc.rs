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
        let detail = params.detail;

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
                let mut todo = tds.new(content).await.map_err(|e| {
                    tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                })?;

                // If detail is provided, update the todo with the detail
                if let Some(detail_text) = detail {
                    todo.detail = Some(detail_text);
                    todo = tds.update(todo).await.map_err(|e| {
                        tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                    })?;
                }

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

    async fn get_todo_detail(
        &self,
        request: tonic::Request<grpc::TodoDetailParams>,
    ) -> Result<tonic::Response<grpc::Todo>, tonic::Status> {
        let mut store = self.store.lock().await;
        let params = request.into_inner();
        let tid = params.tid;
        let tdid = params.tdid;

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

                // Find the specific todo by ID
                for todo in todos {
                    if todo.id == tdid {
                        return Ok(tonic::Response::new(grpc::Todo::from(todo)));
                    }
                }

                return Err(tonic::Status::new(
                    tonic::Code::NotFound,
                    format!("Todo with id {} not found", tdid),
                ));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }

    async fn update_todo_detail(
        &self,
        request: tonic::Request<grpc::UpdateTodoDetailParams>,
    ) -> Result<tonic::Response<grpc::Todo>, tonic::Status> {
        let mut store = self.store.lock().await;
        let params = request.into_inner();
        let tid = params.tid;
        let tdid = params.tdid;
        let new_detail = params.detail;

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

                // Find and update the specific todo by ID
                for mut todo in todos {
                    if todo.id == tdid {
                        todo.detail = Some(new_detail);
                        let updated_todo = tds.update(todo).await.map_err(|e| {
                            tonic::Status::new(tonic::Code::Aborted, format!("{e}"))
                        })?;

                        return Ok(tonic::Response::new(grpc::Todo::from(updated_todo)));
                    }
                }

                return Err(tonic::Status::new(
                    tonic::Code::NotFound,
                    format!("Todo with id {} not found", tdid),
                ));
            }
        }

        Err(tonic::Status::new(
            tonic::Code::NotFound,
            format!("Times with id {} not found", tid),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use timesman_grpc::grpc::times_man_server::TimesMan;

    async fn setup_test_server() -> TMServer {
        let store_type = timesman_bstore::StoreType::Memory;
        let store = store_type.to_store().await.unwrap();
        TMServer { store }
    }

    async fn create_test_times(server: &TMServer) -> u64 {
        let request = Request::new(grpc::TimesTitle {
            title: "Test Times".to_string(),
        });
        let response = server.create_times(request).await.unwrap();
        response.into_inner().id
    }

    #[tokio::test]
    async fn test_get_todo_detail_endpoint() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Create a todo with detail
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Test todo".to_string(),
            detail: Some("This is a detailed description\nwith multiple lines".to_string()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();
        let tdid = todo.id;

        // Test get_todo_detail endpoint
        let get_request = Request::new(grpc::TodoDetailParams { tid, tdid });
        let get_response = server.get_todo_detail(get_request).await.unwrap();
        let retrieved_todo = get_response.into_inner();

        assert_eq!(retrieved_todo.id, tdid);
        assert_eq!(retrieved_todo.content, "Test todo");
        assert_eq!(retrieved_todo.detail, Some("This is a detailed description\nwith multiple lines".to_string()));
        assert!(retrieved_todo.created_at.is_some());
        assert!(retrieved_todo.done_at.is_none());
    }

    #[tokio::test]
    async fn test_update_todo_detail_endpoint() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Create a todo without detail
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Test todo".to_string(),
            detail: None,
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();
        let tdid = todo.id;

        // Update the todo with detail
        let update_request = Request::new(grpc::UpdateTodoDetailParams {
            tid,
            tdid,
            detail: "This is a new detailed description with Unicode: ñáéíóú 🚀".to_string(),
        });
        let update_response = server.update_todo_detail(update_request).await.unwrap();
        let updated_todo = update_response.into_inner();

        assert_eq!(updated_todo.id, tdid);
        assert_eq!(updated_todo.content, "Test todo");
        assert_eq!(updated_todo.detail, Some("This is a new detailed description with Unicode: ñáéíóú 🚀".to_string()));

        // Verify the update persisted by getting the todo again
        let get_request = Request::new(grpc::TodoDetailParams { tid, tdid });
        let get_response = server.get_todo_detail(get_request).await.unwrap();
        let retrieved_todo = get_response.into_inner();
        assert_eq!(retrieved_todo.detail, Some("This is a new detailed description with Unicode: ñáéíóú 🚀".to_string()));
    }

    #[tokio::test]
    async fn test_create_todo_with_detail() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Test creating todo with detail
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Todo with detail".to_string(),
            detail: Some("Initial detail text\nwith newlines and special chars: \"quotes\" & symbols".to_string()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();

        assert_eq!(todo.content, "Todo with detail");
        assert_eq!(todo.detail, Some("Initial detail text\nwith newlines and special chars: \"quotes\" & symbols".to_string()));
        assert!(todo.created_at.is_some());
        assert!(todo.done_at.is_none());

        // Test creating todo without detail
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Simple todo".to_string(),
            detail: None,
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();

        assert_eq!(todo.content, "Simple todo");
        assert_eq!(todo.detail, None);

        // Test creating todo with empty detail
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Empty detail todo".to_string(),
            detail: Some("".to_string()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();

        assert_eq!(todo.content, "Empty detail todo");
        assert_eq!(todo.detail, Some("".to_string()));
    }

    #[tokio::test]
    async fn test_todo_detail_error_handling() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Test get_todo_detail with non-existent times ID
        let get_request = Request::new(grpc::TodoDetailParams {
            tid: 99999,
            tdid: 1,
        });
        let get_result = server.get_todo_detail(get_request).await;
        assert!(get_result.is_err());
        assert_eq!(get_result.unwrap_err().code(), tonic::Code::NotFound);

        // Test get_todo_detail with non-existent todo ID
        let get_request = Request::new(grpc::TodoDetailParams {
            tid,
            tdid: 99999,
        });
        let get_result = server.get_todo_detail(get_request).await;
        assert!(get_result.is_err());
        assert_eq!(get_result.unwrap_err().code(), tonic::Code::NotFound);

        // Test update_todo_detail with non-existent times ID
        let update_request = Request::new(grpc::UpdateTodoDetailParams {
            tid: 99999,
            tdid: 1,
            detail: "New detail".to_string(),
        });
        let update_result = server.update_todo_detail(update_request).await;
        assert!(update_result.is_err());
        assert_eq!(update_result.unwrap_err().code(), tonic::Code::NotFound);

        // Test update_todo_detail with non-existent todo ID
        let update_request = Request::new(grpc::UpdateTodoDetailParams {
            tid,
            tdid: 99999,
            detail: "New detail".to_string(),
        });
        let update_result = server.update_todo_detail(update_request).await;
        assert!(update_result.is_err());
        assert_eq!(update_result.unwrap_err().code(), tonic::Code::NotFound);

        // Test create_todo with non-existent times ID
        let create_request = Request::new(grpc::CreateTodoParams {
            tid: 99999,
            content: "Test".to_string(),
            detail: None,
        });
        let create_result = server.create_todo(create_request).await;
        assert!(create_result.is_err());
        assert_eq!(create_result.unwrap_err().code(), tonic::Code::NotFound);
    }

    #[tokio::test]
    async fn test_get_todos_includes_details() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Create several todos with and without details
        let todos_data = vec![
            ("Todo 1", Some("Detail 1")),
            ("Todo 2", None),
            ("Todo 3", Some("Detail 3 with\nmultiple lines")),
            ("Todo 4", Some("")),
        ];

        let mut created_todo_ids = Vec::new();
        for (content, detail) in todos_data.iter() {
            let create_request = Request::new(grpc::CreateTodoParams {
                tid,
                content: content.to_string(),
                detail: detail.map(|d| d.to_string()),
            });
            let create_response = server.create_todo(create_request).await.unwrap();
            created_todo_ids.push(create_response.into_inner().id);
        }

        // Get all todos and verify details are included
        let get_request = Request::new(grpc::TimesId { id: tid });
        let get_response = server.get_todos(get_request).await.unwrap();
        let todos = get_response.into_inner().todos;

        assert_eq!(todos.len(), 4);

        for (i, todo) in todos.iter().enumerate() {
            let expected_detail = todos_data[i].1.map(|d| d.to_string());
            assert_eq!(todo.content, todos_data[i].0);
            assert_eq!(todo.detail, expected_detail);
            assert!(created_todo_ids.contains(&todo.id));
        }
    }

    #[tokio::test]
    async fn test_todo_detail_persistence() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Create a todo with detail
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Persistent todo".to_string(),
            detail: Some("Original detail".to_string()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let tdid = create_response.into_inner().id;

        // Update the detail multiple times
        let details = vec![
            "First update",
            "Second update with special chars: ñáéíóú",
            "Third update\nwith\nmultiple\nlines",
            "",
        ];

        for detail in details.iter() {
            let update_request = Request::new(grpc::UpdateTodoDetailParams {
                tid,
                tdid,
                detail: detail.to_string(),
            });
            let update_response = server.update_todo_detail(update_request).await.unwrap();
            let updated_todo = update_response.into_inner();
            assert_eq!(updated_todo.detail, Some(detail.to_string()));

            // Verify persistence by getting the todo
            let get_request = Request::new(grpc::TodoDetailParams { tid, tdid });
            let get_response = server.get_todo_detail(get_request).await.unwrap();
            let retrieved_todo = get_response.into_inner();
            assert_eq!(retrieved_todo.detail, Some(detail.to_string()));
        }
    }

    #[tokio::test]
    async fn test_todo_detail_with_done_status() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Create a todo with detail
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Todo to be done".to_string(),
            detail: Some("This todo will be marked as done".to_string()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();
        let tdid = todo.id;

        // Mark todo as done
        let done_request = Request::new(grpc::DoneTodoParams {
            tid,
            tdid,
            done: true,
        });
        let done_response = server.done_todo(done_request).await.unwrap();
        let done_todo = done_response.into_inner();

        assert_eq!(done_todo.detail, Some("This todo will be marked as done".to_string()));
        assert!(done_todo.done_at.is_some());

        // Update detail of done todo
        let update_request = Request::new(grpc::UpdateTodoDetailParams {
            tid,
            tdid,
            detail: "Updated detail for done todo".to_string(),
        });
        let update_response = server.update_todo_detail(update_request).await.unwrap();
        let updated_todo = update_response.into_inner();

        assert_eq!(updated_todo.detail, Some("Updated detail for done todo".to_string()));
        assert!(updated_todo.done_at.is_some());

        // Mark todo as not done
        let undone_request = Request::new(grpc::DoneTodoParams {
            tid,
            tdid,
            done: false,
        });
        let undone_response = server.done_todo(undone_request).await.unwrap();
        let undone_todo = undone_response.into_inner();

        assert_eq!(undone_todo.detail, Some("Updated detail for done todo".to_string()));
        assert!(undone_todo.done_at.is_none());
    }

    #[tokio::test]
    async fn test_todo_detail_edge_cases() {
        let server = setup_test_server().await;
        let tid = create_test_times(&server).await;

        // Test with very long detail
        let long_detail = "x".repeat(10000);
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Long detail todo".to_string(),
            detail: Some(long_detail.clone()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();
        assert_eq!(todo.detail, Some(long_detail));

        // Test with detail containing only whitespace
        let whitespace_detail = "   \n\t\r\n   ".to_string();
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Whitespace detail todo".to_string(),
            detail: Some(whitespace_detail.clone()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();
        assert_eq!(todo.detail, Some(whitespace_detail));

        // Test with detail containing control characters
        let control_detail = "Detail with control chars: \u{0001}\u{0002}\u{001F}".to_string();
        let create_request = Request::new(grpc::CreateTodoParams {
            tid,
            content: "Control chars todo".to_string(),
            detail: Some(control_detail.clone()),
        });
        let create_response = server.create_todo(create_request).await.unwrap();
        let todo = create_response.into_inner();
        assert_eq!(todo.detail, Some(control_detail));
    }
}
