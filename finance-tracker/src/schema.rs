// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        amount -> Numeric,
        description -> Text,
        category_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        hashed_password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(transactions -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    transactions,
    users,
);
