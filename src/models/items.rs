use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::items;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: serde_json::Value,
    pub rarity: serde_json::Value,
    pub attunement: Option<serde_json::Value>,
    pub weapon_type: Option<serde_json::Value>,
    pub armor_type: Option<serde_json::Value>,
    pub conditions: Option<serde_json::Value>,
    pub attached_spell: Option<serde_json::Value>,
    pub has_charges: Option<serde_json::Value>,
    pub inventory: Option<serde_json::Value>,
    pub others: Option<serde_json::Value>,
    pub actions: Option<serde_json::Value>,
}

impl super::Model for Item {
    type Entity = gamemstr_common::item::Item;

    fn new(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            item_type: serde_json::to_value(entity.item_type).unwrap(),
            rarity: serde_json::to_value(entity.rarity).unwrap(),
            attunement: Some(serde_json::to_value(entity.attunement).unwrap()),
            weapon_type: Some(serde_json::to_value(entity.weapon_type).unwrap()),
            armor_type: Some(serde_json::to_value(entity.armor_type).unwrap()),
            conditions: Some(serde_json::to_value(entity.conditions).unwrap()),
            attached_spell: Some(serde_json::to_value(entity.attached_spell).unwrap()),
            has_charges: Some(serde_json::to_value(entity.has_charges).unwrap()),
            inventory: Some(serde_json::to_value(entity.inventory).unwrap()),
            others: Some(serde_json::to_value(entity.others).unwrap()),
            actions: Some(serde_json::to_value(entity.actions).unwrap()),
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            item_type: serde_json::from_value(self.item_type.clone()).unwrap(),
            rarity: serde_json::from_value(self.rarity.clone()).unwrap(),
            attunement: serde_json::from_value(self.attunement.clone().unwrap()).unwrap(),
            weapon_type: serde_json::from_value(self.weapon_type.clone().unwrap()).unwrap(),
            armor_type: serde_json::from_value(self.armor_type.clone().unwrap()).unwrap(),
            conditions: serde_json::from_value(self.conditions.clone().unwrap()).unwrap(),
            attached_spell: serde_json::from_value(self.attached_spell.clone().unwrap()).unwrap(),
            has_charges: serde_json::from_value(self.has_charges.clone().unwrap()).unwrap(),
            inventory: serde_json::from_value(self.inventory.clone().unwrap()).unwrap(),
            others: serde_json::from_value(self.others.clone().unwrap()).unwrap(),
            actions: serde_json::from_value(self.actions.clone().unwrap()).unwrap(),
        }
    }
}
