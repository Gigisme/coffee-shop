use crate::{controllers, entities};
use actix_web::{get, post, web};

#[post("/insertItem")]
async fn submit(insert_item: web::Json<entities::item::ModelNoId>) -> String {
    controllers::item::insert(insert_item).await;
    "successful insert".to_string()
}
