// @generated automatically by Diesel CLI.

diesel::table! {
    creatures (id) {
        id -> Varchar,
        name -> Varchar,
        creature_type -> Jsonb,
        alignment -> Jsonb,
        armor_class -> Int4,
        health_points -> Jsonb,
        speed -> Jsonb,
        stats -> Jsonb,
        saving_throws -> Nullable<Jsonb>,
        damage_resistances -> Nullable<Jsonb>,
        damage_immunities -> Nullable<Jsonb>,
        damage_vulnerabilities -> Nullable<Jsonb>,
        condition_immunities -> Nullable<Jsonb>,
        skills -> Nullable<Jsonb>,
        senses -> Nullable<Jsonb>,
        languages -> Nullable<Jsonb>,
        challenge_rating -> Jsonb,
        racial_traits -> Nullable<Jsonb>,
        description -> Nullable<Varchar>,
        actions -> Nullable<Jsonb>,
        lair -> Nullable<Jsonb>,
        others -> Nullable<Jsonb>,
    }
}

diesel::table! {
    items (id) {
        id -> Varchar,
        name -> Varchar,
        item_type -> Jsonb,
        rarity -> Jsonb,
        attunement -> Nullable<Jsonb>,
        weapon_type -> Nullable<Jsonb>,
        armor_type -> Nullable<Jsonb>,
        conditions -> Nullable<Jsonb>,
        attached_spell -> Nullable<Jsonb>,
        has_charges -> Nullable<Jsonb>,
        inventory -> Nullable<Jsonb>,
        others -> Nullable<Jsonb>,
        actions -> Nullable<Jsonb>,
    }
}

diesel::table! {
    locations (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        world_id -> Varchar,
        map_coordinates -> Jsonb,
        npcs -> Jsonb,
    }
}

diesel::table! {
    spells (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        level -> Jsonb,
        casting_time -> Jsonb,
        duration -> Jsonb,
        damage -> Nullable<Jsonb>,
        range -> Jsonb,
        area -> Nullable<Jsonb>,
        damage_type -> Nullable<Jsonb>,
        components -> Jsonb,
        attack_bonus -> Nullable<Int4>,
        save -> Nullable<Jsonb>,
    }
}

diesel::table! {
    worlds (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(creatures, items, locations, spells, worlds,);
