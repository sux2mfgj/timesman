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
        let gtodos = c.get_todos(tonic::Request::new(tid)).await.unwrap();

        let todos =
            gtodos.into_inner().todos.iter().map(|t| t.into()).collect();

        Ok(todos)
    }

    async fn new(&mut self, content: String) -> Result<Todo, String> {
        todo!();
    }

    async fn done(&mut self, tdid: Tdid, done: bool) -> Result<Todo, String> {
        todo!();
    }

    async fn update(&mut self, todo: Todo) -> Result<Todo, String> {
        todo!();
    }

    async fn delete(&mut self, tdid: Tdid) -> Result<(), String> {
        todo!();
    }
}
