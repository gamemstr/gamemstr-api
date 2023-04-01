use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::locations;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = locations)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: String,
    pub world_id: String,
    pub map_coordinates: serde_json::Value,
    pub npcs: serde_json::Value,
}

impl super::super::Model for Location {
    type Entity = gamemstr_common::world::location::Location;

    fn new(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            world_id: entity.world_id,
            map_coordinates: serde_json::to_value(entity.map_coordinates).unwrap(),
            npcs: serde_json::to_value(entity.npcs).unwrap(),
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            world_id: self.world_id.clone(),
            map_coordinates: serde_json::from_value(self.map_coordinates.clone()).unwrap(),
            npcs: serde_json::from_value(self.npcs.clone()).unwrap(),
        }
    }
}
