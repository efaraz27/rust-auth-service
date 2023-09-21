// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        name -> Varchar,
        secret -> Varchar,
        redirect_uri -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    users,
);
