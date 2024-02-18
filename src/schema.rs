// @generated automatically by Diesel CLI.

diesel::table! {
    file_results (id) {
        id -> Text,
        result -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 128]
        username -> Varchar,
        secret -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    file_results,
    users,
);
