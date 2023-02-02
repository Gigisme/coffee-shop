use actix_cors::Cors;
use actix_web::{http, App, HttpServer};

mod controllers;
mod db;
mod entities;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(routes::submit)
            .service(routes::get_items)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
