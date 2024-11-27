use super::{Post, Store, Times};

use sqlx;
use sqlx::sqlite::SqlitePool;

use async_trait::async_trait;
use tokio::runtime::Runtime;

#[derive(Clone)]
struct SqliteTimes {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted: i64,
}

impl From<SqliteTimes> for Times {
    fn from(value: SqliteTimes) -> Self {
        Times {
            id: value.id,
            title: value.title,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Clone)]
struct SqlitePost {
    pub id: i64,
    pub tid: i64,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<SqlitePost> for Post {
    fn from(value: SqlitePost) -> Self {
        Self {
            id: value.id,
            post: value.post,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct SqliteStore {
    db: Result<SqlitePool, String>,
    rt: Runtime,
}

impl SqliteStore {
    pub fn new(dbfile: &str) -> Self {
        let rt = Runtime::new().unwrap();

        let db = SqlitePool::connect(dbfile);

        let db = rt.block_on(db).map_err(|e| format!("{e}"));

        Self { db, rt }
    }
}

#[async_trait]
impl Store for SqliteStore {
    async fn check(&self) -> Result<(), String> {
        let db = self.db.clone()?;

        if !db.is_closed() {
            Ok(())
        } else {
            Err("Closed".to_string())
        }
    }

    async fn get_times(&self) -> Result<Vec<Times>, String> {
        let db = self.db.clone()?;

        let sql = sqlx::query_as!(
            SqliteTimes,
            r#"select * from times where deleted = 0"#
        )
        .fetch_all(&db);

        let times = self.rt.block_on(sql).map_err(|e| format!("{}", e))?;

        let result = times.iter().map(|st| Times::from(st.clone())).collect();

        Ok(result)
    }

    async fn create_times(&mut self, title: String) -> Result<Times, String> {
        let db = self.db.clone()?;

        let sql = sqlx::query_as!(
            SqliteTimes,
            r#"insert into times("title") values ($1) returning *"#,
            title
        )
        .fetch_one(&db);

        let times = self.rt.block_on(sql).map_err(|e| format!("{}", e))?;

        Ok(Times::from(times))
    }

    async fn update_times(&mut self, _times: Times) -> Result<Times, String> {
        unimplemented!();
    }

    async fn delete_times(&mut self, _tid: i64) -> Result<(), String> {
        unimplemented!();
    }

    async fn get_posts(&self, tid: i64) -> Result<Vec<Post>, String> {
        let db = self.db.clone()?;
        let sql = sqlx::query_as!(
            SqlitePost,
            r#"select * from posts where tid = $1"#,
            tid
        )
        .fetch_all(&db);

        let posts = self.rt.block_on(sql).map_err(|e| format!("{}", e))?;
        let result = posts.iter().map(|sp| Post::from(sp.clone())).collect();

        Ok(result)
    }

    async fn create_post(
        &mut self,
        tid: i64,
        post: String,
    ) -> Result<Post, String> {
        let db = self.db.clone()?;

        let sql = sqlx::query_as!(
            Post,
            r#"insert into posts(tid, post)
                    values ($1, $2)
                    returning id as "id!", post, created_at, updated_at"#,
            tid,
            post
        )
        .fetch_one(&db);

        let post = self.rt.block_on(sql).map_err(|e| format!("{}", e))?;

        Ok(post)
    }

    async fn delete_post(
        &mut self,
        _tid: i64,
        _pid: i64,
    ) -> Result<(), String> {
        unimplemented!();
    }

    async fn update_post(
        &mut self,
        _tid: i64,
        _post: Post,
    ) -> Result<Post, String> {
        unimplemented!();
    }
}
