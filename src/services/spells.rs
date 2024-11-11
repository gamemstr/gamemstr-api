use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::spell::Spell;
use rocket::{
    delete, get, post,
    response::status::{Accepted, BadRequest, Created, NotFound},
    serde::json::Json,
};
use rocket_dyn_templates::{context, Template};

#[get("/spells")]
pub fn list_spells() -> Template {
    let connection = &mut super::establish_connection_pg();
    let spells = schema::spells::dsl::spells
        .load::<models::spells::Spell>(connection)
        .expect("Error loading spells")
        .iter()
        .map(|spell| spell.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "spells",
        context! {
            spells: &spells,
            count: spells.len()
        },
    )
}

#[get("/spells/<id>")]
pub fn get_spell(id: String) -> Result<Json<Spell>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let spell = schema::spells::dsl::spells
        .find(id)
        .first::<models::spells::Spell>(connection);
    match spell {
        Ok(spell) => Ok(Json(spell.to_entity())),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[delete("/spells/<id>")]
pub fn delete_spell(id: String) -> Result<Json<Spell>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let spell = schema::spells::dsl::spells
        .find(&id)
        .first::<models::spells::Spell>(connection);

    let result = diesel::delete(schema::spells::dsl::spells.find(&id))
        .execute(connection);

    match result {
        Ok(_) => {
            match spell {
                Ok(spell) => Ok(Json(spell.to_entity())),
                Err(e) => Err(NotFound(e.to_string())),
            }
        }
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[post("/spells/add", format = "json", data = "<spell>")]
pub fn create_spell(spell: Json<Spell>) -> Result<Created<Json<Spell>>, BadRequest<String>> {
    let connection = &mut super::establish_connection_pg();

    let new_spell = models::spells::Spell::new(spell.clone().0);

    let result = diesel::insert_into(schema::spells::dsl::spells)
        .values(&new_spell)
        .execute(connection);
    
    match result {
        Ok(_) => Ok(Created::new(format!("/spells/{}", new_spell.id)).body(Json(new_spell.to_entity()))),
        Err(e) => Err(BadRequest(e.to_string())),
    }
}

#[post("/spells/<id>", format = "json", data = "<spell>")]
pub fn update_spell(
    id: String,
    spell: Json<Spell>,
) -> Result<Accepted<Json<Spell>>, NotFound<String>> {
    let connection = &mut super::establish_connection_pg();
    let result = diesel::update(schema::spells::dsl::spells.find(&id))
        .set(models::spells::Spell::new(spell.clone().0))
        .execute(connection);
    
    match result {
        Ok(_) => Ok(Accepted(spell)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
