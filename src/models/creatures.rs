use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::creatures;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = creatures)]
pub struct Creature {
    pub id: String,
    pub name: String,
    pub creature_type: serde_json::Value,
    pub alignment: serde_json::Value,
    pub armor_class: i32,
    pub health_points: serde_json::Value,
    pub speed: serde_json::Value,
    pub stats: serde_json::Value,
    pub saving_throws: Option<serde_json::Value>,
    pub damage_resistances: Option<serde_json::Value>,
    pub damage_immunities: Option<serde_json::Value>,
    pub damage_vulnerabilities: Option<serde_json::Value>,
    pub condition_immunities: Option<serde_json::Value>,
    pub skills: Option<serde_json::Value>,
    pub senses: Option<serde_json::Value>,
    pub languages: Option<serde_json::Value>,
    pub challenge_rating: serde_json::Value,
    pub racial_traits: Option<serde_json::Value>,
    pub description: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub lair: Option<serde_json::Value>,
    pub others: Option<serde_json::Value>,
}

impl super::Model for Creature {
    type Entity = gamemstr_common::creature::Creature;

    fn new(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            creature_type: serde_json::to_value(entity.creature_type).unwrap(),
            alignment: serde_json::to_value(entity.alignment).unwrap(),
            armor_class: entity.armor_class,
            health_points: serde_json::to_value(entity.health_points).unwrap(),
            speed: serde_json::to_value(entity.speed).unwrap(),
            stats: serde_json::to_value(entity.stats).unwrap(),
            saving_throws: Some(serde_json::to_value(entity.saving_throws).unwrap()),
            damage_resistances: Some(serde_json::to_value(entity.damage_resistances).unwrap()),
            damage_immunities: Some(serde_json::to_value(entity.damage_immunities).unwrap()),
            damage_vulnerabilities: Some(
                serde_json::to_value(entity.damage_vulnerabilities).unwrap(),
            ),
            condition_immunities: Some(serde_json::to_value(entity.condition_immunities).unwrap()),
            skills: Some(serde_json::to_value(entity.skills).unwrap()),
            senses: Some(serde_json::to_value(entity.senses).unwrap()),
            languages: Some(serde_json::to_value(entity.languages).unwrap()),
            challenge_rating: serde_json::to_value(entity.challenge_rating).unwrap(),
            racial_traits: Some(serde_json::to_value(entity.racial_traits).unwrap()),
            description: entity.description,
            actions: Some(serde_json::to_value(entity.actions).unwrap()),
            lair: Some(serde_json::to_value(entity.lair).unwrap()),
            others: Some(serde_json::to_value(entity.others).unwrap()),
        }
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            creature_type: serde_json::from_value(self.creature_type.clone()).unwrap(),
            alignment: serde_json::from_value(self.alignment.clone()).unwrap(),
            armor_class: self.armor_class,
            health_points: serde_json::from_value(self.health_points.clone()).unwrap(),
            speed: serde_json::from_value(self.speed.clone()).unwrap(),
            stats: serde_json::from_value(self.stats.clone()).unwrap(),
            saving_throws: serde_json::from_value(self.saving_throws.clone().unwrap()).unwrap(),
            damage_resistances: serde_json::from_value(self.damage_resistances.clone().unwrap())
                .unwrap(),
            damage_immunities: serde_json::from_value(self.damage_immunities.clone().unwrap())
                .unwrap(),
            damage_vulnerabilities: serde_json::from_value(
                self.damage_vulnerabilities.clone().unwrap(),
            )
            .unwrap(),
            condition_immunities: serde_json::from_value(
                self.condition_immunities.clone().unwrap(),
            )
            .unwrap(),
            skills: serde_json::from_value(self.skills.clone().unwrap()).unwrap(),
            senses: serde_json::from_value(self.senses.clone().unwrap()).unwrap(),
            languages: serde_json::from_value(self.languages.clone().unwrap()).unwrap(),
            challenge_rating: serde_json::from_value(self.challenge_rating.clone()).unwrap(),
            racial_traits: serde_json::from_value(self.racial_traits.clone().unwrap()).unwrap(),
            description: self.description.clone(),
            actions: serde_json::from_value(self.actions.clone().unwrap()).unwrap(),
            lair: serde_json::from_value(self.lair.clone().unwrap()).unwrap(),
            others: serde_json::from_value(self.others.clone().unwrap()).unwrap(),
        }
    }
}
