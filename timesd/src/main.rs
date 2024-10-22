use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

use sqlx::sqlite::SqlitePool;

async fn get_list(data: web::Data<AppState>) -> impl Responder {
    let comments = sqlx::query_as!(Comment, "select * from comments")
        .fetch_all(&data.db)
        .await
        .unwrap();

    println!("Received a request to get list");
    HttpResponse::Ok().body(serde_json::to_string(&comments).unwrap())
}

#[derive(Deserialize)]
struct Info {
    comment: String,
}

async fn post_append(data: web::Data<AppState>, info: web::Json<Info>) -> impl Responder {
    sqlx::query!(
        r#"
        insert into comments("comment") values ($1)

        "#,
        info.comment
    )
    .execute(&data.db)
    .await
    .unwrap();

    println!("Received a request {0}", info.comment);

    HttpResponse::Ok().body("hello, world!")
}

#[derive(Deserialize)]
struct DeleteInfo {
    id: i64,
}

async fn delete_post(data: web::Data<AppState>, info: web::Json<DeleteInfo>) -> impl Responder {
    sqlx::query!(
        r#"
        delete from comments where (id) = $1
        "#,
        info.id
    )
    .execute(&data.db)
    .await
    .unwrap();

    HttpResponse::Ok().body("ok")
}

#[derive(Debug, Serialize)]
pub struct Comment {
    pub id: i64,
    pub comment: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("./database.db").await.unwrap();

    let appdata = AppState { db: pool.clone() };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(appdata.clone()))
            .route("/post", web::post().to(post_append))
            .route("/post", web::delete().to(delete_post))
            .route("/list", web::get().to(get_list))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
