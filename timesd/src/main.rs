use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
struct AppContext {
    db: SqlitePool,
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

#[derive(Serialize)]
struct Times {
    id: i64,
    title: String,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
    flags: i64,
}

async fn get_times(ctx: web::Data<AppContext>) -> impl Responder {
    let times =
        sqlx::query_as!(Times, r#"select * from times where flags = 0"#)
            .fetch_all(&ctx.db)
            .await
            .unwrap();

    //let times = all_times.into_iter().filter(|t| t.exists()).collect();

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
    ctx: web::Data<AppContext>,
    req: web::Json<CreateTimesRequest>,
) -> impl Responder {
    let times = sqlx::query_as!(
        Times,
        r#"insert into times("title") values ($1) returning *"#,
        req.title
    )
    .fetch_one(&ctx.db)
    .await
    .unwrap();

    #[derive(Serialize)]
    struct ResponseOneTimes {
        base: ResponseBase,
        times: Times,
    }

    let resp = ResponseOneTimes {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        times,
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

async fn delete_times(
    ctx: web::Data<AppContext>,
    path: web::Path<i64>,
) -> impl Responder {
    let tid = path.into_inner();

    sqlx::query!(r#"update times set flags = 1 where id = $1"#, tid)
        .execute(&ctx.db)
        .await
        .unwrap();

    /* TODO: asynchronous delete
    sqlx::query!(r#"delete from posts where times_id = $1"#, tid)
        .execute(&ctx.db)
        .await
        .unwrap();
    */

    let resp = ResponseBase {
        status: 0,
        text: "Ok".to_string(),
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

#[derive(Serialize)]
struct Post {
    id: i64,
    times_id: i64,
    post: String,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
struct GetPostResponse {
    base: ResponseBase,
    posts: Vec<Post>,
}

async fn get_posts(
    ctx: web::Data<AppContext>,
    path: web::Path<i64>,
) -> impl Responder {
    let tid = path.into_inner();
    let posts = sqlx::query_as!(
        Post,
        r#"select * from posts where times_id = $1"#,
        tid
    )
    .fetch_all(&ctx.db)
    .await
    .unwrap();

    let resp = GetPostResponse {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        posts: posts,
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
    ctx: web::Data<AppContext>,
    path: web::Path<i64>,
    req: web::Json<PostPostRequest>,
) -> impl Responder {
    let tid = path.into_inner();
    //TODO: use query_as, but I have to fix an issue related the following link:
    // https://docs.rs/sqlx/latest/sqlx/macro.query_as.html#troubleshooting-error-mismatched-types
    let pid = sqlx::query!(
        r#"insert into posts(times_id, post) values ($1, $2) returning id"#,
        tid,
        req.post
    )
    .fetch_one(&ctx.db)
    .await
    .unwrap();

    let resp = PostPostResponse {
        base: ResponseBase {
            status: 0,
            text: "Ok".to_string(),
        },
        pid: pid.id.unwrap(),
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("./database.db").await.unwrap();

    let ctx = AppContext { db: pool.clone() };

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
