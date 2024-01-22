// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 128]
        username -> Varchar,
        secret -> Text,
    }
}
