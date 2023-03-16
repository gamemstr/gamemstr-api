use crate::schema::items;
use diesel;
use diesel::prelude::*;
use gamemstr_common::item::Item;
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = items)]
pub struct ItemModel {
    pub id: String,
    pub name: String,
    pub attributes: serde_json::Value,
}

impl Model for ItemModel {
    type Entity = Item;

    fn new(entity: Self::Entity) -> Self {
        ItemModel {
            id: entity.id,
            name: entity.name,
            attributes: serde_json::to_value(entity.attributes).unwrap(),
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            attributes: serde_json::from_value(self.attributes.clone()).unwrap(),
        }
    }
}
