// @generated automatically by Diesel CLI.

diesel::table! {
    project (id) {
        id -> Uuid,
        title -> Varchar,
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        active -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    project,
    users,
);
