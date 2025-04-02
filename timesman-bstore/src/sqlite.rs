use super::{File, Post, Store, Times};

use sqlx;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use async_trait::async_trait;
use tokio::fs;
use uuid::Uuid;

use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct SqliteTimes {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<SqliteTimes> for Times {
    fn from(value: SqliteTimes) -> Self {
        Times {
            id: value.id as u64,
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
    pub fid: Option<i64>, // Changed to Option<i64>
}

impl SqlitePost {
    // Helper to convert to Post, potentially loading file data
    async fn into_post(self, store: &SqliteStore) -> Result<Post, String> {
        let file = if let Some(fid) = self.fid {
            Some(store.load_file(fid, self.tid).await?)
        } else {
            None
        };

        Ok(Post {
            id: self.id as u64,
            post: self.post,
            created_at: self.created_at,
            updated_at: self.updated_at,
            file,
        })
    }
}

// Note: Direct From<SqlitePost> for Post is removed as it needs async context to load files.
// Use SqlitePost::into_post instead.

#[derive(Clone)]
struct SqliteFile {
    pub id: i64,
    pub tid: i64,
    pub name: String,
    pub path: String,
    pub created_at: chrono::NaiveDateTime,
}

pub struct SqliteStore {
    db: SqlitePool,
    db_file_path: String,
    files_dir: PathBuf,
}

impl SqliteStore {
    pub async fn new(db_file_path: &String, create: bool) -> Result<Self, String> {
        let db_path = Path::new(db_file_path);
        let files_dir = db_path
            .parent()
            .unwrap_or_else(|| Path::new(".")) // Use current dir if no parent
            .join(format!(
                "{}_files",
                db_path.file_stem().unwrap().to_str().unwrap()
            ));

        // Create files directory if it doesn't exist
        fs::create_dir_all(&files_dir)
            .await
            .map_err(|e| format!("Failed to create files directory: {}", e))?;

        let opt = SqliteConnectOptions::new()
            .filename(db_file_path)
            .create_if_missing(create);
        let db = SqlitePool::connect_with(opt)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        // TODO: Run migrations if necessary

        Ok(Self {
            db,
            db_file_path: db_file_path.clone(),
            files_dir,
        })
    }

    async fn save_file(
        &self,
        tid: i64,
        file_data: Option<(String, File)>, // Renamed parameter for clarity
    ) -> Result<Option<i64>, String> {
        let Some((name, file_content)) = file_data else {
            return Ok(None); // No file provided, return Ok(None) for fid
        };

        let file_bytes = match file_content {
            File::Image(data) => data,
            File::Other(data) => data,
        };

        // Generate a unique filename for storage
        let unique_filename = format!("{}.bin", Uuid::new_v4());
        let storage_path = self.files_dir.join(&unique_filename);

        // Save file to the designated path
        fs::write(&storage_path, &file_bytes)
            .await
            .map_err(|e| format!("Failed to write file {}: {}", storage_path.display(), e))?;

        // Create a new entry in the files table
        let result = sqlx::query!(
            r#"insert into files(tid, name, path) values ($1, $2, $3) returning id"#,
            tid,
            name,
            unique_filename // Store relative path or just the unique name
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| format!("Failed to insert file record: {}", e))?;

        Ok(Some(result.id)) // Return the new file ID
    }

    async fn load_file(&self, fid: i64, _tid: i64) -> Result<(String, File), String> {
        // Fetch file metadata from the database
        let file_record = sqlx::query_as!(
            SqliteFile,
            r#"select id, tid, name, path, created_at from files where id = $1"#,
            fid
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| format!("Failed to fetch file record {}: {}", fid, e))?;

        let storage_path = self.files_dir.join(&file_record.path);

        // Read file from the designated path
        let file_bytes = fs::read(&storage_path)
            .await
            .map_err(|e| format!("Failed to read file {}: {}", storage_path.display(), e))?;

        // Basic MIME type guessing based on name, consider using `mime_guess` crate for better results
        let mime_type = mime_guess::from_path(&file_record.name).first_or_octet_stream();

        let file = if mime_type.type_() == "image" {
            File::Image(file_bytes)
        } else {
            File::Other(file_bytes)
        };

        Ok((file_record.name, file))
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
        let sql = sqlx::query_as!(SqliteTimes, r#"select * from times"#)
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

    async fn update_times(&mut self, times: Times) -> Result<Times, String> {
        let tid = times.id as i64;
        let now = chrono::Utc::now().naive_utc(); // Use UTC for consistency

        let sql = sqlx::query_as!(
            SqliteTimes,
            r#"update times set title = $1, updated_at = $2 where id = $3 returning *"#,
            times.title,
            now,
            tid
        )
        .fetch_one(&self.db);

        let updated_times = sql.await.map_err(|e| format!("{}", e))?;

        Ok(Times::from(updated_times))
    }

    async fn delete_times(&mut self, tid: u64) -> Result<(), String> {
        let tid_i64 = tid as i64;
        // TODO: Consider transaction and deleting associated files from disk
        sqlx::query!("delete from posts where tid = $1", tid_i64)
            .execute(&self.db)
            .await
            .map_err(|e| format!("Failed to delete posts for times {}: {}", tid, e))?;

        sqlx::query!("delete from files where tid = $1", tid_i64)
            .execute(&self.db)
            .await
            .map_err(|e| format!("Failed to delete files for times {}: {}", tid, e))?;

        let result = sqlx::query!("delete from times where id = $1", tid_i64)
            .execute(&self.db)
            .await
            .map_err(|e| format!("{}", e))?;

        if result.rows_affected() == 0 {
            Err(format!("Times with id {} not found", tid))
        } else {
            Ok(())
        }
    }

    async fn get_posts(&mut self, tid: u64) -> Result<Vec<Post>, String> {
        let tid_i64 = tid as i64;
        let sql_posts = sqlx::query_as!(
            SqlitePost,
            r#"select id, tid, post, created_at, updated_at, fid from posts where tid = $1 order by created_at asc"#,
            tid_i64
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| format!("Failed to fetch posts for times {}: {}", tid, e))?;

        // Convert SqlitePost to Post, loading files asynchronously
        let mut result_posts = Vec::new();
        for sp in sql_posts {
            let post = sp.into_post(self).await?;
            result_posts.push(post);
        }

        Ok(result_posts)
    }

    async fn create_post(
        &mut self,
        tid: u64,
        post_text: String, // Renamed parameter
        file_data: Option<(String, File)>, // Renamed parameter
    ) -> Result<Post, String> {
        let tid_i64 = tid as i64;

        // Save the file first, if provided, to get the fid
        let fid = self.save_file(tid_i64, file_data).await?;

        // Insert the post record
        let created_post_record = sqlx::query_as!(
            SqlitePost,
            r#"insert into posts(tid, post, fid)
                    values ($1, $2, $3)
                    returning id, tid, post, created_at, updated_at, fid"#,
            tid_i64,
            post_text,
            fid // This will be None if no file was saved
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| format!("Failed to create post: {}", e))?;

        // Convert the created record back into a Post, potentially loading the file info again
        // (Alternatively, construct Post manually if file data is readily available)
        created_post_record.into_post(self).await
    }

    async fn delete_post(&mut self, tid: u64, pid: u64) -> Result<(), String> {
        let tid_i64 = tid as i64;
        let pid_i64 = pid as i64;

        // Optional: Find the post first to get the associated file ID for deletion
        let post_to_delete = sqlx::query!(
            "select fid from posts where id = $1 and tid = $2",
            pid_i64,
            tid_i64
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| format!("Error checking post {}: {}", pid, e))?;

        // Delete the post record
        let result = sqlx::query!(
            "delete from posts where id = $1 and tid = $2",
            pid_i64,
            tid_i64
        )
        .execute(&self.db)
        .await
        .map_err(|e| format!("Failed to delete post {}: {}", pid, e))?;

        if result.rows_affected() == 0 {
            return Err(format!("Post with id {} not found for times {}", pid, tid));
        }

        // If the post had a file, delete the file record and the file itself
        if let Some(Some(fid)) = post_to_delete.map(|r| r.fid) {
            // Fetch file path before deleting the record
            let file_record = sqlx::query!("select path from files where id = $1", fid)
                .fetch_optional(&self.db)
                .await
                .map_err(|e| format!("Error fetching file record {}: {}", fid, e))?;

            // Delete file record
            sqlx::query!("delete from files where id = $1", fid)
                .execute(&self.db)
                .await
                .map_err(|e| format!("Failed to delete file record {}: {}", fid, e))?;

            // Delete file from disk
            if let Some(record) = file_record {
                let file_path = self.files_dir.join(record.path);
                fs::remove_file(&file_path)
                    .await
                    .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
            }
        }

        Ok(())
    }

    async fn update_post(&mut self, tid: u64, post: Post) -> Result<Post, String> {
        // Note: Updating files associated with a post is complex (replace? remove? add?).
        // This implementation only updates the text content. File updates would require more logic.
        let tid_i64 = tid as i64;
        let pid_i64 = post.id as i64;
        let now = chrono::Utc::now().naive_utc();

        let updated_record = sqlx::query_as!(
            SqlitePost,
            r#"update posts set post = $1, updated_at = $2
               where id = $3 and tid = $4
               returning id, tid, post, created_at, updated_at, fid"#,
            post.post,
            now,
            pid_i64,
            tid_i64
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| format!("Failed to update post {}: {}", post.id, e))?;

        updated_record.into_post(self).await
    }

    async fn get_latest_post(&mut self, tid: u64) -> Result<Option<Post>, String> {
        let tid_i64 = tid as i64;
        let latest_post_record = sqlx::query_as!(
            SqlitePost,
            r#"select id, tid, post, created_at, updated_at, fid
               from posts where tid = $1
               order by created_at desc limit 1"#,
            tid_i64
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| format!("Failed to get latest post for times {}: {}", tid, e))?;

        if let Some(record) = latest_post_record {
            Ok(Some(record.into_post(self).await?))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Debug for SqliteStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqliteStore")
            .field("db_file_path", &self.db_file_path)
            .field("files_dir", &self.files_dir)
            .field("db_pool", &"SqlitePool { ... }") // Avoid printing sensitive pool details
            .finish()
    }
}

// Add mime_guess dependency for load_file
use mime_guess;
