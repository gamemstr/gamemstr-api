use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::world::location::Location;
use rocket::{
    delete, get, post,
    response::status::{Accepted, BadRequest, Created, NotFound},
    serde::json::Json,
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
pub fn get_location(_world_id: String, id: String) -> Result<Json<Location>, NotFound<String>> {
    let connection = &mut super::super::establish_connection_pg();
    let location = schema::locations::dsl::locations
        .find(id)
        .first::<models::worlds::locations::Location>(connection);
    
    match location {
        Ok(location) => Ok(Json(location.to_entity())),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[delete("/worlds/<_world_id>/locations/<id>")]
pub fn delete_location(_world_id: String, id: String) -> Result<Json<Location>, NotFound<String>> {
    let connection = &mut super::super::establish_connection_pg();
    let location = schema::locations::dsl::locations
        .find(&id)
        .first::<models::worlds::locations::Location>(connection);
    let result = diesel::delete(schema::locations::dsl::locations.find(&id))
        .execute(connection);
    match result {
        Ok(_) => {
            match location {
                Ok(location) => Ok(Json(location.to_entity())),
                Err(e) => Err(NotFound(e.to_string())),
            }
        },
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[post(
    "/worlds/<_world_id>/locations/add",
    rank = 1,
    format = "json",
    data = "<location>"
)]
pub fn create_location(
    _world_id: String,
    location: Json<Location>,
) -> Result<Created<Json<Location>>, BadRequest<String>> {
    let connection = &mut super::super::establish_connection_pg();
    let new_location = models::worlds::locations::Location::new(location.clone().0);
    let result = diesel::insert_into(schema::locations::dsl::locations)
        .values(&new_location)
        .execute(connection);
    
    match result {
        Ok(_) => Ok(Created::new(format!("/locations/{}", new_location.id)).body(location)),
        Err(e) => Err(BadRequest(e.to_string())),
    }
}

#[post(
    "/worlds/<_world_id>/locations/<id>",
    rank = 2,
    format = "json",
    data = "<location>"
)]
pub fn update_location(
    _world_id: String,
    id: String,
    location: Json<Location>,
) -> Result<Accepted<Json<Location>>, NotFound<String>> {
    let connection = &mut super::super::establish_connection_pg();
    let result = diesel::update(schema::locations::dsl::locations.find(&id))
        .set(&models::worlds::locations::Location::new(
            location.clone().0,
        ))
        .execute(connection);
    match result {
        Ok(_) => Ok(Accepted(location)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
