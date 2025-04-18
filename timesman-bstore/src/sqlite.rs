use super::{File, Post, Store, Times};

use async_trait::async_trait;
use chrono::Utc;
use migration::{Migrator, MigratorTrait};
use sea_orm::{entity::*, query::*};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use uuid::Uuid; // Moved import to the top

use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

mod file;
mod post;
mod prelude;
mod times;

impl From<times::Model> for Times {
    fn from(model: times::Model) -> Self {
        Self {
            id: model.id as u64,
            title: model.title,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl post::Model {
    fn into(&self, file: Option<(String, File)>) -> Post {
        let post = if let Some(post) = self.text.clone() {
            post
        } else {
            "".to_string()
        };

        Post {
            id: self.id as u64,
            post,
            created_at: self.created_at,
            updated_at: self.updated_at,
            file,
        }
    }
}

pub struct SqliteStore {
    db: DatabaseConnection,
    file_path_base: PathBuf,
}

impl SqliteStore {
    /// SqliteStore::new(":memory:");
    /// SqliteStore::new("//path/to/db.sqlite?mode=rwc");
    pub async fn new(
        db_path: &str,
        file_path_base: PathBuf,
    ) -> Result<Self, String> {
        let mut opt = ConnectOptions::new(format!("sqlite:{db_path}"));

        opt.sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opt).await.unwrap();

        Migrator::up(&db, None).await.unwrap();

        if !file_path_base.exists() {
            std::fs::create_dir(&file_path_base).unwrap();
        }

        Ok(Self { db, file_path_base })
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
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
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
        let ps = post::Entity::find()
            .filter(post::Column::Tid.eq(tid))
            .order_by_asc(post::Column::Id)
            .all(&self.db)
            .await
            .unwrap();

        let fs = file::Entity::find()
            .filter(file::Column::Tid.eq(tid))
            .all(&self.db)
            .await
            .unwrap();

        Ok(ps
            .iter()
            .map(|p| {
                let file = if let Some(fid) = p.file_id {
                    let fdir = self.file_path_base.join(format!("{}", p.tid));
                    let f = fdir.join(format!("{fid}"));
                    let mut data = vec![];
                    let mut file = std::fs::File::open(f).unwrap();
                    file.read_to_end(&mut data).unwrap();

                    let f = fs.iter().find(|f| f.id == fid).unwrap();
                    Some((f.name.clone(), File::Image(data)))
                } else {
                    None
                };

                p.into(file)
            })
            .collect())
    }

    async fn create_post(
        &mut self,
        tid: u64,
        post_text: String,
        file_data: Option<(String, File)>,
    ) -> Result<Post, String> {
        let fdata = file_data.clone();

        let fid = if let Some((name, data)) = file_data {
            let file = file::ActiveModel {
                tid: ActiveValue::Set(tid as i32),
                name: ActiveValue::Set(name),
                ..Default::default()
            };

            let file = file.insert(&self.db).await.unwrap();

            let fid = file.id;

            let fdir = self.file_path_base.join(format!("{tid}"));

            if !fdir.exists() {
                std::fs::create_dir(&fdir).unwrap();
            }

            let fpath = fdir.join(format!("{}", &fid));

            if fpath.exists() {
                todo!();
            }

            let mut ofile = fs::File::create(fpath).unwrap();
            match data {
                File::Text(text) => {
                    write!(ofile, "{text}").unwrap();
                }
                File::Image(data) => {
                    ofile.write_all(data.as_slice()).unwrap();
                }
                File::Other(data) => {
                    ofile.write_all(data.as_slice()).unwrap();
                }
            }

            Some(fid)
        } else {
            None
        };

        let am = post::ActiveModel {
            tid: ActiveValue::Set(tid as i32),
            text: ActiveValue::Set(Some(post_text)),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            file_id: ActiveValue::Set(fid),
            ..Default::default()
        };

        let res = am.insert(&self.db).await.unwrap();

        let post = if let Some(text) = res.text {
            text
        } else {
            "".to_string()
        };

        Ok(Post {
            id: res.id as u64,
            post,
            created_at: res.created_at,
            updated_at: res.updated_at,
            file: fdata,
        })
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

        db.close().await.unwrap();
    }

    #[tokio::test]
    async fn test2() {
        let mut store = SqliteStore::new(":memory:", "./path").await.unwrap();
        store.check().await.unwrap();
    }
}
