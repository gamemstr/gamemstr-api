use crate::{
    models::{self, Model},
    schema,
    services::Result,
};
use diesel::prelude::*;
use gamemstr_common::world::campaign::Campaign;
use rocket::{
    delete, get, post,
    response::status::{Accepted, Created, NotFound},
    serde::json::Json,
    Either,
};
use rocket_dyn_templates::{context, Template};

pub mod sessions;

#[get("/worlds/<id>/campaigns")]
pub fn list_campaigns(id: String) -> Template {
    let connection = &mut super::super::establish_connection_pg();
    let campaigns = schema::campaigns::dsl::campaigns
        .filter(schema::campaigns::dsl::world_id.eq(id))
        .load::<models::worlds::campaigns::Campaign>(connection)
        .expect("Error loading campaigns")
        .iter()
        .map(|campaign| campaign.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "campaigns",
        context! {
            campaigns: &campaigns,
            count: campaigns.len()
        },
    )
}

#[get("/worlds/<_world_id>/campaigns/<id>")]
pub fn get_campaign(_world_id: String, id: String) -> Result<Json<Campaign>> {
    let connection = &mut super::super::establish_connection_pg();
    let campaign = schema::campaigns::dsl::campaigns
        .find(id)
        .first::<models::worlds::campaigns::Campaign>(connection)
        .expect("Error loading campaign")
        .to_entity();
    Ok(Json(campaign))
}

#[delete("/worlds/<_world_id>/campaigns/<id>")]
pub fn delete_campaign(_world_id: String, id: String) -> Result<Json<Campaign>> {
    let connection = &mut super::super::establish_connection_pg();
    let campaign = schema::campaigns::dsl::campaigns
        .find(&id)
        .first::<models::worlds::campaigns::Campaign>(connection)
        .expect("Error loading campaign")
        .to_entity();
    diesel::delete(schema::campaigns::dsl::campaigns.find(&id))
        .execute(connection)
        .expect("Error deleting campaign");
    Ok(Json(campaign))
}

#[post(
    "/worlds/<_world_id>/campaigns/add",
    rank = 1,
    format = "json",
    data = "<campaign>"
)]
pub fn create_campaign(
    _world_id: String,
    campaign: Json<Campaign>,
) -> Result<Created<Json<Campaign>>> {
    let connection = &mut super::super::establish_connection_pg();
    let new_campaign = models::worlds::campaigns::Campaign::new(campaign.clone().0);
    diesel::insert_into(schema::campaigns::dsl::campaigns)
        .values(&new_campaign)
        .execute(connection)
        .expect("Error creating campaign");
    Ok(Created::new("/").body(campaign))
}

#[post(
    "/worlds/<_world_id>/campaigns/<id>",
    rank = 2,
    format = "json",
    data = "<campaign>"
)]
pub fn update_campaign(
    _world_id: String,
    id: String,
    campaign: Json<Campaign>,
) -> Either<Result<Accepted<Json<Campaign>>>, Result<NotFound<String>>> {
    let connection = &mut super::super::establish_connection_pg();
    let result = diesel::update(schema::campaigns::dsl::campaigns.find(&id))
        .set(&models::worlds::campaigns::Campaign::new(
            campaign.clone().0,
        ))
        .execute(connection);
    match result {
        Ok(_) => Either::Left(Ok(Accepted(campaign))),
        Err(_) => Either::Right(Ok(NotFound(format!("Campaign not found with id {}", id)))),
    }
}
