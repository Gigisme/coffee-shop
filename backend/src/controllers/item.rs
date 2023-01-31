use actix_web::web;
use sea_orm::{ActiveModelTrait, Set};

use crate::{db, entities};

pub async fn insert(item: web::Json<entities::item::Model>) {
    let item_clone = item.clone();
    let item = entities::item::ActiveModel {
        name: Set(item_clone.name),
        price: Set(item.price),
        weight: Set(item.weight),
        rating: Set(item.rating),
        description: Set(None),
        ..Default::default()
    };

    let db = db::get_connection().await.unwrap();

    item.insert(&db).await.unwrap();
}
