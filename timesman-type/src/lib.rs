use serde::{Deserialize, Serialize};

pub type Tid = u64;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Times {
    pub id: Tid,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl std::fmt::Display for Times {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(utime) = self.updated_at {
            write!(
                f,
                "{} {} {} {}",
                self.id, self.title, self.created_at, utime
            )
        } else {
            write!(f, "{} {} {}", self.id, self.title, self.created_at)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    Image,
    Text,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
    pub ftype: FileType,
}

pub type Pid = u64;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Post {
    pub id: Pid,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub file: Option<File>,
}

pub type Tdid = u64;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Todo {
    pub id: Tdid,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub done_at: Option<chrono::NaiveDateTime>,
}
