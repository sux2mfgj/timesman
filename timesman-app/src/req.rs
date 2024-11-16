use chrono;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Times {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted: bool,
}

#[derive(Deserialize)]
pub struct Post {
    pub id: i64,
    pub times_id: i64,
    pub post: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize)]
struct ResponseBase {
    status: i64,
    text: String,
}

#[derive(Clone)]
pub struct Requester {
    pub server: String,
}

impl Requester {
    pub fn new(server: &String) -> Self {
        Requester {
            server: server.clone(),
        }
    }

    pub fn get_list(&self) -> Option<Vec<Times>> {
        let url = self.server.clone() + "/times";

        debug!("Request HTTP Get to {}", url);

        #[derive(Deserialize)]
        struct GetTimesResponse {
            base: ResponseBase,
            times: Vec<Times>,
        }

        let resp = reqwest::blocking::get(url)
            .unwrap()
            .json::<GetTimesResponse>()
            .unwrap();

        if resp.base.status != 0 {
            None
        } else {
            Some(resp.times)
        }
    }

    pub fn create_times(&self, title: &String) -> Option<Times> {
        let url = self.server.clone() + "/times";

        debug!("Request HTTP Post to {}", url);

        #[derive(Serialize)]
        struct CreateTimesRequest {
            title: String,
        }

        #[derive(Deserialize)]
        struct CreateTimesResponse {
            base: ResponseBase,
            times: Times,
        }

        let data = CreateTimesRequest {
            title: title.to_string(),
        };

        let client = reqwest::blocking::Client::new();
        let result = client.post(url).json(&data).send().unwrap();

        let resp = result.json::<CreateTimesResponse>().unwrap();

        if resp.base.status != 0 {
            None
        } else {
            Some(resp.times)
        }
    }

    pub fn get_posts(&self, tid: i64) -> Option<Vec<Post>> {
        let url = format!("{}/times/{}", self.server, tid);

        debug!("Request HTTP Get to {}", url);

        #[derive(Deserialize)]
        struct Response {
            base: ResponseBase,
            posts: Vec<Post>,
        }

        let resp: Response = reqwest::blocking::get(url)
            .unwrap()
            .json::<Response>()
            .unwrap();

        if resp.base.status != 0 {
            None
        } else {
            Some(resp.posts)
        }
    }

    pub fn post_post(&self, tid: i64, post: &String) -> Result<Post, String> {
        let url = format!("{}/times/{}", self.server, tid);

        debug!("Request HTTP Post to {}", self.server);

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

        let client = reqwest::blocking::Client::new();
        let result = client.post(url).json(&data).send().unwrap();

        let resp = result.json::<Response>().unwrap();

        if resp.base.status != 0 {
            return Err(format!("request error: {}", resp.base.text));
        }

        Ok(Post {
            times_id: tid,
            id: resp.pid,
            post: post.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        })
    }

    pub fn delete_times(&self, tid: i64) -> Result<(), String> {
        let url = format!("{}/times/{}", self.server, tid);

        debug!("Request HTTP Delete to {}", self.server);

        let client = reqwest::blocking::Client::new();
        let result = client.delete(url).send().unwrap();

        let resp = result.json::<ResponseBase>().unwrap();

        if resp.status != 0 {
            return Err(format!("request error: {}", resp.text));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_get_times() {
        let json = r#"{
  "base": {
    "status": 0,
    "text": "Ok"
  },
  "times": [
    {
      "id": 3,
      "title": "timed について",
      "created_at": "2024-11-02T10:09:01",
      "updated_at": null,
      "flags": 0
    },
    {
      "id": 7,
      "title": "saa",
      "created_at": "2024-11-06T02:42:48",
      "updated_at": null,
      "flags": 0
    },
    {
      "id": 8,
      "title": "サボテン見守りについて",
      "created_at": "2024-11-06T06:30:43",
      "updated_at": null,
      "flags": 0
    }
  ]
}
"#;

        #[derive(Deserialize)]
        struct GetTimesResponse {
            base: ResponseBase,
            times: Vec<Times>,
        }

        let _data: GetTimesResponse = serde_json::from_str(json).unwrap();
    }
}
