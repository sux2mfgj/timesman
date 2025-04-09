use super::{File, Post, Store, Times};

use async_trait::async_trait;
use chrono::Utc;
use migration::{Migrator, MigratorTrait};
use sea_orm::entity::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::fs;
use uuid::Uuid; // Moved import to the top

use std::fmt;
use std::path::{Path, PathBuf};

mod file;
mod post;
mod prelude;
mod times;

impl From<times::Model> for Times {
    fn from(model: times::Model) -> Self {
        Times {
            id: model.id as u64,
            title: model.title,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<post::Model> for Post {
    fn from(model: post::Model) -> Self {
        let post = if let Some(post) = model.text {
            post
        } else {
            "".to_string()
        };

        if let Some(_fid) = model.file_id {
            todo!();
        }

        Self {
            id: model.id as u64,
            post,
            created_at: model.created_at,
            updated_at: model.updated_at,
            file: None,
        }
    }
}

pub struct SqliteStore {
    db: DatabaseConnection,
}

impl SqliteStore {
    ///
    /// SqliteStore::new(":memory:");
    /// SqliteStore::new("//path/to/db.sqlite?mode=rwc");
    pub async fn new(path: &str) -> Result<Self, String> {
        // let mut opt = ConnectOptions::new(format!("sqlite://{}?mode=rwc", path));
        let mut opt = ConnectOptions::new(format!("sqlite:{path}"));

        opt.sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opt).await.unwrap();

        Migrator::up(&db, None).await.unwrap();
        Ok(Self { db })
    }
}

#[async_trait]
impl Store for SqliteStore {
    async fn check(&mut self) -> Result<(), String> {
        self.db.ping().await.map_err(|e| format!("{e}"))
    }

    async fn get_times(&mut self) -> Result<Vec<Times>, String> {
        let ts = times::Entity::find().all(&self.db).await.unwrap();
        Ok(ts.iter().map(|t| Times::from(t.clone())).collect())
    }

    async fn create_times(&mut self, title: String) -> Result<Times, String> {
        let am = times::ActiveModel {
            title: ActiveValue::Set(title),
            ..Default::default()
        };

        let res = am.insert(&self.db).await.unwrap();

        Ok(Times::from(res))
    }

    async fn update_times(&mut self, times: Times) -> Result<Times, String> {
        todo!();
    }

    async fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        todo!();
    }

    async fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        let ps = post::Entity::find().all(&self.db).await.unwrap();
        Ok(ps.iter().map(|p| Post::from(p.clone())).collect())
    }

    async fn create_post(
        &mut self,
        tid: u64,
        post_text: String,                 // Renamed parameter
        file_data: Option<(String, File)>, // Renamed parameter
    ) -> Result<Post, String> {
        if file_data.is_some() {
            todo!();
        }

        let am = post::ActiveModel {
            tid: ActiveValue::Set(tid as i32),
            text: ActiveValue::Set(Some(post_text)),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let res = am.insert(&self.db).await.unwrap();

        Ok(Post::from(res))
    }

    async fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        todo!();
    }

    async fn update_post(
        &mut self,
        tid: u64,
        post: Post,
    ) -> Result<Post, String> {
        todo!();
    }

    async fn get_latest_post(
        &mut self,
        tid: u64,
    ) -> Result<Option<Post>, String> {
        todo!();
    }
}

impl fmt::Debug for SqliteStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

#[cfg(test)]
mod tests {

    use chrono::Utc;
    use log;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database};

    use super::*;

    #[tokio::test]
    async fn test() {
        // let mut opt = ConnectOptions::new("sqlite::memory:");
        let mut opt = ConnectOptions::new("sqlite://./db.sqlite?mode=rwc");

        opt.sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opt).await.unwrap();
        assert!(db.ping().await.is_ok());

        Migrator::up(&db, None).await.unwrap();

        let postam = post::ActiveModel {
            tid: ActiveValue::Set(0),
            text: ActiveValue::Set(Some("hello".to_owned())),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        postam.insert(&db).await.unwrap();

        // post::Entity::find().filter(post::Column::)

        db.close().await.unwrap();
    }

    #[tokio::test]
    async fn test2() {
        let mut store = SqliteStore::new(":memory:").await.unwrap();
        store.check().await.unwrap();
    }
}
