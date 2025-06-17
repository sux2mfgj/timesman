use timesman_grpc::grpc::times_man_client::TimesManClient;
use timesman_grpc::grpc::{TimesTitle, TimesId, CreatePostPrams, DeletePostParam, UpdatePostParam, 
                         CreateTodoParams, TodoDetailParams, UpdateTodoDetailParams, DoneTodoParams};
use timesman_type::{Post, Times, Todo};

pub struct GrpcClient {
    client: TimesManClient<tonic::transport::channel::Channel>,
    rt: tokio::runtime::Runtime,
}

impl super::Client for GrpcClient {
    fn get_times(&mut self) -> Result<Vec<Times>, String> {
        let tary = self
            .rt
            .block_on(async { self.client.get_times(()).await.unwrap() })
            .into_inner();

        let r = tary
            .timeses
            .iter()
            .map(|t| t.clone().into())
            .collect::<Vec<Times>>();
        Ok(r)
    }

    fn create_times(&mut self, title: String) -> Result<Times, String> {
        let request = TimesTitle { title };
        let response = self
            .rt
            .block_on(async { self.client.create_times(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        let request = TimesId { id: tid };
        self.rt
            .block_on(async { self.client.delete_times(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(())
    }

    fn update_times(&mut self, times: Times) -> Result<Times, String> {
        let request: timesman_grpc::grpc::Times = times.into();
        let response = self
            .rt
            .block_on(async { self.client.update_times(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        let request = TimesId { id: tid };
        let response = self
            .rt
            .block_on(async { self.client.get_posts(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        let posts = response
            .into_inner()
            .posts
            .iter()
            .map(|p| p.clone().into())
            .collect::<Vec<Post>>();
        
        Ok(posts)
    }
    fn create_post(&mut self, tid: u64, text: String) -> Result<Post, String> {
        let request = CreatePostPrams { id: tid, text };
        let response = self
            .rt
            .block_on(async { self.client.create_post(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        let request = DeletePostParam { tid, pid };
        self.rt
            .block_on(async { self.client.delete_post(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(())
    }

    fn update_post(&mut self, tid: u64, post: Post) -> Result<Post, String> {
        let request = UpdatePostParam { 
            tid, 
            post: Some(post.into()) 
        };
        let response = self
            .rt
            .block_on(async { self.client.update_post(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn get_todos(&mut self, tid: u64) -> Result<Vec<Todo>, String> {
        let request = TimesId { id: tid };
        let response = self
            .rt
            .block_on(async { self.client.get_todos(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        let todos = response
            .into_inner()
            .todos
            .iter()
            .map(|t| t.clone().into())
            .collect::<Vec<Todo>>();
        
        Ok(todos)
    }

    fn create_todo(&mut self, tid: u64, content: String) -> Result<Todo, String> {
        let request = CreateTodoParams { 
            tid, 
            content, 
            detail: None 
        };
        let response = self
            .rt
            .block_on(async { self.client.create_todo(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn create_todo_with_detail(&mut self, tid: u64, content: String, detail: Option<String>) -> Result<Todo, String> {
        let request = CreateTodoParams { 
            tid, 
            content, 
            detail 
        };
        let response = self
            .rt
            .block_on(async { self.client.create_todo(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn delete_todo(&mut self, tid: u64, tdid: u64) -> Result<(), String> {
        // Note: There's no specific delete_todo endpoint in the proto, 
        // so we'll use done_todo to mark as done for now
        let request = DoneTodoParams { 
            tid, 
            tdid, 
            done: true 
        };
        self.rt
            .block_on(async { self.client.done_todo(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(())
    }

    fn update_todo(&mut self, tid: u64, todo: Todo) -> Result<Todo, String> {
        // For basic todo update, we can use update_todo_detail if detail exists
        if let Some(detail) = &todo.detail {
            let request = UpdateTodoDetailParams { 
                tid, 
                tdid: todo.id, 
                detail: detail.clone() 
            };
            let response = self
                .rt
                .block_on(async { self.client.update_todo_detail(request).await })
                .map_err(|e| format!("gRPC error: {}", e))?;
            
            Ok(response.into_inner().into())
        } else {
            // If no detail, just return the original todo
            // In a full implementation, we'd need an update_todo_content endpoint
            Ok(todo)
        }
    }

    fn get_todo_detail(&mut self, tid: u64, tdid: u64) -> Result<Todo, String> {
        let request = TodoDetailParams { tid, tdid };
        let response = self
            .rt
            .block_on(async { self.client.get_todo_detail(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn update_todo_detail(&mut self, tid: u64, tdid: u64, detail: String) -> Result<Todo, String> {
        let request = UpdateTodoDetailParams { tid, tdid, detail };
        let response = self
            .rt
            .block_on(async { self.client.update_todo_detail(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }

    fn mark_todo_done(&mut self, tid: u64, tdid: u64, done: bool) -> Result<Todo, String> {
        let request = DoneTodoParams { tid, tdid, done };
        let response = self
            .rt
            .block_on(async { self.client.done_todo(request).await })
            .map_err(|e| format!("gRPC error: {}", e))?;
        
        Ok(response.into_inner().into())
    }
}

impl GrpcClient {
    pub fn new(server: &String) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();

        let server: String = server.parse().unwrap();
        let client = rt
            .block_on(async { TimesManClient::connect(server).await.unwrap() });
        Self { client, rt }
    }
}
