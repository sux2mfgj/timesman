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

    async fn update(&mut self, _todo: Todo) -> Result<Todo, String> {
        // Note: Update method not implemented in gRPC proto
        Err("Todo update not supported via gRPC".to_string())
    }

    async fn delete(&mut self, _tdid: Tdid) -> Result<(), String> {
        // Note: Delete method not implemented in gRPC proto
        Err("Todo delete not supported via gRPC".to_string())
    }
}
