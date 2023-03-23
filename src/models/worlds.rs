use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::worlds;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = worlds)]
pub struct World {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl super::Model for World {
    type Entity = gamemstr_common::world::World;

    fn new(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }
}
