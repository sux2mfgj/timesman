use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

use std::sync::Arc;
use std::sync::Mutex;
use store::sqlite3::SqliteStoreBuilder;
use store::{Post, Store, Times};

#[derive(Clone)]
struct Context {
    store: Arc<Mutex<dyn Store>>,
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
    let store = ctx.store.lock().unwrap();
    let times = match store.get_times().await {
        Ok(times) => times,
        Err(e) => {
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

    println!("/get_times");

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
    let mut store = ctx.store.lock().unwrap();
    let times = match store.create_times(req.title.clone()).await {
        Ok(times) => times,
        Err(e) => {
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

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
    path: web::Path<i64>,
) -> impl Responder {
    let tid = path.into_inner();
    let mut store = ctx.store.lock().unwrap();

    match store.delete_times(tid).await {
        Ok(()) => {}
        Err(e) => {
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    }
    /* TODO: asynchronous delete */

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
    path: web::Path<i64>,
) -> impl Responder {
    let tid = path.into_inner();

    let store = ctx.store.lock().unwrap();
    let posts = match store.get_posts(tid).await {
        Ok(posts) => posts,
        Err(e) => {
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

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
    pid: i64,
}

#[derive(Deserialize)]
struct PostPostRequest {
    post: String,
}

async fn post_post(
    ctx: web::Data<Context>,
    path: web::Path<i64>,
    req: web::Json<PostPostRequest>,
) -> impl Responder {
    let tid = path.into_inner();
    let post = req.post.clone();

    let mut store = ctx.store.lock().unwrap();
    let post = match store.create_post(tid, post).await {
        Ok(post) => post,
        Err(e) => {
            let resp = ResponseBase { status: 1, text: e };

            return HttpResponse::Ok()
                .body(serde_json::to_string(&resp).unwrap());
        }
    };

    let resp = PostPostResponse {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        pid: post.id,
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = SqliteStoreBuilder::new("./database.db");
    let store = store.build().await.unwrap();

    let ctx = Context {
        store: Arc::new(Mutex::new(store)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .route("/times", web::get().to(get_times))
            .route("/times", web::post().to(create_times))
            .route("/times/{tid}", web::delete().to(delete_times))
            .route("/times/{tid}", web::get().to(get_posts))
            .route("/times/{tid}", web::post().to(post_post))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
