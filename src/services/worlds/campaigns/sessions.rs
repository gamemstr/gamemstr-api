use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::world::campaign::session::Session;
use rocket::{
    delete, get, post,
    response::status::{Accepted, BadRequest, Created, NotFound},
    serde::json::Json,
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
pub fn get_session(_world_id: String, _campaign_id: String, id: String) -> Result<Json<Session>, NotFound<String>> {
    let connection = &mut crate::services::establish_connection_pg();
    let session = schema::sessions::dsl::sessions
        .find(id)
        .first::<models::worlds::campaigns::sessions::Session>(connection);
    match session {
        Ok(session) => Ok(Json(session.to_entity())),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[delete("/worlds/<_world_id>/campaigns/<_campaign_id>/sessions/<id>")]
pub fn delete_session(_world_id: String, _campaign_id: String, id: String) -> Result<Json<Session>, NotFound<String>> {
    let connection = &mut crate::services::establish_connection_pg();
    let session = schema::sessions::dsl::sessions
        .find(&id)
        .first::<models::worlds::campaigns::sessions::Session>(connection);
    let result = diesel::delete(schema::sessions::dsl::sessions.find(&id))
        .execute(connection);
    match result {
        Ok(_) => {
            match session {
                Ok(session) => Ok(Json(session.to_entity())),
                Err(e) => Err(NotFound(e.to_string())),
            }
        },
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[post(
    "/worlds/<world_id>/campaigns/<campaign_id>/sessions/add",
    rank = 1,
    format = "json",
    data = "<session>"
)]
pub fn create_session(
    world_id: String,
    campaign_id: String,
    session: Json<Session>,
) -> Result<Created<Json<Session>>, BadRequest<String>> {
    let connection = &mut crate::services::establish_connection_pg();
    let new_session = models::worlds::campaigns::sessions::Session::new(session.clone().0);
    let result = diesel::insert_into(schema::sessions::dsl::sessions)
        .values(&new_session)
        .execute(connection);
    match result {
        Ok(_) => Ok(Created::new(format!("/worlds/{}/campaigns/{}/sessions/{}", world_id, campaign_id, new_session.id)).body(session)),
        Err(e) => Err(BadRequest(e.to_string())),
    }
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
) -> Result<Accepted<Json<Session>>, NotFound<String>> {
    let connection = &mut crate::services::establish_connection_pg();
    let new_session = models::worlds::campaigns::sessions::Session::new(session.clone().0);
    let result = diesel::update(schema::sessions::dsl::sessions.find(&id))
        .set(&new_session)
        .execute(connection);
    match result {
        Ok(_) => Ok(Accepted(session)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}
