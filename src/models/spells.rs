use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::spells;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = spells)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: serde_json::Value,
    pub casting_time: serde_json::Value,
    pub duration: serde_json::Value,
    pub damage: Option<serde_json::Value>,
    pub range: serde_json::Value,
    pub area: Option<serde_json::Value>,
    pub damage_type: Option<serde_json::Value>,
    pub components: serde_json::Value,
    pub attack_bonus: Option<i32>,
    pub save: Option<serde_json::Value>,
}

impl super::Model for Spell {
    type Entity = gamemstr_common::spell::Spell;

    fn new(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            level: serde_json::to_value(entity.level).unwrap(),
            casting_time: serde_json::to_value(entity.casting_time).unwrap(),
            duration: serde_json::to_value(entity.duration).unwrap(),
            damage: entity.damage.map(|d| serde_json::to_value(d).unwrap()),
            range: serde_json::to_value(entity.range).unwrap(),
            area: entity.area.map(|a| serde_json::to_value(a).unwrap()),
            damage_type: entity
                .damage_type
                .map(|dt| serde_json::to_value(dt).unwrap()),
            components: serde_json::to_value(entity.components).unwrap(),
            attack_bonus: entity.attack_bonus,
            save: entity.save.map(|s| serde_json::to_value(s).unwrap()),
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            level: serde_json::from_value(self.level.clone()).unwrap(),
            casting_time: serde_json::from_value(self.casting_time.clone()).unwrap(),
            duration: serde_json::from_value(self.duration.clone()).unwrap(),
            damage: self
                .damage
                .clone()
                .map(|d| serde_json::from_value(d).unwrap()),
            range: serde_json::from_value(self.range.clone()).unwrap(),
            area: self
                .area
                .clone()
                .map(|a| serde_json::from_value(a).unwrap()),
            damage_type: self
                .damage_type
                .clone()
                .map(|dt| serde_json::from_value(dt).unwrap()),
            components: serde_json::from_value(self.components.clone()).unwrap(),
            attack_bonus: self.attack_bonus,
            save: self
                .save
                .clone()
                .map(|s| serde_json::from_value(s).unwrap()),
        }
    }
}
