use super::GrpcClient;
use super::TodoStore;

use async_trait::async_trait;
use timesman_type::{Tdid, Tid, Todo};

use timesman_grpc::grpc;
use tonic;

pub(crate) struct GrpcTodoStore {
    client: GrpcClient,
    tid: Tid,
}

impl GrpcTodoStore {
    pub fn new(client: GrpcClient, tid: Tid) -> Self {
        Self { client, tid }
    }
}

#[async_trait]
impl TodoStore for GrpcTodoStore {
    async fn get(&mut self) -> Result<Vec<Todo>, String> {
        let mut c = self.client.lock().await;

        let tid = grpc::TimesId { id: self.tid };
        let gtodos = c.get_todos(tonic::Request::new(tid)).await.map_err(|e| format!("{e}"))?;

        let todos = gtodos
            .into_inner()
            .todos
            .iter()
            .map(|t| t.clone().into())
            .collect();

        Ok(todos)
    }

    async fn new(&mut self, content: String) -> Result<Todo, String> {
        let mut c = self.client.lock().await;
        let param = grpc::CreateTodoParams {
            tid: self.tid,
            content,
            detail: None,
        };
        let todo = c
            .create_todo(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(todo.into_inner().into())
    }

    async fn done(&mut self, tdid: Tdid, done: bool) -> Result<Todo, String> {
        let mut c = self.client.lock().await;
        let param = grpc::DoneTodoParams {
            tid: self.tid,
            tdid,
            done,
        };
        let todo = c
            .done_todo(tonic::Request::new(param))
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(todo.into_inner().into())
    }

    async fn update(&mut self, todo: Todo) -> Result<Todo, String> {
        let mut c = self.client.lock().await;
        
        // For now, we'll use UpdateTodoDetail if the detail field changed
        if let Some(detail) = &todo.detail {
            let param = grpc::UpdateTodoDetailParams {
                tid: self.tid,
                tdid: todo.id,
                detail: detail.clone(),
            };
            let updated_todo = c
                .update_todo_detail(tonic::Request::new(param))
                .await
                .map_err(|e| format!("{e}"))?;
            
            Ok(updated_todo.into_inner().into())
        } else {
            // If no detail, return the todo as-is since we can't update content via gRPC yet
            Ok(todo)
        }
    }

    async fn delete(&mut self, _tdid: Tdid) -> Result<(), String> {
        // Note: Delete method not implemented in gRPC proto
        Err("Todo delete not supported via gRPC".to_string())
    }
}
