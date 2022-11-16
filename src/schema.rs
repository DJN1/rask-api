// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Text,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
