use crate::{controllers, entities};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse};

// TODO: make a proper response
#[post("/insertItem")]
async fn submit(insert_item: web::Json<entities::item::ModelNoId>) -> String {
    controllers::item::insert(insert_item).await;
    "successful insert".to_string()
}

#[get("/items")]
async fn get_items() -> HttpResponse {
    let items = controllers::item::get_items().await;

    let body = serde_json::to_string(&items).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}
