use chrono;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Times {
    pub id: i64,
    pub title: String,
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
}
