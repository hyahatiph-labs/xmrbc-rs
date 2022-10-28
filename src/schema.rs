// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
