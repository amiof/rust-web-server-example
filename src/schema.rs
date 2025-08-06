// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        created_at -> Timestamp,
    }
}
