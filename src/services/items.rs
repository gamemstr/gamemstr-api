use super::Result;
use crate::{
    models::{self, Model},
    schema,
};
use diesel::prelude::*;
use gamemstr_common::item::Item;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use rocket_dyn_templates::{context, Template};

#[get("/items")]
pub fn list_items() -> Template {
    let connection = &mut super::establish_connection_pg();
    let items = schema::items::dsl::items
        .load::<models::items::Item>(connection)
        .expect("Error loading items")
        .iter()
        .map(|item| item.to_entity())
        .collect::<Vec<_>>();
    Template::render(
        "items",
        context! {
            items: &items,
            count: items.len()
        },
    )
}

#[get("/items/<id>")]
pub fn get_item(id: String) -> Result<Json<Item>> {
    let connection = &mut super::establish_connection_pg();
    let item = schema::items::dsl::items
        .find(id)
        .first::<models::items::Item>(connection)
        .expect("Error loading item")
        .to_entity();
    Ok(Json(item))
}

#[delete("/items/<id>")]
pub fn delete_item(id: String) -> Result<Json<Item>> {
    let connection = &mut super::establish_connection_pg();
    let item = schema::items::dsl::items
        .find(&id)
        .first::<models::items::Item>(connection)
        .expect("Error loading item")
        .to_entity();
    diesel::delete(schema::items::dsl::items.find(&id))
        .execute(connection)
        .expect("Error deleting item");
    Ok(Json(item))
}

#[post("/items/add", format = "json", data = "<item>")]
pub fn create_item(item: Json<Item>) -> Result<Created<Json<Item>>> {
    let connection = &mut super::establish_connection_pg();

    let new_item = models::items::Item::new(item.clone().0);

    diesel::insert_into(schema::items::dsl::items)
        .values(&new_item)
        .execute(connection)
        .expect("Error saving new item");
    Ok(Created::new("/").body(item))
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use gamemstr_common::{action, item};
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_list_items() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/items").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_html())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Items"));
    }

    #[test]
    fn test_get_item() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/items/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Test Item"));
    }

    #[test]
    fn test_delete_item() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let item = gamemstr_common::item::Item {
            id: "258759802792856926525".to_string(),
            name: "Test Item".to_string(),
            item_type: item::ItemType::Weapon,
            rarity: item::ItemRarity::Common,
            attunement: Some(item::Attuneable {
                alignments: Some(vec![gamemstr_common::Alignment::ChaoticGood]),
            }),
            weapon_type: Some(item::WeaponType::Sword),
            armor_type: None,
            conditions: Some(vec![gamemstr_common::ConditionType::Blinded]),
            attached_spell: None,
            has_charges: Some(item::Charge {
                num: 5,
                time: item::TimeDivision::Day,
            }),
            inventory: None,
            others: None,
            actions: Some(vec![action::Action::new(action::ActionType::Attack(
                action::attack::Attack::MeleeWeaponAttack(action::attack::Melee {
                    name: "Test Attack".to_string(),
                    modifier: 3,
                    reach: Some(10),
                    target_type: action::attack::TargetType::OneTarget,
                    damage: gamemstr_common::DieStat {
                        die_count: 3,
                        die_type: gamemstr_common::Die::D6,
                        extra: 4,
                    },
                    damage_type: gamemstr_common::DamageType::Slashing,
                    description: "Test Description".to_string(),
                }),
            ))]),
        };
        client
            .post("/items/new")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&item).unwrap())
            .dispatch();
        let response = client.delete("/items/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        assert!(body.contains("Test Item"));
    }

    #[test]
    fn test_create_item() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        client.delete("/items/258759802792856926525").dispatch();
        let item = gamemstr_common::item::Item {
            id: "258759802792856926525".to_string(),
            name: "Test Item".to_string(),
            item_type: item::ItemType::Weapon,
            rarity: item::ItemRarity::Common,
            attunement: Some(item::Attuneable {
                alignments: Some(vec![gamemstr_common::Alignment::ChaoticGood]),
            }),
            weapon_type: Some(item::WeaponType::Sword),
            armor_type: None,
            conditions: Some(vec![gamemstr_common::ConditionType::Blinded]),
            attached_spell: None,
            has_charges: Some(item::Charge {
                num: 5,
                time: item::TimeDivision::Day,
            }),
            inventory: None,
            others: None,
            actions: Some(vec![action::Action::new(action::ActionType::Attack(
                action::attack::Attack::MeleeWeaponAttack(action::attack::Melee {
                    name: "Test Attack".to_string(),
                    modifier: 3,
                    reach: Some(10),
                    target_type: action::attack::TargetType::OneTarget,
                    damage: gamemstr_common::DieStat {
                        die_count: 3,
                        die_type: gamemstr_common::Die::D6,
                        extra: 4,
                    },
                    damage_type: gamemstr_common::DamageType::Slashing,
                    description: "Test Description".to_string(),
                }),
            ))]),
        };
        let response = client
            .post("/items/add")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&item).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        assert!(response
            .content_type()
            .map(|ct| ct.is_json())
            .unwrap_or(false));
        let body = response.into_string().unwrap();
        println!("{}", body);
        assert_eq!(body, serde_json::to_string(&item).unwrap());
    }
}
