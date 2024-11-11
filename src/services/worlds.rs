use std::f64::consts::E;

use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::world::World;
use rocket::{
    delete, get, post,
    response::status::{Accepted, BadRequest, Created, NotFound},
    serde::json::Json,
};
use rocket_dyn_templates::{context, Template};

pub mod campaigns;
pub mod locations;

#[get("/worlds")]
pub fn list_worlds() -> Template {
    let connection = &mut super::establish_connection_pg();
    let worlds = schema::worlds::dsl::worlds
        .load::<models::worlds::World>(connection)
        .expect("Error loading worlds")
        .iter()
        .map(|world| world.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "worlds",
        context! {
            worlds: &worlds,
            count: worlds.len()
        },
    )
}

#[get("/worlds/<id>")]
pub fn get_world(id: String) -> Result<Json<World>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let world = schema::worlds::dsl::worlds
        .find(id)
        .first::<models::worlds::World>(connection);
    match world {
        Ok(world) => Ok(Json(world.to_entity())),
        Err(e) => Err(NotFound(e.to_string())),
        
    }
}

#[delete("/worlds/<id>")]
pub fn delete_world(id: String) -> Result<Json<World>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let world = schema::worlds::dsl::worlds
        .find(&id)
        .first::<models::worlds::World>(connection);

    let result = diesel::delete(schema::worlds::dsl::worlds.find(&id))
        .execute(connection);
    
    match result {
        Ok(_) => {
            match world {
                Ok(world) => Ok(Json(world.to_entity())),
                Err(e) => Err(NotFound(e.to_string())),
            }
        },
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[post("/worlds/add", format = "json", data = "<world>")]
pub fn create_world(world: Json<World>) -> Result<Created<Json<World>>, BadRequest<String>> {
    let connection = &mut super::establish_connection_pg();

    let new_world = models::worlds::World::new(world.clone().0);

    let result = diesel::insert_into(schema::worlds::dsl::worlds)
        .values(&new_world)
        .execute(connection);
    
    match result {
        Ok(_) => Ok(Created::new(format!("/worlds/{}", new_world.id)).body(world)),
        Err(e) => Err(BadRequest(e.to_string())),
    }
}

#[post("/worlds/<id>", format = "json", data = "<world>")]
pub fn update_world(
    id: String,
    world: Json<World>,
) -> Result<Accepted<Json<World>>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let result = diesel::update(schema::worlds::dsl::worlds.find(&id))
        .set(models::worlds::World::new(world.clone().0))
        .execute(connection);
    match result {
        Ok(_) => Ok(Accepted(world)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
