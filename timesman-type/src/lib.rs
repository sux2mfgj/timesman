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
    pub detail: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub done_at: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn times_display_with_updated_at() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let updated = NaiveDateTime::parse_from_str("2023-01-02 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let times = Times {
            id: 1,
            title: "Test Times".to_string(),
            created_at: created,
            updated_at: Some(updated),
        };
        
        let display = format!("{}", times);
        assert!(display.contains("1"));
        assert!(display.contains("Test Times"));
        assert!(display.contains("2023-01-01 10:00:00"));
        assert!(display.contains("2023-01-02 11:00:00"));
    }

    #[test]
    fn times_display_without_updated_at() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let times = Times {
            id: 1,
            title: "Test Times".to_string(),
            created_at: created,
            updated_at: None,
        };
        
        let display = format!("{}", times);
        assert!(display.contains("1"));
        assert!(display.contains("Test Times"));
        assert!(display.contains("2023-01-01 10:00:00"));
        assert!(!display.contains("2023-01-02"));
    }

    #[test]
    fn times_equality() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let times1 = Times {
            id: 1,
            title: "Test".to_string(),
            created_at: created,
            updated_at: None,
        };
        
        let times2 = Times {
            id: 1,
            title: "Test".to_string(),
            created_at: created,
            updated_at: None,
        };
        
        assert_eq!(times1, times2);
    }

    #[test]
    fn filetype_debug_image() {
        let image_data = vec![0x12, 0x34, 0x56, 0x78];
        let filetype = FileType::Image(image_data);
        let debug_str = format!("{:?}", filetype);
        assert!(debug_str.contains("Image file: size 4"));
    }

    #[test]
    fn filetype_debug_text() {
        let text_data = "Hello, world!".to_string();
        let filetype = FileType::Text(text_data);
        let debug_str = format!("{:?}", filetype);
        assert!(debug_str.contains("Text file:  size 13"));
    }

    #[test]
    fn filetype_debug_other() {
        let other_data = vec![0xAB, 0xCD, 0xEF];
        let filetype = FileType::Other(other_data);
        let debug_str = format!("{:?}", filetype);
        assert!(debug_str.contains("Unknown file:  size 3"));
    }

    #[test]
    fn filetype_equality() {
        let image1 = FileType::Image(vec![1, 2, 3]);
        let image2 = FileType::Image(vec![1, 2, 3]);
        let image3 = FileType::Image(vec![1, 2, 4]);
        
        assert_eq!(image1, image2);
        assert_ne!(image1, image3);
        
        let text1 = FileType::Text("hello".to_string());
        let text2 = FileType::Text("hello".to_string());
        let text3 = FileType::Text("world".to_string());
        
        assert_eq!(text1, text2);
        assert_ne!(text1, text3);
    }

    #[test]
    fn file_creation() {
        let file = File {
            name: "test.txt".to_string(),
            ftype: FileType::Text("content".to_string()),
        };
        
        assert_eq!(file.name, "test.txt");
        match file.ftype {
            FileType::Text(content) => assert_eq!(content, "content"),
            _ => panic!("Expected Text file type"),
        }
    }

    #[test]
    fn post_with_file() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let file = File {
            name: "attachment.jpg".to_string(),
            ftype: FileType::Image(vec![1, 2, 3, 4]),
        };
        
        let post = Post {
            id: 1,
            post: "Test post".to_string(),
            created_at: created,
            updated_at: None,
            file: Some(file),
            tag: Some(42),
        };
        
        assert_eq!(post.id, 1);
        assert_eq!(post.post, "Test post");
        assert!(post.file.is_some());
        assert_eq!(post.tag, Some(42));
    }

    #[test]
    fn post_without_file() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let post = Post {
            id: 2,
            post: "Simple post".to_string(),
            created_at: created,
            updated_at: None,
            file: None,
            tag: None,
        };
        
        assert_eq!(post.id, 2);
        assert_eq!(post.post, "Simple post");
        assert!(post.file.is_none());
        assert!(post.tag.is_none());
    }

    #[test]
    fn tag_creation() {
        let tag = Tag {
            id: 1,
            name: "work".to_string(),
        };
        
        assert_eq!(tag.id, 1);
        assert_eq!(tag.name, "work");
    }

    #[test]
    fn todo_done() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let done = NaiveDateTime::parse_from_str("2023-01-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let todo = Todo {
            id: 1,
            content: "Finish project".to_string(),
            detail: Some("Complete the final implementation and testing of the project before the deadline.".to_string()),
            created_at: created,
            done_at: Some(done),
        };
        
        assert_eq!(todo.id, 1);
        assert_eq!(todo.content, "Finish project");
        assert!(todo.done_at.is_some());
    }

    #[test]
    fn todo_pending() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let todo = Todo {
            id: 2,
            content: "Start new task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        
        assert_eq!(todo.id, 2);
        assert_eq!(todo.content, "Start new task");
        assert!(todo.done_at.is_none());
    }

    #[test]
    fn todo_equality() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let todo1 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        
        let todo2 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        
        assert_eq!(todo1, todo2);
    }

    #[test]
    fn todo_with_detail() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let todo = Todo {
            id: 3,
            content: "Research project".to_string(),
            detail: Some("Research the latest trends in web development, focusing on performance optimization and user experience. Look into frameworks like Next.js, SvelteKit, and emerging technologies. Document findings with links and examples.".to_string()),
            created_at: created,
            done_at: None,
        };
        
        assert_eq!(todo.id, 3);
        assert_eq!(todo.content, "Research project");
        assert!(todo.detail.is_some());
        assert!(todo.detail.as_ref().unwrap().contains("web development"));
        assert!(todo.done_at.is_none());
    }

    #[test]
    fn todo_detail_equality() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let detail_text = "Detailed explanation of the task".to_string();
        
        let todo1 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some(detail_text.clone()),
            created_at: created,
            done_at: None,
        };
        
        let todo2 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some(detail_text),
            created_at: created,
            done_at: None,
        };
        
        assert_eq!(todo1, todo2);
        
        // Test inequality when details differ
        let todo3 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some("Different detail".to_string()),
            created_at: created,
            done_at: None,
        };
        
        assert_ne!(todo1, todo3);
    }

    #[test]
    fn serde_serialization() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let times = Times {
            id: 1,
            title: "Test".to_string(),
            created_at: created,
            updated_at: None,
        };
        
        let json = serde_json::to_string(&times).unwrap();
        let deserialized: Times = serde_json::from_str(&json).unwrap();
        
        assert_eq!(times, deserialized);
    }

    #[test]
    fn test_todo_detail_serialization() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test with detail field
        let todo_with_detail = Todo {
            id: 1,
            content: "Test task".to_string(),
            detail: Some("Detailed description of the task with multiple lines\nand special characters: ñáéíóú".to_string()),
            created_at: created,
            done_at: None,
        };
        
        let json = serde_json::to_string(&todo_with_detail).unwrap();
        let deserialized: Todo = serde_json::from_str(&json).unwrap();
        assert_eq!(todo_with_detail, deserialized);
        
        // Test without detail field  
        let todo_without_detail = Todo {
            id: 2,
            content: "Simple task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        
        let json = serde_json::to_string(&todo_without_detail).unwrap();
        let deserialized: Todo = serde_json::from_str(&json).unwrap();
        assert_eq!(todo_without_detail, deserialized);
    }

    #[test] 
    fn test_todo_detail_backward_compatibility() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test that old JSON without detail field can be deserialized
        let old_todo_json = r#"{
            "id": 1,
            "content": "Old task without detail field",
            "created_at": "2023-01-01T10:00:00",
            "done_at": null
        }"#;
        
        let deserialized: Todo = serde_json::from_str(old_todo_json).unwrap();
        assert_eq!(deserialized.id, 1);
        assert_eq!(deserialized.content, "Old task without detail field");
        assert_eq!(deserialized.detail, None);
        assert_eq!(deserialized.created_at, created);
        assert_eq!(deserialized.done_at, None);
        
        // Test that new JSON with explicit null detail works
        let new_todo_json = r#"{
            "id": 2,
            "content": "New task with explicit null detail",
            "detail": null,
            "created_at": "2023-01-01T10:00:00",
            "done_at": null
        }"#;
        
        let deserialized: Todo = serde_json::from_str(new_todo_json).unwrap();
        assert_eq!(deserialized.detail, None);
    }

    #[test]
    fn test_todo_detail_validation() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test with empty detail
        let todo_empty_detail = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some("".to_string()),
            created_at: created,
            done_at: None,
        };
        assert!(todo_empty_detail.detail.as_ref().unwrap().is_empty());
        
        // Test with very long detail (simulate realistic limit)
        let long_detail = "x".repeat(10000);
        let todo_long_detail = Todo {
            id: 2,
            content: "Task".to_string(),
            detail: Some(long_detail.clone()),
            created_at: created,
            done_at: None,
        };
        assert_eq!(todo_long_detail.detail.as_ref().unwrap().len(), 10000);
        
        // Test with Unicode characters
        let unicode_detail = "Task with Unicode: 🚀 ñáéíóú 中文 العربية 🎉".to_string();
        let todo_unicode = Todo {
            id: 3,
            content: "Unicode task".to_string(),
            detail: Some(unicode_detail.clone()),
            created_at: created,
            done_at: None,
        };
        assert_eq!(todo_unicode.detail.as_ref().unwrap(), &unicode_detail);
        
        // Test serialization roundtrip with Unicode
        let json = serde_json::to_string(&todo_unicode).unwrap();
        let deserialized: Todo = serde_json::from_str(&json).unwrap();
        assert_eq!(todo_unicode, deserialized);
        
        // Test with newlines and special characters
        let special_detail = "Line 1\nLine 2\r\nTabs:\t\tSpaces:   \"Quotes\" 'Single' \\Backslashes\\".to_string();
        let todo_special = Todo {
            id: 4,
            content: "Special chars".to_string(),
            detail: Some(special_detail.clone()),
            created_at: created,
            done_at: None,
        };
        
        let json = serde_json::to_string(&todo_special).unwrap();
        let deserialized: Todo = serde_json::from_str(&json).unwrap();
        assert_eq!(todo_special, deserialized);
    }

    #[test]
    fn test_todo_detail_equality() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test equality with same detail
        let todo1 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some("Same detail".to_string()),
            created_at: created,
            done_at: None,
        };
        
        let todo2 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some("Same detail".to_string()),
            created_at: created,
            done_at: None,
        };
        assert_eq!(todo1, todo2);
        
        // Test inequality with different details
        let todo3 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some("Different detail".to_string()),
            created_at: created,
            done_at: None,
        };
        assert_ne!(todo1, todo3);
        
        // Test equality with both None details
        let todo4 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        
        let todo5 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        assert_eq!(todo4, todo5);
        
        // Test inequality between Some and None
        assert_ne!(todo1, todo4);
        
        // Test with empty string vs None
        let todo6 = Todo {
            id: 1,
            content: "Task".to_string(),
            detail: Some("".to_string()),
            created_at: created,
            done_at: None,
        };
        assert_ne!(todo4, todo6); // None != Some("")
    }
}
