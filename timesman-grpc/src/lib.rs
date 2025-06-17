pub mod grpc {
    tonic::include_proto!("timesman");
}

impl Into<timesman_type::Times> for grpc::Times {
    fn into(self) -> timesman_type::Times {
        let c = self.created_at.unwrap();
        let ctime = chrono::DateTime::from_timestamp(c.seconds, c.nanos as u32)
            .unwrap();

        let utime = if let Some(u) = self.updated_at {
            Some(
                chrono::DateTime::from_timestamp(u.seconds, u.nanos as u32)
                    .unwrap()
                    .naive_local(),
            )
        } else {
            None
        };

        timesman_type::Times {
            id: self.id,
            title: self.title,
            created_at: ctime.naive_local(),
            updated_at: utime,
        }
    }
}

use chrono::{Datelike, NaiveDateTime, Timelike};

fn to_timestamp(c: NaiveDateTime) -> prost_types::Timestamp {
    prost_types::Timestamp::date_time(
        c.year() as i64,
        c.month() as u8,
        c.day() as u8,
        c.hour() as u8,
        c.minute() as u8,
        c.second() as u8,
    )
    .unwrap()
}

impl From<timesman_type::Times> for grpc::Times {
    fn from(value: timesman_type::Times) -> Self {
        let ctime = {
            let c = value.created_at;
            prost_types::Timestamp::date_time(
                c.year() as i64,
                c.month() as u8,
                c.day() as u8,
                c.hour() as u8,
                c.minute() as u8,
                c.second() as u8,
            )
            .unwrap()
        };

        let utime = if let Some(u) = value.updated_at {
            let t = prost_types::Timestamp::date_time(
                u.year() as i64,
                u.month() as u8,
                u.day() as u8,
                u.hour() as u8,
                u.minute() as u8,
                u.second() as u8,
            )
            .unwrap();
            Some(t)
        } else {
            None
        };

        Self {
            id: value.id as u64,
            title: value.title,
            created_at: Some(ctime),
            updated_at: utime,
        }
    }
}

impl Into<timesman_type::Post> for grpc::Post {
    fn into(self) -> timesman_type::Post {
        let c = self.created_at.unwrap();
        let ctime = chrono::DateTime::from_timestamp(c.seconds, c.nanos as u32)
            .unwrap()
            .naive_local();

        let utime = if let Some(u) = self.updated_at {
            Some(
                chrono::DateTime::from_timestamp(u.seconds, u.nanos as u32)
                    .unwrap()
                    .naive_local(),
            )
        } else {
            None
        };

        timesman_type::Post {
            id: self.id,
            post: self.post,
            created_at: ctime,
            updated_at: utime,
            file: None,
            tag: self.tagid,
        }
    }
}

impl From<timesman_type::Post> for grpc::Post {
    fn from(value: timesman_type::Post) -> Self {
        let ctime = {
            let c = value.created_at;
            prost_types::Timestamp::date_time(
                c.year() as i64,
                c.month() as u8,
                c.day() as u8,
                c.hour() as u8,
                c.minute() as u8,
                c.second() as u8,
            )
            .unwrap()
        };

        let utime = if let Some(u) = value.updated_at {
            let t = prost_types::Timestamp::date_time(
                u.year() as i64,
                u.month() as u8,
                u.day() as u8,
                u.hour() as u8,
                u.minute() as u8,
                u.second() as u8,
            )
            .unwrap();
            Some(t)
        } else {
            None
        };

        Self {
            id: value.id,
            post: value.post,
            created_at: Some(ctime),
            updated_at: utime,
            tagid: value.tag,
        }
    }
}

impl From<timesman_type::Todo> for grpc::Todo {
    fn from(value: timesman_type::Todo) -> Self {
        let ctime = to_timestamp(value.created_at);
        let dtime = if let Some(u) = value.done_at {
            Some(to_timestamp(u))
        } else {
            None
        };

        Self {
            id: value.id,
            content: value.content,
            detail: value.detail,
            created_at: Some(ctime),
            done_at: dtime,
        }
    }
}

