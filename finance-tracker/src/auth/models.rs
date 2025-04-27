use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::auth::schema::users;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::auth::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct UserLoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}