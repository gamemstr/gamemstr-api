use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::item::Item;
use rocket::{
    delete, get, post,
    response::status::{Accepted, BadRequest, Created, NotFound},
    serde::json::Json,
};
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
pub fn get_item(id: String) -> Result<Json<Item>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let item = schema::items::dsl::items
        .find(id)
        .first::<models::items::Item>(connection);
    match item {
        Ok(item) => Ok(Json(item.to_entity())),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[delete("/items/<id>")]
pub fn delete_item(id: String) -> Result<Json<Item>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let item = schema::items::dsl::items
        .find(&id)
        .first::<models::items::Item>(connection);

    let result = diesel::delete(schema::items::dsl::items.find(&id))
        .execute(connection);
    match result {
        Ok(_) => {
            match item {
                Ok(item) => Ok(Json(item.to_entity())),
                Err(e) => Err(NotFound(e.to_string())),
            }
        },
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[post("/items/add", format = "json", data = "<item>")]
pub fn create_item(item: Json<Item>) -> Result<Created<Json<Item>>, BadRequest<String>> {
    let connection = &mut super::establish_connection_pg();

    let new_item = models::items::Item::new(item.clone().0);

    let result = diesel::insert_into(schema::items::dsl::items)
        .values(&new_item)
        .execute(connection);
    
    match result {
        Ok(_) => Ok(Created::new(format!("items/{}", new_item.id)).body(item)),
        Err(e) => Err(BadRequest(e.to_string())),
    }
}

#[post("/items/<id>", format = "json", data = "<item>")]
pub fn update_item(
    id: String,
    item: Json<Item>,
) -> Result<Accepted<Json<Item>>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let result = diesel::update(schema::items::dsl::items.find(&id))
        .set(models::items::Item::new(item.clone().0))
        .execute(connection);
    match result {
        Ok(_) => Ok(Accepted(item)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
