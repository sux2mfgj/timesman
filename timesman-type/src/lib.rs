use std::fmt::Debug;

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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    Image(Vec<u8>),
    Text(String),
    Other(Vec<u8>),
}

impl Debug for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FileType::Image(img) => {
                write!(f, "Image file: size {}", img.len())
            }
            FileType::Text(txt) => {
                write!(f, "Text file:  size {}", txt.len())
            }
            FileType::Other(data) => {
                write!(f, "Unknown file:  size {}", data.len())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub name: String,
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
    pub tag: Option<TagId>,
}

pub type TagId = u64;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
}

pub type Tdid = u64;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Todo {
    pub id: Tdid,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub done_at: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_file() {
        println!(
            "{:?}",
            File {
                name: "hello".to_string(),
                ftype: FileType::Image(vec![0x12, 0x34, 0x56, 0x78])
            }
        )
    }
}
