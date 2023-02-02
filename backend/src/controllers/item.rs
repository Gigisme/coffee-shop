use actix_web::web;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::{db, entities};

pub async fn insert(item: web::Json<entities::item::ModelNoId>) {
    let item = entities::item::ActiveModel {
        name: Set(item.name.clone()),
        price: Set(item.price),
        weight: Set(item.weight),
        rating: Set(item.rating),
        description: Set(None),
        ..Default::default()
    };

    let db = db::get_connection().await.unwrap();

    item.insert(&db).await.unwrap();
}

pub async fn get_items() -> Vec<entities::item::Model> {
    let db = db::get_connection().await.unwrap();

    let items = entities::item::Entity::find().all(&db).await;
    items.unwrap()
}
