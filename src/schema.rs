// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        deadline -> Text,
        priority -> Integer,
        done -> Bool,
    }
}
