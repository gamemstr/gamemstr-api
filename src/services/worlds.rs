use super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::world::World;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket_dyn_templates::{context, Template};

#[get("/worlds")]
pub fn list_worlds() -> Template {
    use models::worlds::World;
    let connection = &mut super::establish_connection_pg();
    let worlds = schema::worlds::dsl::worlds
        .load::<World>(connection)
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
    use models::worlds::World;
    let connection = &mut super::establish_connection_pg();
    let world = schema::worlds::dsl::worlds
        .find(id)
        .first::<World>(connection)
        .expect("Error loading world")
        .to_entity();
    Ok(Json(world))
}

#[delete("/worlds/<id>")]
pub fn delete_world(id: String) -> Result<Json<World>> {
    use models::worlds::World;
    let connection = &mut super::establish_connection_pg();
    let world = schema::worlds::dsl::worlds
        .find(&id)
        .first::<World>(connection)
        .expect("Error loading world")
        .to_entity();
    diesel::delete(schema::worlds::dsl::worlds.find(&id))
        .execute(connection)
        .expect("Error deleting world");
    Ok(Json(world))
}

#[post("/worlds/add", format = "json", data = "<world>")]
pub fn create_world(world: Json<World>) -> Result<Created<Json<World>>> {
    use models::worlds::World;
    let connection = &mut super::establish_connection_pg();

    let new_world = World::new(world.clone().0);

    diesel::insert_into(schema::worlds::dsl::worlds)
        .values(&new_world)
        .execute(connection)
        .expect("Error saving new world");
    Ok(Created::new("/").body(world))
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_list_worlds() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/worlds").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_html())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Worlds"));
    }

    #[test]
    fn test_get_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/worlds/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Test World"));
    }

    #[test]
    fn test_delete_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let world = gamemstr_common::world::World {
            id: "258759802792856926525".to_string(),
            name: "Test World".to_string(),
            description: "Test Description".to_string(),
        };
        client
            .post("/worlds/new")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&world).unwrap())
            .dispatch();
        let response = client.delete("/worlds/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Test World"));
    }

    #[test]
    fn test_create_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        client.delete("/worlds/258759802792856926525").dispatch();
        let world = gamemstr_common::world::World {
            id: "258759802792856926525".to_string(),
            name: "Test World".to_string(),
            description: "Test Description".to_string(),
        };
        let response = client
            .post("/worlds/add")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&world).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        println!("{}", body);
        assert_eq!(body, serde_json::to_string(&world).unwrap());
    }
}
