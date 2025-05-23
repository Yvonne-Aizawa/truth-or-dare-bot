// @generated automatically by Diesel CLI.

diesel::table! {
    dares (id) {
        id -> Integer,
        content -> Text,
        author -> Text,
        rating -> Text,
        status -> Text,
        submit_date -> Timestamp,
    }
}

diesel::table! {
    truths (id) {
        id -> Integer,
        content -> Text,
        author -> Text,
        rating -> Text,
        status -> Text,
        submit_date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(dares, truths,);
