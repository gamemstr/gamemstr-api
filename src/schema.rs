// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Varchar,
        name -> Varchar,
        attributes -> Jsonb,
    }
}

diesel::table! {
    worlds (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(items, worlds,);
