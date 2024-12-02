use serde::{Deserialize, Serialize};

use super::{Post, Store, Times};
use async_trait::async_trait;

#[derive(Deserialize, Clone)]
struct RemPost {
    pub id: i64,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Clone)]
struct RemTimes {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<RemTimes> for Times {
    fn from(value: RemTimes) -> Self {
        Self {
            id: value.id,
            title: value.title,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<RemPost> for Post {
    fn from(value: RemPost) -> Self {
        Self {
            id: value.id,
            post: value.post,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Deserialize)]
struct ResponseBase {
    status: i64,
    text: String,
}

pub struct RemoteStore {
    server: String,
}

impl RemoteStore {
    pub fn new(mut server: String) -> Self {
        let server = if server.ends_with('/') {
            server.pop();
            server
        } else {
            server
        };

        Self { server }
    }
}

#[async_trait]
impl Store for RemoteStore {
    async fn check(&self) -> Result<(), String> {
        match self.get_times().await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn get_times(&self) -> Result<Vec<Times>, String> {
        let url = self.server.clone() + "/times";

        // debug!("Request HTTP Get to {}", url);

        #[derive(Deserialize)]
        struct Response {
            base: ResponseBase,
            times: Vec<RemTimes>,
        }

        let resp = reqwest::get(url)
            .await
            .unwrap()
            .json::<Response>()
            .await
            .unwrap();

        let times = resp
            .times
            .iter()
            .map(|rt| Times::from(rt.clone()))
            .collect();

        if resp.base.status != 0 {
            Err(resp.base.text)
        } else {
            Ok(times)
        }
    }

    async fn create_times(&mut self, title: String) -> Result<Times, String> {
        let url = self.server.clone() + "/times";

        // debug!("Request HTTP Post to {}", url);

        #[derive(Serialize)]
        struct CreateTimesRequest {
            title: String,
        }

        #[derive(Deserialize)]
        struct CreateTimesResponse {
            base: ResponseBase,
            times: RemTimes,
        }

        let data = CreateTimesRequest {
            title: title.to_string(),
        };

        let client = reqwest::Client::new();
        let result = client.post(url).json(&data).send().await.unwrap();

        let resp = result.json::<CreateTimesResponse>().await.unwrap();

        if resp.base.status != 0 {
            Err(resp.base.text)
        } else {
            Ok(Times::from(resp.times))
        }
    }

    async fn delete_times(&mut self, tid: i64) -> Result<(), String> {
        let url = format!("{}/times/{}", self.server, tid);

        // debug!("Request HTTP Delete to {}", self.server);

        let client = reqwest::Client::new();
        let result = client.delete(url).send().await.unwrap();

        let resp = result.json::<ResponseBase>().await.unwrap();

        if resp.status != 0 {
            return Err(format!("request error: {}", resp.text));
        }

        Ok(())
    }

    async fn update_times(&mut self, _times: Times) -> Result<Times, String> {
        unimplemented!();
    }

    async fn get_posts(&self, tid: i64) -> Result<Vec<Post>, String> {
        let url = format!("{}/times/{}", self.server, tid);

        // debug!("Request HTTP Get to {}", url);

        #[derive(Deserialize)]
        struct Response {
            base: ResponseBase,
            posts: Vec<RemPost>,
        }

        let resp = reqwest::get(url)
            .await
            .unwrap()
            .json::<Response>()
            .await
            .unwrap();

        let posts =
            resp.posts.iter().map(|rp| Post::from(rp.clone())).collect();

        if resp.base.status != 0 {
            Err(resp.base.text)
        } else {
            Ok(posts)
        }
    }

    async fn create_post(
        &mut self,
        tid: i64,
        post: String,
    ) -> Result<Post, String> {
        let url = format!("{}/times/{}", self.server, tid);

        // debug!("Request HTTP Post to {}", self.server);

        #[derive(Serialize)]
        struct Request {
            post: String,
        }

        #[derive(Deserialize)]
        struct Response {
            base: ResponseBase,
            pid: i64,
        }

        let data = Request {
            post: post.to_string(),
        };

        let client = reqwest::Client::new();
        let result = client.post(url).json(&data).send().await.unwrap();

        let resp = result.json::<Response>().await.unwrap();

        if resp.base.status != 0 {
            return Err(format!("request error: {}", resp.base.text));
        }

        Ok(Post {
            id: resp.pid,
            post: post.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        })
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

    async fn get_latest_post(&self, tid: i64) -> Result<Option<Post>, String> {
        let posts = self.get_posts(tid).await?;

        if let Some(p) = posts.iter().max_by_key(|p| p.id) {
            return Ok(Some(p.clone()));
        }

        Ok(None)
    }
}
