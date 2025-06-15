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
            created_at,
            done_at,
        }
    }
}
