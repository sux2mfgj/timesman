use chrono;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Times {
    id: i64,
    title: String,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>
}

pub struct Requester {
    pub server: String
}

impl Requester {
    pub fn new(server: &String) -> Self {
        Requester {
            server: server.clone()
        }
    }

    pub fn get_list(&self) -> Option<Vec<Times>> {
        None
    }
}
