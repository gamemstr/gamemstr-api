use super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::spell::Spell;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
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
pub fn get_spell(id: String) -> Result<Json<Spell>> {
    let connection = &mut super::establish_connection_pg();
    let spell = schema::spells::dsl::spells
        .find(id)
        .first::<models::spells::Spell>(connection)
        .expect("Error loading spell")
        .to_entity();
    Ok(Json(spell))
}

#[delete("/spells/<id>")]
pub fn delete_spell(id: String) -> Result<Json<Spell>> {
    let connection = &mut super::establish_connection_pg();
    let spell = schema::spells::dsl::spells
        .find(&id)
        .first::<models::spells::Spell>(connection)
        .expect("Error loading spell")
        .to_entity();
    diesel::delete(schema::spells::dsl::spells.find(&id))
        .execute(connection)
        .expect("Error deleting spell");
    Ok(Json(spell))
}

#[post("/spells/add", format = "json", data = "<spell>")]
pub fn create_spell(spell: Json<Spell>) -> Result<Created<Json<Spell>>> {
    let connection = &mut super::establish_connection_pg();

    let new_spell = models::spells::Spell::new(spell.clone().0);

    diesel::insert_into(schema::spells::dsl::spells)
        .values(&new_spell)
        .execute(connection)
        .expect("Error saving new spell");
    Ok(Created::new("/").body(spell))
}
