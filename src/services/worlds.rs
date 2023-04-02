use super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::world::World;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket::{
    response::status::{Accepted, Created, NotFound},
    Either,
};
use rocket_dyn_templates::{context, Template};

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
pub fn get_world(id: String) -> Result<Json<World>> {
    let connection = &mut super::establish_connection_pg();
    let world = schema::worlds::dsl::worlds
        .find(id)
        .first::<models::worlds::World>(connection)
        .expect("Error loading world")
        .to_entity();
    Ok(Json(world))
}

#[delete("/worlds/<id>")]
pub fn delete_world(id: String) -> Result<Json<World>> {
    let connection = &mut super::establish_connection_pg();
    let world = schema::worlds::dsl::worlds
        .find(&id)
        .first::<models::worlds::World>(connection)
        .expect("Error loading world")
        .to_entity();
    diesel::delete(schema::worlds::dsl::worlds.find(&id))
        .execute(connection)
        .expect("Error deleting world");
    Ok(Json(world))
}

#[post("/worlds/add", format = "json", data = "<world>")]
pub fn create_world(world: Json<World>) -> Result<Created<Json<World>>> {
    let connection = &mut super::establish_connection_pg();

    let new_world = models::worlds::World::new(world.clone().0);

    diesel::insert_into(schema::worlds::dsl::worlds)
        .values(&new_world)
        .execute(connection)
        .expect("Error saving new world");
    Ok(Created::new("/").body(world))
}

#[post("/worlds/<id>", format = "json", data = "<world>")]
pub fn update_world(
    id: String,
    world: Json<World>,
) -> Either<Result<Accepted<Json<World>>>, Result<NotFound<String>>> {
    let connection = &mut super::establish_connection_pg();
    let result = diesel::update(schema::worlds::dsl::worlds.find(&id))
        .set(models::worlds::World::new(world.clone().0))
        .execute(connection);
    match result {
        Ok(_) => Either::Left(Ok(Accepted(Some(world)))),
        Err(_) => Either::Right(Ok(NotFound(id))),
    }
}
