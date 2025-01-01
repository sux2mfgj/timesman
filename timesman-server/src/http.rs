use std::sync::Arc;
use tokio::sync::Mutex;

use timesman_bstore::Store;
use timesman_type::{Post, Times};

use super::TimesManServer;

use actix_web::{web, App, HttpResponse, Responder};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct Context {
    store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
}

pub struct HttpServer {}

#[async_trait]
impl TimesManServer for HttpServer {
    async fn run(
        &self,
        listen: &str,
        store: Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>,
    ) {
        actix_web::HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(Context {
                    store: store.clone(),
                }))
                .route("/times", web::get().to(get_times))
                .route("/times", web::post().to(create_times))
                .route("/times/{tid}", web::delete().to(delete_times))
                .route("/times/{tid}", web::get().to(get_posts))
                .route("/times/{tid}", web::post().to(post_post))
        })
        .bind(listen)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}

#[derive(Serialize)]
struct ResponseBase {
    status: u64,
    text: String,
}

#[derive(Serialize)]
struct ResponseTimes {
    base: ResponseBase,
    times: Vec<Times>,
}

async fn get_times(ctx: web::Data<Context>) -> impl Responder {
    let mut store = ctx.store.lock().await; //.unwrap();
    let times = match store.get_times().await {
        Ok(times) => times,
        Err(e) => {
            tracing::info!("failed to get times from store {e}");
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

    tracing::info!("get times. num: {}", times.iter().len());

    let resp = ResponseTimes {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        times,
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

#[derive(Deserialize)]
struct CreateTimesRequest {
    title: String,
}

async fn create_times(
    ctx: web::Data<Context>,
    req: web::Json<CreateTimesRequest>,
) -> impl Responder {
    let mut store = ctx.store.lock().await; //.unwrap();
    let times = match store.create_times(req.title.clone()).await {
        Ok(times) => times,
        Err(e) => {
            tracing::info!("failed to create title: {e}");
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

    tracing::info!(
        "create times with title: {}, id: {}",
        &req.title,
        &times.id
    );
    #[derive(Serialize)]
    struct Response {
        base: ResponseBase,
        times: Times,
    }

    let resp = Response {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        times,
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

async fn delete_times(
    ctx: web::Data<Context>,
    path: web::Path<u64>,
) -> impl Responder {
    let tid = path.into_inner();
    let mut store = ctx.store.lock().await;

    match store.delete_times(tid).await {
        Ok(()) => {}
        Err(e) => {
            tracing::info!("failed to delete times: {e}");
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    }
    /* TODO: asynchronous delete */

    tracing::info!("mark as delete the times {}", tid);
    let resp = ResponseBase {
        status: 0,
        text: "Ok".to_string(),
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

#[derive(Serialize)]
struct GetPostResponse {
    base: ResponseBase,
    posts: Vec<Post>,
}

async fn get_posts(
    ctx: web::Data<Context>,
    path: web::Path<u64>,
) -> impl Responder {
    let tid = path.into_inner();

    let mut store = ctx.store.lock().await;
    let posts = match store.get_posts(tid).await {
        Ok(posts) => posts,
        Err(e) => {
            tracing::info!("failed to get posts for times {}: {}", tid, &e);
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

    tracing::info!("get posts ({}) for times {}", posts.iter().len(), tid);

    let resp = GetPostResponse {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        posts,
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

#[derive(Serialize)]
struct PostPostResponse {
    base: ResponseBase,
    pid: u64,
}

#[derive(Deserialize)]
struct PostPostRequest {
    post: String,
}

async fn post_post(
    ctx: web::Data<Context>,
    path: web::Path<u64>,
    req: web::Json<PostPostRequest>,
) -> impl Responder {
    let tid = path.into_inner();
    let post = req.post.clone();

    let mut store = ctx.store.lock().await;
    let post = match store.create_post(tid, post).await {
        Ok(post) => post,
        Err(e) => {
            tracing::info!("failed to create a post for times {}: {}", tid, &e);
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

    tracing::info!(
        "create a post ({}, {}) for times {}",
        post.id,
        post.post,
        tid
    );

    let resp = PostPostResponse {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        pid: post.id,
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}
