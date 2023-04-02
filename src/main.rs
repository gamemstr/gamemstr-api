pub mod models;
pub mod schema;
mod services;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![services::creatures::list_creatures])
        .mount("/", routes![services::creatures::get_creature])
        .mount("/", routes![services::creatures::delete_creature])
        .mount("/", routes![services::creatures::create_creature])
        .mount("/", routes![services::creatures::update_creature])
        .mount("/", routes![services::items::list_items])
        .mount("/", routes![services::items::get_item])
        .mount("/", routes![services::items::delete_item])
        .mount("/", routes![services::items::create_item])
        .mount("/", routes![services::items::update_item])
        .mount("/", routes![services::spells::list_spells])
        .mount("/", routes![services::spells::get_spell])
        .mount("/", routes![services::spells::delete_spell])
        .mount("/", routes![services::spells::create_spell])
        .mount("/", routes![services::spells::update_spell])
        .mount("/", routes![services::worlds::create_world])
        .mount("/", routes![services::worlds::get_world])
        .mount("/", routes![services::worlds::delete_world])
        .mount("/", routes![services::worlds::list_worlds])
        .mount("/", routes![services::worlds::locations::list_locations])
        .mount("/", routes![services::worlds::locations::get_location])
        .mount("/", routes![services::worlds::locations::delete_location])
        .mount("/", routes![services::worlds::locations::create_location])
        .mount("/", routes![services::worlds::locations::update_location])
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests {
    use gamemstr_common::{action, item, spell};
    use rocket::{
        http::{ContentType, Status},
        local::blocking::Client,
    };

    #[test]
    fn test_creatures_api() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/creatures").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Creatures"));
        let creature = gamemstr_common::creature::Creature {
            id: "258759802792856926525".to_string(),
            name: "Test Creature".to_string(),
            creature_type: gamemstr_common::creature::CreatureType::NPC,
            alignment: gamemstr_common::Alignment::ChaoticGood,
            armor_class: 10,
            health_points: gamemstr_common::creature::Health {
                health: gamemstr_common::DieStat {
                    die_count: 3,
                    die_type: gamemstr_common::Die::D6,
                    extra: 4,
                },
            },
            speed: gamemstr_common::creature::MovementSpeed::Walk(30),
            stats: vec![
                gamemstr_common::creature::Stat {
                    stat_type: gamemstr_common::creature::StatType::Strength,
                    value: 10,
                    modifier: 0,
                },
                gamemstr_common::creature::Stat {
                    stat_type: gamemstr_common::creature::StatType::Dexterity,
                    value: 10,
                    modifier: 0,
                },
                gamemstr_common::creature::Stat {
                    stat_type: gamemstr_common::creature::StatType::Constitution,
                    value: 10,
                    modifier: 0,
                },
                gamemstr_common::creature::Stat {
                    stat_type: gamemstr_common::creature::StatType::Intelligence,
                    value: 10,
                    modifier: 0,
                },
                gamemstr_common::creature::Stat {
                    stat_type: gamemstr_common::creature::StatType::Wisdom,
                    value: 10,
                    modifier: 0,
                },
                gamemstr_common::creature::Stat {
                    stat_type: gamemstr_common::creature::StatType::Charisma,
                    value: 10,
                    modifier: 0,
                },
            ],
            saving_throws: None,
            damage_vulnerabilities: None,
            damage_resistances: None,
            damage_immunities: None,
            condition_immunities: None,
            skills: None,
            senses: None,
            languages: None,
            challenge_rating: "1".into(),
            racial_traits: None,
            description: None,
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
            lair: None,
            others: None,
        };
        let response = client
            .post("/creatures/add")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&creature).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        let response = client.get("/creatures/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Test Creature"));
        let new_creature = gamemstr_common::creature::Creature {
            name: "Updated Creature".to_string(),
            ..creature
        };
        let response = client
            .post("/creatures/258759802792856926525")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&new_creature).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);
        assert_eq!(
            serde_json::from_str::<gamemstr_common::creature::Creature>(
                &response.into_string().unwrap()
            )
            .unwrap()
            .name,
            "Updated Creature"
        );
        let response = client.delete("/creatures/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_items_api() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/items").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Items"));
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
            .header(ContentType::JSON)
            .body(serde_json::to_string(&item).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        let response = client.get("/items/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some(serde_json::to_string(&item).unwrap())
        );
        let new_item = gamemstr_common::item::Item {
            name: "Updated Item".to_string(),
            ..item
        };
        let response = client
            .post("/items/258759802792856926525")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&new_item).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);
        assert_eq!(
            serde_json::from_str::<gamemstr_common::item::Item>(&response.into_string().unwrap())
                .unwrap()
                .name,
            "Updated Item"
        );
        let response = client.delete("/items/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_spells_api() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/spells").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Spells"));
        let spell = gamemstr_common::spell::Spell {
            id: "258759802792856926525".to_string(),
            name: "Test Spell".to_string(),
            description: "Test Description".to_string(),
            level: spell::SpellLevel::Cantrip,
            casting_time: spell::CastingTime::Action,
            duration: spell::Duration::Concentration,
            damage: Some(gamemstr_common::DieStat {
                die_count: 5,
                die_type: gamemstr_common::Die::D6,
                extra: 2,
            }),
            range: spell::SpellRange::Touch,
            area: Some(spell::Area::Cone(10)),
            damage_type: Some(gamemstr_common::DamageType::Slashing),
            components: spell::Components::VS,
            attack_bonus: Some(4),
            save: Some(spell::Save::Dexterity(Some(13))),
        };
        let response = client
            .post("/spells/add")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&spell).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        let response = client.get("/spells/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some(serde_json::to_string(&spell).unwrap())
        );
        let new_spell = gamemstr_common::spell::Spell {
            name: "Updated Spell".to_string(),
            ..spell
        };
        let response = client
            .post("/spells/258759802792856926525")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&new_spell).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);
        assert_eq!(
            serde_json::from_str::<gamemstr_common::spell::Spell>(&response.into_string().unwrap())
                .unwrap()
                .name,
            "Updated Spell"
        );
        let response = client.delete("/spells/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_worlds_api() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/worlds").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Worlds"));
        let world = gamemstr_common::world::World {
            id: "258759802792856926525".to_string(),
            name: "Test World".to_string(),
            description: "Test Description".to_string(),
        };
        let response = client
            .post("/worlds/add")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&world).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        let response = client.get("/worlds/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some(serde_json::to_string(&world).unwrap())
        );
        let response = client.delete("/worlds/258759802792856926525").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    pub fn test_locations_api() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");
        let world = gamemstr_common::world::World {
            id: "2587598027928569265".to_string(),
            name: "Test World".to_string(),
            description: "Test Description".to_string(),
        };
        client
            .post("/worlds/add")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&world).unwrap())
            .dispatch();
        let response = client
            .get("/worlds/2587598027928569265/locations")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Locations"));
        let location = gamemstr_common::world::location::Location {
            id: "258759802792856926525".to_string(),
            name: "Test Location".to_string(),
            description: "Test Description".to_string(),
            world_id: "2587598027928569265".to_string(),
            map_coordinates: gamemstr_common::world::map::MapCoordinates {
                x: 0,
                y: 0,
                map_id: "25875980279286525".to_string(),
            },
            npcs: vec![],
        };
        let response = client
            .post("/worlds/2587598027928569265/locations/add")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&location).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        let response = client
            .get("/worlds/2587598027928569265/locations/258759802792856926525")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some(serde_json::to_string(&location).unwrap())
        );
        let new_location = gamemstr_common::world::location::Location {
            name: "Updated Location".to_string(),
            ..location
        };
        let response = client
            .post("/worlds/2587598027928569265/locations/258759802792856926525")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&new_location).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);
        assert_eq!(
            serde_json::from_str::<gamemstr_common::world::location::Location>(
                &response.into_string().unwrap()
            )
            .unwrap()
            .name,
            "Updated Location"
        );
        let response = client
            .delete("/worlds/2587598027928569265/locations/258759802792856926525")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        client.delete("/worlds/2587598027928569265").dispatch();
    }
}
