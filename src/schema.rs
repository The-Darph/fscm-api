// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Integer,
        subtype -> Integer,
        description -> Text,
        body -> Text,
        scale -> Integer,
        source -> Text,
        transpired -> Nullable<Text>,
        published_date -> Nullable<Text>,
    }
}

diesel::table! {
    events_subtypes (id) {
        id -> Nullable<Integer>,
        event_id -> Integer,
        subtype_id -> Integer,
    }
}

diesel::table! {
    subtypes (id) {
        id -> Nullable<Integer>,
        description -> Text,
    }
}

diesel::table! {
    types (id) {
        id -> Nullable<Integer>,
        description -> Text,
    }
}

diesel::joinable!(events -> subtypes (subtype));
diesel::joinable!(events -> types (type_));
diesel::joinable!(events_subtypes -> events (event_id));
diesel::joinable!(events_subtypes -> subtypes (subtype_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    events_subtypes,
    subtypes,
    types,
);
