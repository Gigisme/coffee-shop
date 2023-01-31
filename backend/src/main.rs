use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod controllers;
mod db;
mod entities;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/")]
async fn submit(insert_item: web::Json<entities::item::Model>) -> String {
    controllers::item::insert(insert_item).await;
    "successful insert".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(submit))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
