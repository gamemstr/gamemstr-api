use super::Result;
use crate::{models, schema};
use diesel::prelude::*;
use gamemstr_common::spell::Spell;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket_dyn_templates::{context, Template};

#[get("/spells")]
pub fn list_spells() -> Template {
    use models::spells::Spell;
    let connection = &mut super::establish_connection_pg();
    let spells = schema::spells::dsl::spells
        .load::<Spell>(connection)
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
    use models::spells::Spell;
    let connection = &mut super::establish_connection_pg();
    let spell = schema::spells::dsl::spells
        .find(id)
        .first::<Spell>(connection)
        .expect("Error loading spell")
        .to_entity();
    Ok(Json(spell))
}

#[delete("/spells/<id>")]
pub fn delete_spell(id: String) -> Result<Json<Spell>> {
    use models::spells::Spell;
    let connection = &mut super::establish_connection_pg();
    let spell = schema::spells::dsl::spells
        .find(&id)
        .first::<Spell>(connection)
        .expect("Error loading spell")
        .to_entity();
    diesel::delete(schema::spells::dsl::spells.find(&id))
        .execute(connection)
        .expect("Error deleting spell");
    Ok(Json(spell))
}

#[post("/spells/add", format = "json", data = "<spell>")]
pub fn create_spell(spell: Json<Spell>) -> Result<Created<Json<Spell>>> {
    use models::spells::Spell;
    let connection = &mut super::establish_connection_pg();

    let new_spell = Spell::new(spell.clone().0);

    diesel::insert_into(schema::spells::dsl::spells)
        .values(&new_spell)
        .execute(connection)
        .expect("Error saving new spell");
    Ok(Created::new("/").body(spell))
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_list_spells() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/spells").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_html())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Spells"));
    }

    #[test]
    fn test_get_spell() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/spells/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Test Spell"));
    }

    #[test]
    fn test_delete_spell() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let spell = gamemstr_common::spell::Spell {
            id: "258759802792856926525".to_string(),
            name: "Test Spell".to_string(),
            description: "Test Description".to_string(),
            level: gamemstr_common::spell::SpellLevel::Cantrip,
            casting_time: gamemstr_common::spell::CastingTime::Action,
            duration: gamemstr_common::spell::Duration::Instantaneous,
            damage: None,
            range: gamemstr_common::spell::SpellRange::Touch,
            area: None,
            damage_type: None,
            components: gamemstr_common::spell::Components::V,
            attack_bonus: Some(4),
            save: None,
        };
        client
            .post("/spells/new")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&spell).unwrap())
            .dispatch();
        let response = client.delete("/spells/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Test World"));
    }

    #[test]
    fn test_create_spell() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        client.delete("/spells/258759802792856926525").dispatch();
        let spell = gamemstr_common::spell::Spell {
            id: "258759802792856926525".to_string(),
            name: "Test Spell".to_string(),
            description: "Test Description".to_string(),
            level: gamemstr_common::spell::SpellLevel::Cantrip,
            casting_time: gamemstr_common::spell::CastingTime::Action,
            duration: gamemstr_common::spell::Duration::Instantaneous,
            damage: None,
            range: gamemstr_common::spell::SpellRange::Touch,
            area: None,
            damage_type: None,
            components: gamemstr_common::spell::Components::V,
            attack_bonus: Some(4),
            save: None,
        };
        let response = client
            .post("/spells/add")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&spell).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        println!("{}", body);
        assert_eq!(body, serde_json::to_string(&spell).unwrap());
    }
}
