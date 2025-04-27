
diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        hashed_password -> Varchar,
        created_at -> Timestamp,
    }
}
