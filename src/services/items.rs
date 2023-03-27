use super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::item::Item;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket_dyn_templates::{context, Template};

#[get("/items")]
pub fn list_items() -> Template {
    let connection = &mut super::establish_connection_pg();
    let items = schema::items::dsl::items
        .load::<models::items::Item>(connection)
        .expect("Error loading items")
        .iter()
        .map(|item| item.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "items",
        context! {
            items: &items,
            count: items.len()
        },
    )
}

#[get("/items/<id>")]
pub fn get_item(id: String) -> Result<Json<Item>> {
    let connection = &mut super::establish_connection_pg();
    let item = schema::items::dsl::items
        .find(id)
        .first::<models::items::Item>(connection)
        .expect("Error loading item")
        .to_entity();
    Ok(Json(item))
}

#[delete("/items/<id>")]
pub fn delete_item(id: String) -> Result<Json<Item>> {
    let connection = &mut super::establish_connection_pg();
    let item = schema::items::dsl::items
        .find(&id)
        .first::<models::items::Item>(connection)
        .expect("Error loading item")
        .to_entity();
    diesel::delete(schema::items::dsl::items.find(&id))
        .execute(connection)
        .expect("Error deleting item");
    Ok(Json(item))
}

#[post("/items/add", format = "json", data = "<item>")]
pub fn create_item(item: Json<Item>) -> Result<Created<Json<Item>>> {
    let connection = &mut super::establish_connection_pg();

    let new_item = models::items::Item::new(item.clone().0);

    diesel::insert_into(schema::items::dsl::items)
        .values(&new_item)
        .execute(connection)
        .expect("Error saving new item");
    Ok(Created::new("/").body(item))
}
