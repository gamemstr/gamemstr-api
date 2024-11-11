use std::sync::Arc;

use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::creature::Creature;
use rocket::{
    delete, get, post,
    response::status::{Accepted, BadRequest, Created, NotFound},
    serde::json::Json,
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
pub fn get_creature(id: String) -> Result<Json<Creature>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let creature = schema::creatures::dsl::creatures
        .find(id)
        .first::<models::creatures::Creature>(connection);
    match creature {
        Ok(creature) => Ok(Json(creature.to_entity())),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[delete("/creatures/<id>")]
pub fn delete_creature(id: String) -> Result<Json<Creature>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let creature = schema::creatures::dsl::creatures
        .find(&id)
        .first::<models::creatures::Creature>(connection);
    let result = diesel::delete(schema::creatures::dsl::creatures.find(&id))
        .execute(connection);
    
    match result {
        Ok(_) => {
            match creature {
                Ok(creature) => Ok(Json(creature.to_entity())),
                Err(e) => Err(NotFound(e.to_string())),
            }
        },
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[post("/creatures/add", format = "json", data = "<creature>")]
pub fn create_creature(creature: Json<Creature>) -> Result<Created<Json<Creature>>, BadRequest<String>> {
    let connection = &mut super::establish_connection_pg();

    let new_creature = models::creatures::Creature::new(creature.clone().0);
    let result = diesel::insert_into(schema::creatures::dsl::creatures)
        .values(&new_creature)
        .execute(connection);

    match result {
        Ok(_) => Ok(Created::new(format!("/creatures/{}", new_creature.id)).body(creature)),
        Err(e) => Err(BadRequest(e.to_string())),
    }
}

#[post("/creatures/<id>", format = "json", data = "<creature>")]
pub fn update_creature(
    id: String,
    creature: Json<Creature>,
) -> Result<Accepted<Json<Creature>>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let result = diesel::update(schema::creatures::dsl::creatures.find(&id))
        .set(&models::creatures::Creature::new(creature.clone().0))
        .execute(connection);
    match result {
        Ok(_) => Ok(Accepted(creature)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
