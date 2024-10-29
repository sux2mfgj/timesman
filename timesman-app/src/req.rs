use chrono;
use log;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Times {
    pub id: i64,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
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
}
