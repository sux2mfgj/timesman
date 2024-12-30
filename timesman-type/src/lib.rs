use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Times {
    pub id: i64,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
