use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::campaigns;

pub mod sessions;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = campaigns)]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub description: String,
    pub world_id: String,
    pub players: serde_json::Value,
}

impl crate::models::Model for Campaign {
    type Entity = gamemstr_common::world::campaign::Campaign;

    fn new(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            world_id: entity.world_id,
            players: serde_json::to_value(entity.players).unwrap(),
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            world_id: self.world_id.clone(),
            players: serde_json::from_value(self.players.clone()).unwrap(),
        }
    }
}
   