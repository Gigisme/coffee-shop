use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use sea_orm::{ActiveModelTrait, Database, Set};

mod db;
mod entities;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("sqlite:./sqlite.db?mode=rwc")
        .await
        .unwrap();

    let item = entities::item::ActiveModel {
        name: Set("test".to_owned()),
        price: Set(0.0),
        weight: Set(0.0),
        rating: Set(0.0),
        description: Set(None),
        ..Default::default()
    };

    let test_item: entities::item::Model = item.insert(&db).await.unwrap();

    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
