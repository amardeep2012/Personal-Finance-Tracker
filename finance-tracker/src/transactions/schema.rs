diesel::table! {
    transactions (id) {
        id -> Int4,
        amount -> Numeric,
        description -> Text,
        category_id -> Int4,
        created_at -> Timestamp,
    }
}