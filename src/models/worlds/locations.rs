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