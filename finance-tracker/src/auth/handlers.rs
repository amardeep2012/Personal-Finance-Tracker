use rocket::{post, serde::json::Json};
use rocket::response::status::BadRequest;
use crate::auth::{AuthResponse, LoginResponse};
// Removed redundant import of UserInput

#[post("/signup", format = "json", data = "<user_input>")]
pub async fn signup_route(user_input: Json<UserInput>) -> Result<Json<AuthResponse>, BadRequest<String>> {
    match sign_up(user_input.into_inner()).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(BadRequest(err)),
    }
}

#[post("/login", format = "json", data = "<user_input>")]
pub async fn login_route(user_input: Json<UserLoginInput>) -> Result<Json<LoginResponse>, BadRequest<String>> {
    match login(user_input.into_inner()).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(BadRequest(err)),
    }
}

// pub fn routes() -> Vec<rocket::Route> {
//     routes![signup_route, login_route]
// }


use diesel::prelude::*;
use crate::auth::schema::users; // Import the users schema
use crate::auth::password::{hash_password, verify_password};
use crate::auth::token::create_jwt;
use crate::db;
use crate::auth::models::{UserInput, UserLoginInput};

pub async fn sign_up(user_input: UserInput) -> Result<AuthResponse, String> {
    let conn = &mut db::establish_connection();
    let hashed_password = hash_password(&user_input.password).map_err(|e| e.to_string())?;

    // Insert user into the database (Diesel)
    diesel::insert_into(users::table)
        .values((
            users::email.eq(&user_input.email),
            users::hashed_password.eq(&hashed_password),
            users::name.eq(&user_input.name), // Include the name field
        ))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    // Generate JWT
    let token = create_jwt(&user_input.email);
    Ok(AuthResponse { token })
}

pub async fn login(user_input: UserLoginInput) -> Result<LoginResponse, String> {
    let conn = &mut db::establish_connection();

    let user = users::table
        .filter(users::email.eq(&user_input.email))
        .select(crate::auth::models::User::as_select())
        .first(conn)
        .map_err(|_| "User not found")?;

    // Verify password
    if !verify_password(&user.hashed_password, &user_input.password).unwrap_or(false) {
        return Err("Invalid credentials".to_string());
    }

    // Generate JWT
    let _token = create_jwt(&user_input.email);
    Ok(LoginResponse{ message: "Login successful".to_string(),})
}
