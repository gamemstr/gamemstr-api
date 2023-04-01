use super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::creature::Creature;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket::{
    response::status::{Accepted, Created, NotFound},
    Either,
};
use rocket_dyn_templates::{context, Template};

#[get("/creatures")]
pub fn list_creatures() -> Template {
    let connection = &mut super::establish_connection_pg();
    let creatures = schema::creatures::dsl::creatures
        .load::<models::creatures::Creature>(connection)
        .expect("Error loading creatures")
        .iter()
        .map(|creature| creature.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "creatures",
        context! {
            creatures: &creatures,
            count: creatures.len()
        },
    )
}

#[get("/creatures/<id>")]
pub fn get_creature(id: String) -> Result<Json<Creature>> {
    let connection = &mut super::establish_connection_pg();
    let creature = schema::creatures::dsl::creatures
        .find(id)
        .first::<models::creatures::Creature>(connection)
        .expect("Error loading creature")
        .to_entity();
    Ok(Json(creature))
}

#[delete("/creatures/<id>")]
pub fn delete_creature(id: String) -> Result<Json<Creature>> {
    let connection = &mut super::establish_connection_pg();
    let creature = schema::creatures::dsl::creatures
        .find(&id)
        .first::<models::creatures::Creature>(connection)
        .expect("Error loading creature")
        .to_entity();
    diesel::delete(schema::creatures::dsl::creatures.find(&id))
        .execute(connection)
        .expect("Error deleting creature");
    Ok(Json(creature))
}

#[post("/creatures/add", format = "json", data = "<creature>")]
pub fn create_creature(creature: Json<Creature>) -> Result<Created<Json<Creature>>> {
    let connection = &mut super::establish_connection_pg();

    let new_creature = models::creatures::Creature::new(creature.clone().0);
    diesel::insert_into(schema::creatures::dsl::creatures)
        .values(&new_creature)
        .execute(connection)
        .expect("Error saving new creature");
    Ok(Created::new("/creatures").body(creature))
}

#[post("/creatures/<id>", format = "json", data = "<creature>")]
pub fn update_creature(
    id: String,
    creature: Json<Creature>,
) -> Either<Result<Accepted<Json<Creature>>>, Result<NotFound<String>>> {
    let connection = &mut super::establish_connection_pg();
    let result = diesel::update(schema::creatures::dsl::creatures.find(&id))
        .set(&models::creatures::Creature::new(creature.clone().0))
        .execute(connection);
    match result {
        Ok(_) => Either::Left(Ok(Accepted(Some(creature)))),
        Err(_) => Either::Right(Ok(NotFound(format!("Creature with id {} not found", id)))),
    }
}
