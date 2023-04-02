use super::super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::world::location::Location;
use rocket::{
    delete, get, post,
    response::status::{Accepted, Created, NotFound},
    serde::json::Json,
    Either,
};
use rocket_dyn_templates::{context, Template};

#[get("/worlds/<id>/locations")]
pub fn list_locations(id: String) -> Template {
    let connection = &mut super::super::establish_connection_pg();
    let locations = schema::locations::dsl::locations
        .filter(schema::locations::dsl::world_id.eq(id))
        .load::<models::worlds::locations::Location>(connection)
        .expect("Error loading locations")
        .iter()
        .map(|location| location.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "locations",
        context! {
            locations: &locations,
            count: locations.len()
        },
    )
}

#[get("/worlds/<_world_id>/locations/<id>")]
pub fn get_location(_world_id: String, id: String) -> Result<Json<Location>> {
    let connection = &mut super::super::establish_connection_pg();
    let location = schema::locations::dsl::locations
        .find(id)
        .first::<models::worlds::locations::Location>(connection)
        .expect("Error loading location")
        .to_entity();
    Ok(Json(location))
}

#[delete("/worlds/<_world_id>/locations/<id>")]
pub fn delete_location(_world_id: String, id: String) -> Result<Json<Location>> {
    let connection = &mut super::super::establish_connection_pg();
    let location = schema::locations::dsl::locations
        .find(&id)
        .first::<models::worlds::locations::Location>(connection)
        .expect("Error loading location")
        .to_entity();
    diesel::delete(schema::locations::dsl::locations.find(&id))
        .execute(connection)
        .expect("Error deleting location");
    Ok(Json(location))
}

#[post(
    "/worlds/<_world_id>/locations/add",
    format = "json",
    data = "<location>"
)]
pub fn create_location(
    _world_id: String,
    location: Json<Location>,
) -> Result<Created<Json<Location>>> {
    let connection = &mut super::super::establish_connection_pg();
    let new_location = models::worlds::locations::Location::new(location.clone().0);
    diesel::insert_into(schema::locations::dsl::locations)
        .values(&new_location)
        .execute(connection)
        .expect("Error creating location");
    Ok(Created::new("/").body(location))
}

#[post(
    "/worlds/<_world_id>/locations/<id>",
    format = "json",
    data = "<location>"
)]
pub fn update_location(
    _world_id: String,
    id: String,
    location: Json<Location>,
) -> Either<Result<Accepted<Json<Location>>>, Result<NotFound<String>>> {
    let connection = &mut super::super::establish_connection_pg();
    let result = diesel::update(schema::locations::dsl::locations.find(&id))
        .set(&models::worlds::locations::Location::new(
            location.clone().0,
        ))
        .execute(connection);
    match result {
        Ok(_) => Either::Left(Ok(Accepted(Some(location)))),
        Err(_) => Either::Right(Ok(NotFound(format!("Location not found with id {}", id)))),
    }
}
