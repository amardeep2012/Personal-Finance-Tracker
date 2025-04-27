#[macro_use] extern crate rocket;

use rocket::launch;
// use rocket::serde::json::Json;
use rocket::fairing::AdHoc;
// use rocket::tokio::sync::Mutex;
// use std::sync::Arc;

mod db;
mod auth;

use auth::{signup_route, login_route};

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok(); // Load environment variables from .env
    rocket::build()
        .mount("/auth", routes![signup_route, login_route]) // Mount auth routes
        .attach(AdHoc::on_ignite("Database Connection", |rocket| async {
            // let database_url = std::env::var("DATABASE_URL")
                // .expect("DATABASE_URL must be set in .env");

            // Test DB connection
            db::establish_connection();
            println!("✅ Successfully connected to database!");

            rocket
        }))
}

