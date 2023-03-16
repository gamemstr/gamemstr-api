use crate::schema::worlds;
use diesel;
use diesel::prelude::*;
use gamemstr_common::world::World;
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = worlds)]
pub struct WorldModel {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Model for WorldModel {
    type Entity = World;

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
