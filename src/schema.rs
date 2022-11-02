// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        subaddress -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
