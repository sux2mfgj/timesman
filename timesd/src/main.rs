use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
struct AppContext {
    db: SqlitePool,
}

#[derive(Serialize)]
struct ResponseBase{
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
    updated_at: Option<chrono::NaiveDateTime>
}

async fn get_times(ctx: web::Data<AppContext>) -> impl Responder {

    let times = sqlx::query_as!(Times, r#"select * from times"#)
        .fetch_all(&ctx.db).await.unwrap();

    let resp = ResponseTimes {
        base: ResponseBase{
            status: 0,
            text: "Ok".to_string(),
        },
        times: times,
    };

    HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("./database.db").await.unwrap();

    let ctx= AppContext{ db: pool.clone() };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .route("/times", web::get().to(get_times))
            //.route("/times", web::post().to(create_times))
            //.route("/times/{tid}", web::get().to(get_posts))
            //.route("/times/{tid}", web::post().to(post_post))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
