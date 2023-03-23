// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(
    items,
    spells,
    worlds,
);