impl Into<timesman_type::Todo> for grpc::Todo {
    fn into(self) -> timesman_type::Todo {
        let created_at = if let Some(c) = self.created_at {
            chrono::DateTime::from_timestamp(c.seconds, c.nanos as u32)
                .unwrap()
                .naive_local()
        } else {
            panic!();
        };

        let done_at = if let Some(d) = self.done_at {
            Some(
                chrono::DateTime::from_timestamp(d.seconds, d.nanos as u32)
                    .unwrap()
                    .naive_local(),
            )
        } else {
            None
        };

        timesman_type::Todo {
            id: self.id,
            content: self.content,
            detail: self.detail,
            created_at,
            done_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    
    #[test]
    fn test_todo_grpc_conversion_with_detail() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let done = NaiveDateTime::parse_from_str("2023-01-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test timesman_type::Todo -> grpc::Todo with detail
        let original_todo = timesman_type::Todo {
            id: 1,
            content: "Test task".to_string(),
            detail: Some("This is a detailed description\nwith multiple lines\nand special chars: ñáéíóú 🚀".to_string()),
            created_at: created,
            done_at: Some(done),
        };
        
        let grpc_todo: grpc::Todo = original_todo.clone().into();
        assert_eq!(grpc_todo.id, 1);
        assert_eq!(grpc_todo.content, "Test task");
        assert_eq!(grpc_todo.detail, Some("This is a detailed description\nwith multiple lines\nand special chars: ñáéíóú 🚀".to_string()));
        assert!(grpc_todo.created_at.is_some());
        assert!(grpc_todo.done_at.is_some());
        
        // Test grpc::Todo -> timesman_type::Todo roundtrip
        let roundtrip_todo: timesman_type::Todo = grpc_todo.into();
        assert_eq!(roundtrip_todo.id, original_todo.id);
        assert_eq!(roundtrip_todo.content, original_todo.content);
        assert_eq!(roundtrip_todo.detail, original_todo.detail);
        assert_eq!(roundtrip_todo.created_at, original_todo.created_at);
        assert_eq!(roundtrip_todo.done_at, original_todo.done_at);
    }

    #[test]
    fn test_todo_grpc_conversion_without_detail() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test backward compatibility - todo without detail
        let original_todo = timesman_type::Todo {
            id: 2,
            content: "Simple task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        
        let grpc_todo: grpc::Todo = original_todo.clone().into();
        assert_eq!(grpc_todo.id, 2);
        assert_eq!(grpc_todo.content, "Simple task");
        assert_eq!(grpc_todo.detail, None);
        assert!(grpc_todo.created_at.is_some());
        assert!(grpc_todo.done_at.is_none());
        
        // Test roundtrip
        let roundtrip_todo: timesman_type::Todo = grpc_todo.into();
        assert_eq!(roundtrip_todo, original_todo);
    }

    #[test]
    fn test_grpc_message_validation() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test with empty detail string vs None
        let todo_empty_detail = timesman_type::Todo {
            id: 3,
            content: "Task".to_string(),
            detail: Some("".to_string()),
            created_at: created,
            done_at: None,
        };
        
        let grpc_todo: grpc::Todo = todo_empty_detail.into();
        assert_eq!(grpc_todo.detail, Some("".to_string()));
        
        // Test that we handle None properly
        let todo_none_detail = timesman_type::Todo {
            id: 4,
            content: "Task".to_string(),
            detail: None,
            created_at: created,
            done_at: None,
        };
        
        let grpc_todo: grpc::Todo = todo_none_detail.into();
        assert_eq!(grpc_todo.detail, None);
        
        // Test with very long detail
        let long_detail = "x".repeat(5000);
        let todo_long_detail = timesman_type::Todo {
            id: 5,
            content: "Task".to_string(),
            detail: Some(long_detail.clone()),
            created_at: created,
            done_at: None,
        };
        
        let grpc_todo: grpc::Todo = todo_long_detail.into();
        assert_eq!(grpc_todo.detail, Some(long_detail));
        
        // Test with special characters and Unicode
        let special_detail = "Special chars: \n\r\t \"quotes\" 'apostrophes' \\backslashes\\ ñáéíóú 中文 العربية 🚀🎉".to_string();
        let todo_special = timesman_type::Todo {
            id: 6,
            content: "Special task".to_string(),
            detail: Some(special_detail.clone()),
            created_at: created,
            done_at: None,
        };
        
        let grpc_todo: grpc::Todo = todo_special.into();
        assert_eq!(grpc_todo.detail, Some(special_detail));
    }

    #[test]
    fn test_times_grpc_conversion() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let updated = NaiveDateTime::parse_from_str("2023-01-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test Times conversion with updated_at
        let original_times = timesman_type::Times {
            id: 1,
            title: "Test Times".to_string(),
            created_at: created,
            updated_at: Some(updated),
        };
        
        let grpc_times: grpc::Times = original_times.clone().into();
        assert_eq!(grpc_times.id, 1);
        assert_eq!(grpc_times.title, "Test Times");
        assert!(grpc_times.created_at.is_some());
        assert!(grpc_times.updated_at.is_some());
        
        let roundtrip_times: timesman_type::Times = grpc_times.into();
        assert_eq!(roundtrip_times.id, original_times.id);
        assert_eq!(roundtrip_times.title, original_times.title);
        assert_eq!(roundtrip_times.created_at, original_times.created_at);
        assert_eq!(roundtrip_times.updated_at, original_times.updated_at);
    }

    #[test]
    fn test_post_grpc_conversion() {
        let created = NaiveDateTime::parse_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test Post conversion
        let original_post = timesman_type::Post {
            id: 1,
            post: "Test post content".to_string(),
            created_at: created,
            updated_at: None,
            file: None,
            tag: Some(42),
        };
        
        let grpc_post: grpc::Post = original_post.clone().into();
        assert_eq!(grpc_post.id, 1);
        assert_eq!(grpc_post.post, "Test post content");
        assert!(grpc_post.created_at.is_some());
        assert!(grpc_post.updated_at.is_none());
        assert_eq!(grpc_post.tagid, Some(42));
        
        let roundtrip_post: timesman_type::Post = grpc_post.into();
        assert_eq!(roundtrip_post.id, original_post.id);
        assert_eq!(roundtrip_post.post, original_post.post);
        assert_eq!(roundtrip_post.created_at, original_post.created_at);
        assert_eq!(roundtrip_post.updated_at, original_post.updated_at);
        assert_eq!(roundtrip_post.tag, original_post.tag);
    }

    #[test]
    fn test_timestamp_conversion_edge_cases() {
        // Test with minimum and maximum valid dates
        let min_date = NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let max_date = NaiveDateTime::parse_from_str("2038-01-19 03:14:07", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let todo_min = timesman_type::Todo {
            id: 1,
            content: "Min date task".to_string(),
            detail: Some("Task with minimum timestamp".to_string()),
            created_at: min_date,
            done_at: None,
        };
        
        let grpc_todo: grpc::Todo = todo_min.clone().into();
        let roundtrip_todo: timesman_type::Todo = grpc_todo.into();
        assert_eq!(roundtrip_todo.created_at, todo_min.created_at);
        
        let todo_max = timesman_type::Todo {
            id: 2,
            content: "Max date task".to_string(),
            detail: Some("Task with maximum timestamp".to_string()),
            created_at: max_date,
            done_at: Some(max_date),
        };
        
        let grpc_todo: grpc::Todo = todo_max.clone().into();
        let roundtrip_todo: timesman_type::Todo = grpc_todo.into();
        assert_eq!(roundtrip_todo.created_at, todo_max.created_at);
        assert_eq!(roundtrip_todo.done_at, todo_max.done_at);
    }
}
