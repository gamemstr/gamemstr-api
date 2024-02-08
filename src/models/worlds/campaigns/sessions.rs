use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::sessions;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub campaign_id: String,
    pub description: String,
    pub notes: serde_json::Value,
    pub plan: serde_json::Value,
    pub recap: serde_json::Value,
}

impl super::super::super::Model for Session {
    type Entity = gamemstr_common::world::campaign::session::Session;

    fn new(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            campaign_id: entity.campaign_id,
            description: entity.description,
            notes: serde_json::to_value(entity.notes).unwrap(),
            plan: serde_json::to_value(entity.plan).unwrap(),
            recap: serde_json::to_value(entity.recap).unwrap(),
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            campaign_id: self.campaign_id.clone(),
            description: self.description.clone(),
            notes: serde_json::from_value(self.notes.clone()).unwrap(),
            plan: serde_json::from_value(self.plan.clone()).unwrap(),
            recap: serde_json::from_value(self.recap.clone()).unwrap(),
        }
    }
}