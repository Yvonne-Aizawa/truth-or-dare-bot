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
    error_log (id) {
        id -> Integer,
        error_message -> Text,
        error_code -> Text,
        stack_trace -> Nullable<Text>,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    moderation (id) {
        id -> Integer,
        moderation_type -> Text,
        kind -> Text,
        item_id -> Integer,
        moderator_id -> Text,
        reason -> Nullable<Text>,
        timestamp -> Timestamp,
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

diesel::table! {
    user_log (id) {
        id -> Integer,
        user_id -> Text,
        action -> Text,
        details -> Nullable<Text>,
        timestamp -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dares,
    error_log,
    moderation,
    truths,
    user_log,
);
