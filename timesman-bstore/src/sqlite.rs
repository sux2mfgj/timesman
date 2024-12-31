use super::{Post, Store, Times};

use sqlx;
use sqlx::sqlite::SqlitePool;

use async_trait::async_trait;

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
    db: SqlitePool,
}

pub struct SqliteStoreBuilder {
    dbfile: String,
}

impl SqliteStoreBuilder {
    pub fn new(dbfile: &str) -> Self {
        Self {
            dbfile: dbfile.to_string(),
        }
    }

    pub async fn build(&self) -> Result<SqliteStore, String> {
        let db = SqlitePool::connect(&self.dbfile)
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(SqliteStore { db })
    }
}

#[async_trait]
impl Store for SqliteStore {
    async fn check(&mut self) -> Result<(), String> {
        if !self.db.is_closed() {
            Ok(())
        } else {
            Err("Closed".to_string())
        }
    }

    async fn get_times(&mut self) -> Result<Vec<Times>, String> {
        let sql = sqlx::query_as!(
            SqliteTimes,
            r#"select * from times where deleted = 0"#
        )
        .fetch_all(&self.db);

        let times = sql.await.map_err(|e| format!("{e}"))?;

        let result = times.iter().map(|st| Times::from(st.clone())).collect();

        Ok(result)
    }

    async fn create_times(&mut self, title: String) -> Result<Times, String> {
        let sql = sqlx::query_as!(
            SqliteTimes,
            r#"insert into times("title") values ($1) returning *"#,
            title
        )
        .fetch_one(&self.db);

        let times = sql.await.map_err(|e| format!("{}", e))?;

        Ok(Times::from(times))
    }

    async fn update_times(&mut self, _times: Times) -> Result<Times, String> {
        unimplemented!();
    }

    async fn delete_times(&mut self, _tid: i64) -> Result<(), String> {
        unimplemented!();
    }

    async fn get_posts(&mut self, tid: i64) -> Result<Vec<Post>, String> {
        let sql = sqlx::query_as!(
            SqlitePost,
            r#"select * from posts where tid = $1"#,
            tid
        )
        .fetch_all(&self.db);

        let posts = sql.await.map_err(|e| format!("{}", e))?;
        let result = posts.iter().map(|sp| Post::from(sp.clone())).collect();

        Ok(result)
    }

    async fn create_post(
        &mut self,
        tid: i64,
        post: String,
    ) -> Result<Post, String> {
        let sql = sqlx::query_as!(
            Post,
            r#"insert into posts(tid, post)
                    values ($1, $2)
                    returning id as "id!", post, created_at, updated_at"#,
            tid,
            post
        )
        .fetch_one(&self.db);

        let post = sql.await.map_err(|e| format!("{}", e))?;

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

    async fn get_latest_post(
        &mut self,
        tid: i64,
    ) -> Result<Option<Post>, String> {
        Ok(None)
    }
}
