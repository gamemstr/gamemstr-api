use super::super::super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::world::campaign::session::Session;
use rocket::{
    delete, get, post,
    response::status::{Accepted, Created, NotFound},
    serde::json::Json,
    Either,
};
use rocket_dyn_templates::{context, Template};

#[get("/worlds/<_world_id>/campaigns/<campaign_id>/sessions")]
pub fn list_sessions(_world_id: String, campaign_id: String) -> Template {
    let connection = &mut crate::services::establish_connection_pg();
    let sessions = schema::sessions::dsl::sessions
        .filter(schema::sessions::dsl::campaign_id.eq(campaign_id))
        .load::<models::worlds::campaigns::sessions::Session>(connection)
        .expect("Error loading sessions")
        .iter()
        .map(|session| session.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "sessions",
        context! {
            sessions: &sessions,
            count: sessions.len()
        },
    )
}

#[get("/worlds/<_world_id>/campaigns/<_campaign_id>/sessions/<id>")]
pub fn get_session(_world_id: String, _campaign_id: String, id: String) -> Result<Json<Session>> {
    let connection = &mut crate::services::establish_connection_pg();
    let session = schema::sessions::dsl::sessions
        .find(id)
        .first::<models::worlds::campaigns::sessions::Session>(connection)
        .expect("Error loading session")
        .to_entity();
    Ok(Json(session))
}

#[delete("/worlds/<_world_id>/campaigns/<_campaign_id>/sessions/<id>")]
pub fn delete_session(_world_id: String, _campaign_id: String, id: String) -> Result<Json<Session>> {
    let connection = &mut crate::services::establish_connection_pg();
    let session = schema::sessions::dsl::sessions
        .find(&id)
        .first::<models::worlds::campaigns::sessions::Session>(connection)
        .expect("Error loading session")
        .to_entity();
    diesel::delete(schema::sessions::dsl::sessions.find(&id))
        .execute(connection)
        .expect("Error deleting session");
    Ok(Json(session))
}

#[post(
    "/worlds/<_world_id>/campaigns/<_campaign_id>/sessions/add",
    rank = 1,
    format = "json",
    data = "<session>"
)]
pub fn create_session(
    _world_id: String,
    _campaign_id: String,
    session: Json<Session>,
) -> Result<Created<Json<Session>>> {
    let connection = &mut crate::services::establish_connection_pg();
    let new_session = models::worlds::campaigns::sessions::Session::new(session.clone().0);
    diesel::insert_into(schema::sessions::dsl::sessions)
        .values(&new_session)
        .execute(connection)
        .expect("Error creating session");
    Ok(Created::new("/").body(session))
}

#[post(
    "/worlds/<_world_id>/campaigns/<_campaign_id>/sessions/<id>",
    rank = 2,
    format = "json",
    data = "<session>"
)]
pub fn update_session(
    _world_id: String,
    _campaign_id: String,
    id: String,
    session: Json<Session>,
) -> Either<Result<Accepted<Json<Session>>>, Result<NotFound<String>>> {
    let connection = &mut crate::services::establish_connection_pg();
    let new_session = models::worlds::campaigns::sessions::Session::new(session.clone().0);
    let result = diesel::update(schema::sessions::dsl::sessions.find(&id))
        .set(&new_session)
        .execute(connection);
    match result {
        Ok(_) => Either::Left(Ok(Accepted(session))),
        Err(_) => Either::Right(Ok(NotFound(format!("Session not found with id {}", id)))),
    }
}
