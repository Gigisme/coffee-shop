use actix_web::web;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm_migration::DbErr;

use crate::{db, entities};

pub async fn insert(item: web::Json<entities::item::ModelInsert>) -> Result<&'static str, DbErr> {
    let db = db::get_connection().await;

    let item = entities::item::ActiveModel {
        name: Set(item.name.clone()),
        price: Set(item.price),
        weight: Set(item.weight),
        description: Set(None),
        rating: Set(0.0),
        ..Default::default()
    };

    match db {
        Ok(db) => match item.insert(&db).await {
            Ok(_) => Ok("Inserted successfully"),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

pub async fn get_items() -> Vec<entities::item::Model> {
    let db = db::get_connection().await.unwrap();

    let items = entities::item::Entity::find().all(&db).await;
    items.unwrap()
}
